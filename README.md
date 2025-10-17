# 🎥 video-intel

**Real-time video intelligence:** Webcam → FFmpeg → MediaMTX (RTSP) → Frigate (AI detection) → MQTT → Rust CLI

## ✨ Features

- 🎯 Real-time object detection (person, car, etc.)
- 📹 Webcam streaming via FFmpeg
- 🔔 Live event notifications via MQTT
- 🦀 Rust CLI for event monitoring
- 🌐 Web UI for video playback and events
- 🐳 One-command Docker deployment

## 🚀 Quick Start

### 1. Start Infrastructure

```bash
cd video-intel
make up
```

Starts: Mosquitto (MQTT), MediaMTX (RTSP), Frigate (AI)

### 2. Stream Your Webcam

**macOS:**
```bash
ffmpeg -f avfoundation -framerate 30 -pixel_format uyvy422 -i "0:0" \
  -vf scale=-2:720 -r 15 \
  -c:v h264_videotoolbox -profile:v high -g 30 -tune zerolatency \
  -f rtsp -rtsp_transport tcp rtsp://localhost:8554/demo
```

**Linux:**
```bash
ffmpeg -f v4l2 -framerate 30 -video_size 1280x720 -i /dev/video0 \
  -vf scale=-2:720 -r 15 \
  -c:v libx264 -preset veryfast -tune zerolatency -g 30 \
  -f rtsp -rtsp_transport tcp rtsp://localhost:8554/demo
```

**Keep this terminal running!**

### 3. Restart Frigate & Run CLI

```bash
# Wait 10 seconds after starting FFmpeg, then:
docker restart vi_frigate
sleep 15

# Run the event CLI
make cli
```

### 4. See Detections!

Move in front of your webcam:
```
new: person on demo (id=1760664012.618671-12580m)
update: person on demo (id=1760664012.618671-12580m)
end: person on demo (id=1760664012.618671-12580m)
```

## 🌐 Access Points

- **Frigate UI**: http://localhost:5000
- **MQTT Broker**: localhost:1883
- **RTSP Stream**: rtsp://localhost:8554/demo

## 📦 What's Included

### Docker Services
- **MediaMTX** - RTSP server (port 8554)
- **Mosquitto** - MQTT broker (port 1883)  
- **Frigate** - AI object detection (port 5000)

### Rust Applications
- **frigate-cli** - Event notification CLI
- **frigate-agent** - Web service with health checks

## 📁 Structure

```
video-intel/
├── compose/              # Docker configs
│   ├── docker-compose.yml
│   ├── frigate.yml
│   ├── mediamtx.yml
│   └── mosquitto-no-auth.conf
├── apps/
│   ├── frigate-cli/      # Rust MQTT subscriber
│   └── frigate-agent/    # Rust web service
├── docs/
├── START_HERE.md         # Detailed setup
└── WEBCAM_SETUP.md       # Platform-specific guide
```

## 🛠️ Commands

```bash
make up       # Start Docker services
make down     # Stop and remove services  
make logs     # View all logs
make cli      # Run Rust CLI event viewer
make agent    # Run Rust agent service
```

## ⚙️ Configuration

- **Frigate**: `compose/frigate.yml` - Camera settings, object detection
- **MediaMTX**: `compose/mediamtx.yml` - RTSP paths
- **MQTT**: `compose/mosquitto-no-auth.conf` - Broker config

## 🔧 Troubleshooting

### No video in Frigate UI
1. Check FFmpeg is running: `ps aux | grep ffmpeg`
2. Verify stream: `ffplay rtsp://localhost:8554/demo`
3. Restart Frigate: `docker restart vi_frigate`

### No detection events
1. Open http://localhost:5000 - verify live video
2. Move visibly for 3-5 seconds (CPU detection is slow)
3. Check objects tracked in `compose/frigate.yml`

### Camera permission (macOS)
System Settings → Privacy & Security → Camera → Enable Terminal

## 📚 Documentation

- [START_HERE.md](START_HERE.md) - Complete walkthrough
- [WEBCAM_SETUP.md](WEBCAM_SETUP.md) - Platform guides
- [TEST_COMMANDS.md](TEST_COMMANDS.md) - Testing steps
- [docs/](docs/) - Architecture & troubleshooting

## 🎯 Use Cases

- Real-time security monitoring
- Pet activity tracking  
- Smart home automation triggers
- Occupancy detection
- Event logging and analytics

## 🚧 Roadmap

- [ ] Multi-camera support
- [ ] SQLite event storage
- [ ] Prometheus metrics
- [ ] Grafana dashboards
- [ ] Custom detection zones
- [ ] Push notifications
- [ ] Cloud storage integration

## 📄 License

MIT

---

**Status:** ✅ Production ready | Built with Rust 🦀 + Docker 🐳 + AI 🤖

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
