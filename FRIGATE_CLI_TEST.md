# Frigate CLI Testing Guide

## Prerequisites

Make sure Frigate and MQTT broker are running:

```bash
# Check if containers are running
docker ps | grep -E "(frigate|mosquitto)"

# Start if not running
cd compose && docker-compose up -d
```

## Test 1: Basic Connection Test

```bash
# Run with verbose mode to see ALL MQTT traffic
make cli ARGS="--verbose"

# Or manually
cd apps/frigate-cli && cargo run --release -- --broker-host localhost --broker-port 1883 --topic "frigate/#" --verbose
```

**Expected output:**
```
🔌 Connecting to MQTT broker at localhost:1883...
✅ Connected to MQTT broker
📡 Subscribed to topic 'frigate/#'
🔍 Listening for Frigate events...

📬 Topic: frigate/stats (234 bytes)
📊 frigate/stats: {"cpu_usages": {...}}
💓 Keepalive (msgs: 5)
```

## Test 2: Detection Events Only (Default)

```bash
# Listen only for detection events
make cli

# Or manually
cd apps/frigate-cli && cargo run --release -- --broker-host localhost --broker-port 1883 --topic "frigate/events"
```

**Expected output when detection happens:**
```
🎯 new: person detected on demo (confidence: 85.3%) [id: 1234.567]
🎯 update: person detected on demo (confidence: 87.2%) [id: 1234.567]
🎯 end: person detected on demo [id: 1234.567]
```

## Test 3: Trigger a Detection

### Option A: Wave at the camera
1. Make sure your webcam is working
2. Wave your hand in front of the camera
3. Check Frigate UI: http://localhost:5000

### Option B: Use test video
```bash
# Stream a test video with person
ffmpeg -re -i test_video_with_person.mp4 \
  -c:v libx264 -preset ultrafast -f rtsp \
  rtsp://localhost:8554/demo
```

### Option C: Publish test event manually
```bash
# Install mosquitto-clients if not available
apt-get install mosquitto-clients  # Ubuntu/Debian
brew install mosquitto              # macOS

# Publish test event
mosquitto_pub -h localhost -p 1883 -t "frigate/events" -m '{
  "type": "new",
  "before": null,
  "after": {
    "id": "test123",
    "camera": "demo",
    "label": "person",
    "score": 0.89,
    "current_zones": ["yard"],
    "box": [100, 100, 200, 200]
  }
}'
```

**Expected CLI output:**
```
🎯 new: person detected on demo (confidence: 89.0%) [zones: yard] [id: test123]
```

## Test 4: Check Frigate Status

```bash
# Check if Frigate is detecting anything
curl -s http://localhost:5000/api/stats | jq '.detection_fps'

# Check recent events
curl -s http://localhost:5000/api/events | jq '.[0:3]'
```

## Troubleshooting

### No messages at all?

1. **Check if MQTT broker is running:**
   ```bash
   docker ps | grep mosquitto
   ```

2. **Test MQTT connection directly:**
   ```bash
   # Subscribe to all topics
   mosquitto_sub -h localhost -p 1883 -t "#" -v
   ```

3. **Check Frigate logs:**
   ```bash
   docker logs vi_frigate --tail 50
   ```

### CLI connects but no detection events?

1. **Run in verbose mode to see all MQTT traffic:**
   ```bash
   make cli ARGS="--verbose"
   ```

2. **Check if Frigate is detecting objects:**
   - Open http://localhost:5000
   - Look at the live feed
   - Check if bounding boxes appear around objects

3. **Check Frigate config:**
   ```bash
   cat compose/frigate.yml
   ```

### Connection refused?

Make sure the broker is accessible:
```bash
# Test connection
telnet localhost 1883

# Check docker network
docker network inspect compose_default
```

## What You Should See

### With `--verbose` flag:
- 📬 Every MQTT message received
- 💓 Keepalive messages every 30 seconds
- 📊 Stats updates
- 🎯 Detection events
- Message payload sizes

### Without `--verbose` (default):
- Only 🎯 detection events (new/update/end)
- Clean, focused output

## Next Steps

Once you confirm the CLI works:
1. Set up camera(s) in Frigate config
2. Configure detection zones
3. Adjust confidence thresholds
4. Use CLI to monitor real-time detections
