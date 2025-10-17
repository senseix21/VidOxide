# 🎥 video-intel

**One-command video intelligence stack:** FFmpeg → MediaMTX (RTSP) → Frigate (object detection) → MQTT → Rust event processors.

## 🚀 Quick Start

```bash
# 1. Start the Docker stack
make up

# 2. Publish webcam to MediaMTX (choose your platform)

# macOS (avfoundation + hardware encoder)
ffmpeg -f avfoundation -framerate 30 -pixel_format uyvy422 -i "0:0" \
  -vf scale=-2:720 -r 15 \
  -c:v h264_videotoolbox -profile:v high -g 30 -tune zerolatency \
  -f rtsp -rtsp_transport tcp rtsp://localhost:8554/demo

# Linux (v4l2 + x264)
ffmpeg -f v4l2 -framerate 30 -video_size 1280x720 -i /dev/video0 \
  -vf scale=-2:720 -r 15 \
  -c:v libx264 -preset veryfast -tune zerolatency -g 30 \
  -f rtsp -rtsp_transport tcp rtsp://localhost:8554/demo

# 3. Watch logs
make logs
```

**Access Points:**
- Frigate UI: http://localhost:5000
- MediaMTX RTSP: rtsp://localhost:8554/demo
- MQTT Broker: localhost:1883
- Frigate Agent health: http://localhost:8080/healthz

## 📦 What's Included

### Services (Docker Compose)
- **MediaMTX**: RTSP/HLS/WebRTC server receiving FFmpeg streams
- **FFmpeg**: Loops demo.mp4 as H.264 RTSP stream @ 720p/15fps
- **Frigate**: Real-time object detection (person, car tracking)
- **Mosquitto**: MQTT broker for event streaming
- **frigate-cli**: Rust CLI app printing detection events to stdout
- **frigate-agent**: Rust service with /healthz endpoint + MQTT consumer

### Rust Apps (Workspace)
```
apps/
├── frigate-cli/     # Minimal MQTT subscriber & event printer
└── frigate-agent/   # Web server + MQTT processor (M1: thumbnails, metrics)
```

## 🛠️ Development

### Run Rust apps locally (outside containers)
```bash
# Run CLI against local MQTT broker
make cli

# Run agent with web server
make agent
```

### Configuration
- **Frigate**: `compose/frigate.yml` (camera config, object tracking)
- **MediaMTX**: `compose/mediamtx.yml` (RTSP/HLS endpoints)
- **MQTT**: `compose/mosquitto-no-auth.conf` (open broker for dev)
- **Environment**: `.env.example` → copy to `.env` for customization

### Health Check
```bash
bash infra/scripts/health.sh
```

## 📋 Roadmap

- **M0**: ✅ Boot stack + CLI (current)
- **M1**: Agent v0.1 (thumbnails, /healthz, Prometheus metrics)
- **M2**: Storage & rules (SQLite/Postgres, S3, event deduplication)
- **M3**: Ops hardening (CI, auth, TLS, Grafana dashboards)

See [docs/ROADMAP.md](docs/ROADMAP.md) for details.

## 🏗️ Architecture

```
FFmpeg (demo.mp4 loop)
    ↓ RTSP (H.264)
MediaMTX (:8554)
    ↓ RTSP relay
Frigate (object detection)
    ↓ MQTT events (frigate/events)
Mosquitto (:1883)
    ↓
┌───────────────────┐
│ frigate-cli       │  (stdout logger)
│ frigate-agent     │  (web + processor)
└───────────────────┘
```

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) for system design.

## 🧪 Testing

```bash
# Check logs for detection events
docker compose -f compose/docker-compose.yml logs -f vi_frigate_cli

# Verify RTSP stream
ffplay rtsp://localhost:8554/demo

# Query Frigate API
curl http://localhost:5000/api/events
```

## 🛑 Stop & Clean

```bash
make down  # stops containers + removes volumes
```

## 📝 Notes

- **No auth** on MQTT/Frigate for MVP (add in M3)
- **Sample video**: 640x360 MP4 from filesamples.com
- **Detection**: Frigate uses CPU inference (add GPU in M2)
- **Persistence**: Events in-memory only (add DB in M2)

## 📚 Documentation

- [Architecture](docs/ARCHITECTURE.md)
- [Roadmap](docs/ROADMAP.md)
- [Runbooks](docs/RUNBOOKS.md)
- [Troubleshooting](docs/TROUBLESHOOTING.md)

## 🤝 Contributing

1. Fork the repo
2. Create feature branch (`git checkout -b feat/amazing`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feat/amazing`)
5. Open Pull Request

## 📄 License

MIT

---

**Built for real-time video intelligence at scale.**
# video-intel
