# 🚀 Quick Start - Enhanced Detection System

## 3-Step Setup

### 1️⃣ Start Webcam Stream
```bash
./stream-webcam.sh
```
✅ Streams to: `rtsp://localhost:8554/demo`

### 2️⃣ Run Detection Monitor
```bash
make monitor
```
✅ Publishes idle status every 2 seconds

### 3️⃣ Watch Events in CLI
```bash
make cli
```
✅ Shows all detections + idle messages

---

## Expected Output

### When No Objects Detected:
```
💤 demo - No objects detected (last: 5 seconds ago)
💤 demo - No objects detected (last: 7 seconds ago)
```

### When Objects Detected:
```
🎯 new: person detected on demo (confidence: 85.3%)
🎯 update: cup detected on demo (confidence: 45.8%)
🎯 end: person detected on demo
```

---

## What's Detected? (80+ Objects)

**Will definitely work:**
- 👤 Person (just sit in front of camera!)
- 📱 Cell phone
- 📖 Book
- ☕ Cup/Mug
- 💻 Laptop
- 🖱️ Mouse
- 📺 Remote control

**Might work:**
- 🪑 Chair
- 🍌 Banana
- ✂️ Scissors
- 🎾 Sports ball
- 🎒 Backpack

---

## Troubleshooting

**No detections?**
```bash
curl http://localhost:5000/api/stats | jq '.cameras.demo.detection_fps'
```
Should show: `> 0`

**No idle messages?**
Check monitor is running:
```bash
ps aux | grep detection-monitor
```

**CLI not showing status?**
Subscribe to all topics:
```bash
./target/release/frigate-cli --topic "frigate/#"
```

---

## Quick Commands

```bash
make up        # Start all services
make monitor   # Run detection monitor
make cli       # Run CLI event viewer
make down      # Stop everything
```

---

**Open Frigate UI**: http://localhost:5000

See `DETECTION_TEST.md` for full guide.
