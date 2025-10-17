# 🎬 START HERE - Complete Setup Guide

## ✅ Current Status

Your stack is **RUNNING**:
```
✅ vi_mosquitto  - MQTT broker (localhost:1883)
✅ vi_mediamtx   - RTSP server (localhost:8554)
✅ vi_frigate    - Object detection (localhost:5000)
```

## 🚀 Next Steps

### 1. Publish Your Webcam

Choose your platform:

#### macOS (Recommended)
```bash
# List cameras first
ffmpeg -f avfoundation -list_devices true -i ""

# Publish (replace "0:0" with your device index)
ffmpeg -f avfoundation -framerate 30 -pixel_format uyvy422 -i "0:0" \
  -vf scale=-2:720 -r 15 \
  -c:v h264_videotoolbox -profile:v high -g 30 -tune zerolatency \
  -f rtsp -rtsp_transport tcp rtsp://localhost:8554/demo
```

#### Linux
```bash
# List cameras
v4l2-ctl --list-devices

# Publish (replace /dev/video0 with your device)
ffmpeg -f v4l2 -framerate 30 -video_size 1280x720 -i /dev/video0 \
  -vf scale=-2:720 -r 15 \
  -c:v libx264 -preset veryfast -tune zerolatency -g 30 \
  -f rtsp -rtsp_transport tcp rtsp://localhost:8554/demo
```

**Keep this terminal open** - FFmpeg must run continuously.

### 2. Verify Stream (Optional)

```bash
# Test with ffplay
ffplay -rtsp_transport tcp rtsp://localhost:8554/demo

# Or with VLC
# Media → Open Network Stream → rtsp://localhost:8554/demo
```

### 3. Open Frigate UI

```bash
open http://localhost:5000
# Or browse to: http://localhost:5000
```

You should see:
- "demo" camera in the list
- Live video feed
- Detection zones (if configured)

### 4. Run Rust CLI (Watch Events)

Open a **new terminal**:

```bash
cd /Users/abuhamzah/Dev/rust/projects/erik/video-intel
make cli
```

Expected output when you move in frame:
```
subscribed to localhost:1883 topic frigate/events
new: person on demo (id=1729...)
update: person on demo (id=1729...)
end: person on demo (id=1729...)
```

### 5. (Optional) Run Rust Agent

Another terminal:

```bash
make agent
```

Then check health:
```bash
curl http://localhost:8080/healthz
# Should return: ok
```

## 📊 Useful Commands

```bash
# View logs
make logs

# Stop everything
make down

# Restart stack
make down && make up

# Check container status
docker ps --filter "name=vi_"

# View specific service logs
docker logs -f vi_frigate
docker logs -f vi_mediamtx
docker logs -f vi_mosquitto
```

## 🎯 What to Expect

1. **FFmpeg**: Continuous stream, no errors
2. **Frigate UI**: Live camera feed at http://localhost:5000
3. **Rust CLI**: Event messages when objects detected
4. **Detection lag**: ~1-2 seconds on CPU (normal)

## 🐛 Troubleshooting

### Frigate shows "No Signal" or black screen
```bash
# Check MediaMTX received the stream
curl http://localhost:8554

# Check Frigate logs
docker logs vi_frigate | tail -20

# Verify FFmpeg is publishing
ps aux | grep ffmpeg
```

### No detections appearing
- **Move in frame** - CPU detection is slower
- Check Frigate Events tab in UI
- Ensure `objects.track: [person]` in `compose/frigate.yml`
- Lower resolution if too slow: `-vf scale=-2:480`

### Permission denied (macOS camera)
```bash
# Grant camera access
System Settings → Privacy & Security → Camera → Enable Terminal/iTerm
```

### Port conflicts
```bash
# Find what's using ports
lsof -i :8554 -i :1883 -i :5000

# Kill if needed
lsof -ti:8554 | xargs kill
```

## 📚 Full Documentation

- [README.md](README.md) - Complete project overview
- [WEBCAM_SETUP.md](WEBCAM_SETUP.md) - Detailed webcam publishing guide
- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) - System design
- [docs/TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md) - Common issues

## 🎓 Learning Path

1. ✅ Get webcam streaming (you're here)
2. 📝 Customize detection in `compose/frigate.yml`
3. 🦀 Modify Rust apps in `apps/frigate-cli` and `apps/frigate-agent`
4. 📊 Add Prometheus metrics (M1)
5. 💾 Add event storage (M2)

## 💡 Pro Tips

- **Hardware encoding** (macOS): `-c:v h264_videotoolbox` (faster, less CPU)
- **Lower latency**: `-tune zerolatency -g 15`
- **Multiple cameras**: See [WEBCAM_SETUP.md](WEBCAM_SETUP.md#advanced-multiple-cameras)
- **Development**: Edit code in `apps/`, then `make cli` to test instantly

---

**Ready to detect!** 🚀

Start with step 1 above (publish webcam) and watch the magic happen.
