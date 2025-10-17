# 📹 Webcam Publishing Guide

## Architecture
```
[Webcam] → FFmpeg (host) → rtsp://localhost:8554/demo → MediaMTX → Frigate → MQTT → Rust CLI
```

## Step 1: List Your Webcam

### macOS
```bash
# List available video devices
ffmpeg -f avfoundation -list_devices true -i ""

# Example output:
# [AVFoundation indev @ 0x...] [0] FaceTime HD Camera
# [AVFoundation indev @ 0x...] [1] OBS Virtual Camera
```

### Linux
```bash
# List video devices
v4l2-ctl --list-devices

# Or use ls
ls -l /dev/video*
```

### Windows
```bash
# List DirectShow devices
ffmpeg -list_devices true -f dshow -i dummy
```

## Step 2: Publish to MediaMTX

### macOS (Hardware Accelerated)
```bash
# Replace "0:0" with your device index from step 1
ffmpeg -f avfoundation -framerate 30 -pixel_format uyvy422 -i "0:0" \
  -vf scale=-2:720 -r 15 \
  -c:v h264_videotoolbox -profile:v high -g 30 -tune zerolatency \
  -f rtsp -rtsp_transport tcp rtsp://localhost:8554/demo
```

**Flags explained:**
- `-f avfoundation`: macOS video input
- `-i "0:0"`: Device index (video:audio)
- `-vf scale=-2:720`: Scale to 720p, preserve aspect ratio
- `-r 15`: Output 15 FPS
- `-c:v h264_videotoolbox`: Use hardware encoder (M1/M2/Intel)
- `-g 30`: GOP size (keyframe every 30 frames)
- `-tune zerolatency`: Optimize for live streaming
- `-rtsp_transport tcp`: Reliable transport (not UDP)

### Linux (x264 Software Encoder)
```bash
# Replace /dev/video0 with your device
ffmpeg -f v4l2 -framerate 30 -video_size 1280x720 -i /dev/video0 \
  -vf scale=-2:720 -r 15 \
  -c:v libx264 -preset veryfast -tune zerolatency -g 30 \
  -f rtsp -rtsp_transport tcp rtsp://localhost:8554/demo
```

### Windows (DirectShow)
```bash
# Replace "Integrated Camera" with your device name
ffmpeg -f dshow -framerate 30 -video_size 1280x720 -i video="Integrated Camera" \
  -vf scale=-2:720 -r 15 \
  -c:v libx264 -preset veryfast -tune zerolatency -g 30 \
  -f rtsp -rtsp_transport tcp rtsp://localhost:8554/demo
```

## Step 3: Verify Stream

### Test with ffplay
```bash
ffplay -rtsp_transport tcp rtsp://localhost:8554/demo
```

### Test with VLC
```
Media → Open Network Stream → rtsp://localhost:8554/demo
```

### Check MediaMTX stats
```bash
curl http://localhost:8888
```

## Step 4: Check Frigate

Open http://localhost:5000 and you should see your camera feed.

## Troubleshooting

### Black screen in Frigate
- Ensure FFmpeg is running and publishing to `:8554/demo`
- Check MediaMTX logs: `docker logs vi_mediamtx`
- Verify Frigate can reach localhost (network_mode: host)

### Permission denied (macOS)
```bash
# Grant camera access to Terminal/iTerm
System Settings → Privacy & Security → Camera → Enable Terminal
```

### No video device (Linux)
```bash
# Check if camera is detected
lsusb | grep -i camera

# Check permissions
sudo usermod -a -G video $USER
# Log out and back in
```

### Laggy stream
```bash
# Lower resolution
-vf scale=-2:480 -r 10

# Or use faster preset (more CPU)
-preset ultrafast
```

### Audio not needed?
```bash
# Add -an to disable audio
ffmpeg ... -an -f rtsp ...
```

## Advanced: Multiple Cameras

```bash
# Camera 1
ffmpeg -f avfoundation -i "0:0" ... rtsp://localhost:8554/cam1

# Camera 2  
ffmpeg -f avfoundation -i "1:0" ... rtsp://localhost:8554/cam2
```

Then update `compose/frigate.yml`:
```yaml
cameras:
  cam1:
    ffmpeg:
      inputs:
        - path: rtsp://localhost:8554/cam1
          roles: [detect]
  cam2:
    ffmpeg:
      inputs:
        - path: rtsp://localhost:8554/cam2
          roles: [detect]
```

## Full Example Workflow

```bash
# Terminal 1: Start stack
cd video-intel
make up

# Terminal 2: Publish webcam (macOS)
ffmpeg -f avfoundation -framerate 30 -pixel_format uyvy422 -i "0:0" \
  -vf scale=-2:720 -r 15 \
  -c:v h264_videotoolbox -profile:v high -g 30 -tune zerolatency \
  -f rtsp -rtsp_transport tcp rtsp://localhost:8554/demo

# Terminal 3: Watch MQTT events
make cli

# Browser: Open Frigate UI
open http://localhost:5000
```

## Quick Reference

| Platform | Input Format | Encoder | Notes |
|----------|-------------|---------|-------|
| macOS | avfoundation | h264_videotoolbox | Hardware accelerated |
| Linux | v4l2 | libx264 | Software encoder |
| Windows | dshow | libx264 | Software encoder |
| Raspberry Pi | v4l2 | h264_omx | Hardware on Pi 4 |

## Stop Publishing

Press `Ctrl+C` in the FFmpeg terminal or:
```bash
pkill -f "ffmpeg.*rtsp://localhost:8554"
```
