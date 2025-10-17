# 🚀 Quick Start - Video Intelligence

Get your video intelligence system running in 3 minutes!

## Step 1: Start Infrastructure (30 seconds)

```bash
cd video-intel
make up
```

Wait for containers to start. You should see:
```
✔ Container vi_mosquitto  Started
✔ Container vi_mediamtx   Started
✔ Container vi_frigate    Started
```

## Step 2: Stream Webcam (10 seconds to start)

**Open a NEW terminal** and run:

### macOS:
```bash
ffmpeg -f avfoundation -framerate 30 -pixel_format uyvy422 -i "0:0" \
  -vf scale=-2:720 -r 15 \
  -c:v h264_videotoolbox -profile:v high -g 30 -tune zerolatency \
  -f rtsp -rtsp_transport tcp rtsp://localhost:8554/demo
```

### Linux:
```bash
ffmpeg -f v4l2 -framerate 30 -video_size 1280x720 -i /dev/video0 \
  -vf scale=-2:720 -r 15 \
  -c:v libx264 -preset veryfast -tune zerolatency -g 30 \
  -f rtsp -rtsp_transport tcp rtsp://localhost:8554/demo
```

**Leave this running!** You should see:
```
frame=   50 fps= 14 q=-0.0 size=N/A time=00:00:03.26 bitrate=N/A
```

## Step 3: Restart Frigate (20 seconds)

**Open a 3rd terminal:**

```bash
docker restart vi_frigate
sleep 15
```

## Step 4: Run Event CLI

In the same terminal:

```bash
cd video-intel
make cli
```

You should see:
```
subscribed to localhost:1883 topic frigate/events
```

## Step 5: Test Detection! 👋

**Move in front of your webcam!**

You'll see events appear:
```
new: person on demo (id=1760664012.618671-12580m)
update: person on demo (id=1760664012.618671-12580m)
update: person on demo (id=1760664012.618671-12580m)
end: person on demo (id=1760664012.618671-12580m)
```

## Step 6: View in Browser

Open: http://localhost:5000

You'll see:
- Live webcam feed
- Bounding boxes around detected objects
- Event list on the right

---

## 🎉 That's it!

Your video intelligence system is now running with:
- ✅ Real-time object detection
- ✅ MQTT event streaming
- ✅ Rust CLI notifications
- ✅ Web UI for monitoring

## Next Steps

- Configure detection in `compose/frigate.yml`
- Add more objects to track: `track: [person, car, dog, cat]`
- Lower threshold for easier detection: `threshold: 0.4`
- Build your own event processor using the MQTT stream

## Troubleshooting

**No video?**
- Check FFmpeg is running: `ps aux | grep ffmpeg`
- Verify stream: `ffplay rtsp://localhost:8554/demo`

**No events?**
- Make sure you're visible in frame for 3-5 seconds
- CPU detection is slower - be patient!
- Check Frigate UI shows video

**Camera permission denied? (macOS)**
System Settings → Privacy & Security → Camera → Enable Terminal

---

**Need help?** See [README.md](README.md) for full documentation.
