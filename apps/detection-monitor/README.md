# Detection Monitor

Monitors Frigate detection events and publishes "no detection" status messages every 2 seconds when no objects are detected.

## Features

- 🔍 Tracks active detections from Frigate
- 💤 Publishes idle status every 2 seconds when no objects detected
- ⏱️ Reports time since last detection
- 📡 Publishes to MQTT topic: `frigate/{camera}/detection_status`

## Status Messages

When no objects are detected:
```json
{
  "status": "idle",
  "camera": "demo",
  "message": "No objects detected",
  "timestamp": 1729563911,
  "last_detection": "15 seconds ago"
}
```

## Usage

```bash
# Default (localhost:1883, camera: demo)
make monitor

# Custom settings
BROKER_HOST=192.168.1.100 BROKER_PORT=1883 CAMERA_NAME=frontdoor cargo run --release
```

## Environment Variables

- `BROKER_HOST` - MQTT broker host (default: localhost)
- `BROKER_PORT` - MQTT broker port (default: 1883)
- `CAMERA_NAME` - Camera name (default: demo)

## Example Output

```
🔍 Detection Monitor starting...
📡 Subscribed to frigate/events
✅ Detection active: person (1729563911.123-abc)
🎯 Active detections: 1
✅ Detection active: cup (1729563913.456-def)
🎯 Active detections: 2
❌ Detection ended: person (1729563911.123-abc)
🎯 Active detections: 1
❌ Detection ended: cup (1729563913.456-def)
🔇 No detections - published status
🔇 No detections - published status
```

## Integration with CLI

The enhanced `frigate-cli` automatically displays these messages:

```bash
./target/release/frigate-cli --topic "frigate/#"
```

Output:
```
🎯 new: person detected on demo (confidence: 85.0%)
💤 demo - No objects detected (last: 5 seconds ago)
💤 demo - No objects detected (last: 7 seconds ago)
```
