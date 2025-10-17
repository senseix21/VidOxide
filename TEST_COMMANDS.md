# 🧪 Testing Commands - Step by Step

## Prerequisites Check

```bash
# Make sure you're in the project directory
cd /Users/abuhamzah/Dev/rust/projects/erik/video-intel

# Check Docker is running
docker ps

# If Docker isn't running
open -a Docker && sleep 10
```

## Test 1: Start the Docker Stack

```bash
# Start all services (Mosquitto, MediaMTX, Frigate)
make up

# Expected output: All 3 containers created and started
# Wait 10 seconds for services to initialize
```

**Verify:**
```bash
# Check all services are running
docker ps --filter "name=vi_"

# Expected: 3 containers (vi_mosquitto, vi_mediamtx, vi_frigate)
# Status: Up X minutes (healthy) for Frigate
```

## Test 2: Check Available Cameras

```bash
# List your webcam devices
ffmpeg -f avfoundation -list_devices true -i "" 2>&1 | grep -A 5 "AVFoundation video devices"

# Expected output:
# [0] FaceTime HD Camera (Built-in)
# [1] Capture screen 0
# etc.
```

## Test 3: Stream Webcam to MediaMTX

**Open a NEW terminal window/tab** and run:

```bash
# For macOS (replace "0:0" if your camera is at a different index)
ffmpeg -f avfoundation -framerate 30 -pixel_format uyvy422 -i "0:0" \
  -vf scale=-2:720 -r 15 \
  -c:v h264_videotoolbox -profile:v high -g 30 -tune zerolatency \
  -f rtsp -rtsp_transport tcp rtsp://localhost:8554/demo
```

**For Linux:**
```bash
# Replace /dev/video0 with your camera device
ffmpeg -f v4l2 -framerate 30 -video_size 1280x720 -i /dev/video0 \
  -vf scale=-2:720 -r 15 \
  -c:v libx264 -preset veryfast -tune zerolatency -g 30 \
  -f rtsp -rtsp_transport tcp rtsp://localhost:8554/demo
```

**Expected output:**
```
Input #0, avfoundation, from '0:0':
  Duration: N/A, start: ...
  Stream #0:0: Video: rawvideo, uyvy422, 1280x720, 30 tbr
Output #0, rtsp, to 'rtsp://localhost:8554/demo':
  Stream #0:0: Video: h264, yuv420p, 1280x720, 15 fps
frame=   50 fps= 14 q=-0.0 size=N/A time=00:00:03.26 bitrate=N/A
```

**Leave this terminal running!** FFmpeg must continue streaming.

## Test 4: Verify Stream is Received

**Open another terminal:**

```bash
# Check MediaMTX logs
docker logs vi_mediamtx 2>&1 | tail -10

# Expected to see:
# [RTSP] [session ...] is publishing to path 'demo', 2 tracks (H264, MPEG-4 Audio)
```

**Or test with ffplay (if installed):**
```bash
ffplay -rtsp_transport tcp rtsp://localhost:8554/demo
# Should show your webcam feed
```

## Test 5: Check Frigate is Processing

```bash
# Restart Frigate to pick up the stream
docker restart vi_frigate

# Wait 15 seconds
sleep 15

# Check Frigate logs
docker logs vi_frigate 2>&1 | tail -20

# Expected to see:
# [INFO] Capture process started for demo
# [INFO] FastAPI started
```

## Test 6: Run Rust CLI (Watch for Events)

**Open another terminal:**

```bash
cd /Users/abuhamzah/Dev/rust/projects/erik/video-intel

# Run the CLI
make cli

# Expected output:
# Compiling... (first time only)
# subscribed to localhost:1883 topic frigate/events
```

**Leave this running!** It will print events when Frigate detects objects.

## Test 7: Trigger Detection Events

**Now move in front of your webcam!**

Watch the Rust CLI terminal for output like:
```
new: person on demo (id=1729097123.456-abc123)
update: person on demo (id=1729097123.456-abc123)
update: person on demo (id=1729097123.456-abc123)
end: person on demo (id=1729097123.456-abc123)
```

**Note:** First detection may take 10-30 seconds (CPU mode is slower).

## Test 8: Check Frigate UI

```bash
# Open Frigate in browser
open http://localhost:5000

# Or manually browse to: http://localhost:5000
```

