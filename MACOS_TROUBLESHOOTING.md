# macOS Troubleshooting Guide

## Issue: CLI Connects but No Events Received

### Problem
On macOS, the CLI connects to MQTT but doesn't receive detection events:
```
🔌 Connecting to MQTT broker at localhost:1883...
✅ Connected to MQTT broker
📡 Subscribed to topic 'frigate/events'
🔍 Listening for Frigate events...
[no events appear]
```

## Root Cause
Docker Desktop for Mac uses a VM, and `localhost:1883` from the host may not reach the containerized MQTT broker properly.

## Solutions

### Solution 1: Use Docker Host IP (Recommended)

**On macOS, use `host.docker.internal` or the container IP:**

```bash
# Find the MQTT container IP
docker inspect vi_mosquitto | grep IPAddress

# Run CLI with container IP
./target/release/frigate-cli --broker-host 172.18.0.3 --broker-port 1883 --topic frigate/events
```

Or use Docker's special hostname:
```bash
# This might work on newer Docker Desktop versions
./target/release/frigate-cli --broker-host host.docker.internal --broker-port 1883 --topic frigate/events
```

### Solution 2: Port Forward Verification

Check if port 1883 is actually accessible:

```bash
# Test MQTT connection
brew install mosquitto
mosquitto_sub -h localhost -p 1883 -t "frigate/#" -v
```

**If this works**, the CLI should work. If it doesn't, the issue is port forwarding.

### Solution 3: Run CLI in Docker

Create a simple wrapper to run the CLI inside Docker network:

```bash
docker run --rm -it --network compose_default \
  -v "$(pwd)/target/release/frigate-cli:/app/frigate-cli" \
  ubuntu:22.04 \
  /app/frigate-cli --broker-host vi_mosquitto --broker-port 1883 --topic frigate/events
```

### Solution 4: Fix Docker Compose Networking

Modify `compose/docker-compose.yml` to expose MQTT on host:

```yaml
services:
  mosquitto:
    ports:
      - "1883:1883"  # Make sure this line exists
```

Restart services:
```bash
cd compose && docker compose down && docker compose up -d
```

## Verification Steps

### 1. Check Docker Containers
```bash
docker ps | grep -E "(mosquitto|frigate)"
```

Should show all running.

### 2. Check Port Binding
```bash
docker port vi_mosquitto
```

Should show: `1883/tcp -> 0.0.0.0:1883`

### 3. Test MQTT Connectivity
```bash
# From macOS host
nc -zv localhost 1883
```

Should show: `Connection to localhost port 1883 [tcp/*] succeeded!`

### 4. Monitor MQTT Traffic
```bash
# Subscribe to all topics
docker exec vi_mosquitto mosquitto_sub -t "#" -v
```

Should show constant traffic from Frigate.

## Working Configuration for macOS

### Option A: Find Container IP and Use It

```bash
# Get MQTT container IP
MQTT_IP=$(docker inspect vi_mosquitto -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}')
echo "MQTT IP: $MQTT_IP"

# Run CLI with that IP
./target/release/frigate-cli --broker-host $MQTT_IP --broker-port 1883 --topic frigate/events
```

### Option B: Update Makefile for macOS

Add a macOS-specific target in `Makefile`:

```makefile
cli-mac:
	@MQTT_IP=$$(docker inspect vi_mosquitto -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}'); \
	cd apps/frigate-cli && cargo run --release -- --broker-host $$MQTT_IP --broker-port 1883 --topic frigate/events
```

Then run:
```bash
make cli-mac
```

## Quick Fix Script

Create `run-cli-mac.sh`:

```bash
#!/bin/bash
# Get MQTT container IP dynamically
MQTT_IP=$(docker inspect vi_mosquitto -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}')

if [ -z "$MQTT_IP" ]; then
    echo "❌ Cannot find MQTT container IP"
    echo "Is vi_mosquitto running?"
    exit 1
fi

echo "🔌 Connecting to MQTT at $MQTT_IP:1883"
./target/release/frigate-cli --broker-host "$MQTT_IP" --broker-port 1883 --topic "frigate/events"
```

Make executable and run:
```bash
chmod +x run-cli-mac.sh
./run-cli-mac.sh
```

## Debug: Raw MQTT Subscribe

If nothing works, test raw MQTT from inside the container:

```bash
# Enter the MQTT container
docker exec -it vi_mosquitto sh

# Subscribe to all topics
mosquitto_sub -t "#" -v

# You should see constant traffic from Frigate
```

If you see traffic here but not from host, it's definitely a networking issue.

## Expected Behavior After Fix

```bash
./run-cli-mac.sh
🔌 Connecting to MQTT at 172.18.0.3:1883
✅ Connected to MQTT broker
📡 Subscribed to topic 'frigate/events'
🔍 Listening for Frigate events...

🎯 new: cup detected on demo (confidence: 51.2%)
🎯 update: bed detected on demo (confidence: 68.4%)
```

## Still Not Working?

1. **Check Frigate is detecting**:
   ```bash
   curl http://localhost:5000/api/stats | jq '.cameras.demo.detection_fps'
   ```
   Should be > 0

2. **Check MQTT broker logs**:
   ```bash
   docker logs vi_mosquitto --tail 50
   ```

3. **Verify network**:
   ```bash
   docker network inspect compose_default
   ```

4. **Try with verbose mode**:
   ```bash
   ./target/release/frigate-cli --broker-host <IP> --broker-port 1883 --topic "frigate/#" --verbose
   ```

## Alternative: Detection Monitor

The detection monitor might work better as it can run in Docker:

```bash
# Run monitor in Docker network
docker run --rm -it --network compose_default \
  -e BROKER_HOST=vi_mosquitto \
  rust:latest bash -c "
    cd /tmp && 
    git clone https://github.com/senseix21/VidOxide.git &&
    cd VidOxide &&
    cargo run --release --bin detection-monitor
  "
```

---

**TL;DR for macOS:**
```bash
# Quick fix
MQTT_IP=$(docker inspect vi_mosquitto -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}')
./target/release/frigate-cli --broker-host $MQTT_IP --broker-port 1883 --topic frigate/events
```
