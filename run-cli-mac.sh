#!/bin/bash
# macOS CLI Runner - Automatically finds MQTT container IP

echo "🔍 Finding MQTT container IP..."
MQTT_IP=$(docker inspect vi_mosquitto -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' 2>/dev/null)

if [ -z "$MQTT_IP" ]; then
    echo "❌ Cannot find MQTT container IP"
    echo ""
    echo "Troubleshooting:"
    echo "1. Is vi_mosquitto running?"
    echo "   docker ps | grep mosquitto"
    echo ""
    echo "2. Try starting services:"
    echo "   make up"
    exit 1
fi

echo "✅ Found MQTT at: $MQTT_IP:1883"
echo "🔌 Starting Frigate CLI..."
echo ""

# Build if needed
if [ ! -f "./target/release/frigate-cli" ]; then
    echo "📦 Building CLI..."
    cargo build --release --bin frigate-cli
fi

# Run CLI with container IP
./target/release/frigate-cli --broker-host "$MQTT_IP" --broker-port 1883 --topic "frigate/events" "$@"
