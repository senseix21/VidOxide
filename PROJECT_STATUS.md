# 📊 Project Status

## ✅ What's Working

### Infrastructure (Docker)
- ✅ **Mosquitto MQTT** - Running on localhost:1883
- ✅ **MediaMTX RTSP** - Running on localhost:8554  
- ✅ **Frigate** - Running on localhost:5000 (UI accessible)

### Rust Applications
- ✅ **frigate-cli** - Compiles, ready to run with `make cli`
- ✅ **frigate-agent** - Compiles, ready to run with `make agent`

### Configuration
- ✅ All config files in place (frigate.yml, mediamtx.yml, mosquitto.conf)
- ✅ Makefiles for easy commands
- ✅ Documentation complete

## 🎯 What You Need to Do

### **ONLY 1 STEP:** Publish your webcam

Run this command in a terminal:

**macOS:**
\`\`\`bash
ffmpeg -f avfoundation -framerate 30 -pixel_format uyvy422 -i "0:0" \\
  -vf scale=-2:720 -r 15 \\
  -c:v h264_videotoolbox -profile:v high -g 30 -tune zerolatency \\
  -f rtsp -rtsp_transport tcp rtsp://localhost:8554/demo
\`\`\`

**Linux:**
\`\`\`bash
ffmpeg -f v4l2 -framerate 30 -video_size 1280x720 -i /dev/video0 \\
  -vf scale=-2:720 -r 15 \\
  -c:v libx264 -preset veryfast -tune zerolatency -g 30 \\
  -f rtsp -rtsp_transport tcp rtsp://localhost:8554/demo
\`\`\`

Then:
1. Open http://localhost:5000 → See live feed
2. Run \`make cli\` → See detection events
3. Move in frame → Watch events appear!

## 📁 Project Structure

\`\`\`
video-intel/
├── compose/              # Docker configs
│   ├── docker-compose.yml
│   ├── frigate.yml
│   ├── mediamtx.yml
│   └── mosquitto-no-auth.conf
├── apps/                 # Rust workspace
│   ├── frigate-cli/      # MQTT subscriber CLI
│   └── frigate-agent/    # Web server + processor
├── docs/                 # Documentation
├── infra/                # Scripts & Makefile
├── START_HERE.md         # 👈 Complete setup guide
├── WEBCAM_SETUP.md       # Platform-specific webcam guide
└── README.md             # Project overview
\`\`\`

## 🚀 Quick Commands

\`\`\`bash
make up      # Start Docker stack
make down    # Stop & clean
make logs    # View logs
make cli     # Run Rust CLI
make agent   # Run Rust agent
\`\`\`

## 📋 Next Steps (After Webcam Works)

### Milestone 1 (M1) - Agent Enhancements
- [ ] Fetch thumbnails for each event
- [ ] Add Prometheus metrics at /metrics
- [ ] Health checks with retry logic

### Milestone 2 (M2) - Storage & Rules
- [ ] SQLite event storage
- [ ] S3 integration for recordings
- [ ] Event deduplication
- [ ] Custom detection rules

### Milestone 3 (M3) - Production
- [ ] CI/CD pipeline
- [ ] Authentication & TLS
- [ ] Grafana dashboards
- [ ] Multi-camera support

## 🎓 Key Design Decisions

1. **Webcam from host** - No file-based demo, real-time only
2. **Frigate in host network mode** - Accesses localhost:8554
3. **Rust CLI runs on host** - Faster iteration, no container rebuild
4. **TCP transport** - More reliable than UDP for RTSP
5. **CPU detection** - Works everywhere, GPU optional later

## 📊 Performance Expectations

- **Detection latency**: 1-2 seconds (CPU mode)
- **Video latency**: <500ms (local network)
- **Event throughput**: ~100 events/min (typical home use)

## 🔗 Important Links

- Frigate UI: http://localhost:5000
- MediaMTX RTSP: rtsp://localhost:8554/demo
- Agent health: http://localhost:8080/healthz (when running)
- MQTT broker: mqtt://localhost:1883

## 💡 Tips

- Keep FFmpeg terminal open (it must run continuously)
- Frigate UI takes ~10-20s to initialize on first start
- CPU detection is slower but works everywhere
- Hardware encoding (h264_videotoolbox on macOS) saves CPU

---

**Status**: Ready for webcam publishing! 🎥

See [START_HERE.md](START_HERE.md) for step-by-step instructions.
