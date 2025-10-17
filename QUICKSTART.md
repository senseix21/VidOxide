# 🚀 Quick Start Guide

## Current Status
Docker is pulling images (Frigate is ~500MB, takes 5-10 min on first run).

## What's Happening
```bash
make up  # Running now - pulling:
  ✅ mosquitto (2MB) 
  ✅ mediamtx (20MB)
  ✅ ffmpeg (50MB)
  ⏳ frigate (500MB) <- downloading now
```

## Once `make up` Completes

### 1. Check Services
```bash
docker ps --filter "name=vi_"
# Should show 5 containers running
```

### 2. View Logs
```bash
make logs
# Watch detection events in real-time
```

### 3. Access UIs
- **Frigate**: http://localhost:5000
- **MediaMTX stats**: http://localhost:8888  
- **Agent health**: http://localhost:8080/healthz

### 4. Test RTSP Stream
```bash
# Use VLC or ffplay
ffplay rtsp://localhost:8554/demo

# Or curl MediaMTX
curl http://localhost:8888
```

### 5. Run Rust Apps Locally (Development)
```bash
# Terminal 1: CLI subscriber
make cli
# Prints events like: "new: person on demo (id=abc123)"

# Terminal 2: Agent with web server
make agent
# Access: http://localhost:8080/healthz
```

## Troubleshooting

### Docker pull slow?
```bash
# Check progress
docker ps -a

# Or monitor in another terminal
cd compose && docker compose logs -f
```

### Port conflicts?
```bash
# Check what's using ports
lsof -i :5000 -i :8554 -i :1883

# Kill native MediaMTX if still running
pkill mediamtx
```

### Start fresh
```bash
make down  # Stop & clean
make up    # Restart
```

## Expected First-Run Timeline
- Image pull: 5-10 min (first time only)
- Container start: 30 sec
- Frigate init: 10-20 sec
- FFmpeg connect: 5 sec
- First detection: ~10 sec after video starts

## Success Indicators
✅ `docker ps` shows 5 containers  
✅ Frigate UI loads at localhost:5000  
✅ `make logs` shows "person detected" events  
✅ `make cli` prints MQTT messages  

## Next Steps (After Stack is Running)
1. Open Frigate UI: http://localhost:5000
2. Click "demo" camera to see live feed
3. Watch Events tab for detections
4. Run `make cli` to see MQTT events
5. Edit code in `apps/` and re-run with `make cli/agent`