**What to check:**
1. Click on "demo" camera → should see live feed
2. Click "Events" tab → should see detection events
3. Click on an event → should see snapshot with bounding box

## Test 9: View All Logs

```bash
# See logs from all services
make logs

# Or individual services:
docker logs -f vi_frigate
docker logs -f vi_mediamtx
docker logs -f vi_mosquitto
```

## Test 10: Test MQTT Directly (Optional)

```bash
# Install mosquitto clients if not already
# brew install mosquitto  # macOS
# sudo apt-get install mosquitto-clients  # Linux

# Subscribe to all Frigate topics
mosquitto_sub -h localhost -p 1883 -t "frigate/#" -v
```

## Quick Status Check

```bash
# Run this anytime to check status
cd /Users/abuhamzah/Dev/rust/projects/erik/video-intel

echo "=== Docker Services ==="
docker ps --filter "name=vi_" --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"

echo -e "\n=== FFmpeg Stream ==="
ps aux | grep "ffmpeg.*8554" | grep -v grep || echo "Not running"

echo -e "\n=== Rust CLI ==="
ps aux | grep "frigate-cli" | grep -v grep || echo "Not running"

echo -e "\n=== MediaMTX Active Streams ==="
docker logs vi_mediamtx 2>&1 | grep "publishing to path" | tail -1
```

## Cleanup Commands

```bash
# Stop FFmpeg (in its terminal, press Ctrl+C)
# Or kill it:
pkill -f "ffmpeg.*rtsp://localhost:8554"

# Stop Rust CLI (in its terminal, press Ctrl+C)

# Stop Docker stack
make down

# Or keep services but remove volumes:
docker compose -f compose/docker-compose.yml down -v
```

## Troubleshooting Commands

### Frigate shows "No Signal"
```bash
# Check if stream is active
curl -I rtsp://localhost:8554/demo

# Restart Frigate
docker restart vi_frigate
```

### No detections appearing
```bash
# Check Frigate config
docker exec vi_frigate cat /config/config.yml

# Check detection is enabled
docker logs vi_frigate 2>&1 | grep -i detect | tail -10
```

### Permission denied for camera (macOS)
```bash
# Grant camera access:
# System Settings → Privacy & Security → Camera → Enable Terminal/iTerm
```

### Port conflicts
```bash
# Find what's using the ports
lsof -i :1883  # MQTT
lsof -i :8554  # RTSP
lsof -i :5000  # Frigate UI

# Kill if needed
lsof -ti:8554 | xargs kill
```

## Complete Test Workflow (Copy-Paste)

Here's everything in sequence for easy testing:

```bash
# Terminal 1: Start stack
cd /Users/abuhamzah/Dev/rust/projects/erik/video-intel
make up
sleep 10
docker ps --filter "name=vi_"
```

```bash
# Terminal 2: Start webcam stream (macOS)
ffmpeg -f avfoundation -framerate 30 -pixel_format uyvy422 -i "0:0" \
  -vf scale=-2:720 -r 15 \
  -c:v h264_videotoolbox -profile:v high -g 30 -tune zerolatency \
  -f rtsp -rtsp_transport tcp rtsp://localhost:8554/demo
# Keep running!
```

```bash
# Terminal 3: Restart Frigate and run CLI
cd /Users/abuhamzah/Dev/rust/projects/erik/video-intel
sleep 15
docker restart vi_frigate
sleep 10
make cli
# Watch for events!
```

```bash
# Browser: Open Frigate UI
open http://localhost:5000
```

**Now move in front of webcam and watch Terminal 3 for detection events!**

---

## Expected Timeline

- **0:00** - `make up` starts services
- **0:10** - Services ready, start FFmpeg
- **0:15** - FFmpeg connected to MediaMTX
- **0:25** - Restart Frigate to pick up stream
- **0:35** - Start Rust CLI
- **0:40** - Move in front of camera
- **0:50** - First detection appears in CLI! 🎉

## Success Indicators

✅ `docker ps` shows 3 running containers  
✅ FFmpeg shows `frame=... fps=...` incrementing  
✅ MediaMTX logs show "publishing to path 'demo'"  
✅ Frigate logs show "Capture process started"  
✅ Rust CLI shows "subscribed to localhost:1883"  
✅ Moving triggers CLI output with "person" events  
✅ Frigate UI shows live camera feed  

---

**Happy testing! 🚀**
