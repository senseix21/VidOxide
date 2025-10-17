#!/bin/bash
# Quick test script for video-intel

set -e

echo "🧪 Video Intelligence Stack - Quick Test"
echo "========================================"
echo ""

# Test 1: Docker
echo "📦 Test 1: Docker Services"
docker ps --filter "name=vi_" --format "{{.Names}}: {{.Status}}" || {
    echo "❌ Docker not running or services not started"
    echo "Run: make up"
    exit 1
}
echo "✅ Docker services OK"
echo ""

# Test 2: FFmpeg
echo "📹 Test 2: Webcam Stream"
if ps aux | grep "ffmpeg.*8554" | grep -v grep > /dev/null; then
    echo "✅ FFmpeg is streaming"
else
    echo "⚠️  FFmpeg not running"
    echo "Start with:"
    echo "  ffmpeg -f avfoundation -framerate 30 -pixel_format uyvy422 -i \"0:0\" \\"
    echo "    -vf scale=-2:720 -r 15 \\"
    echo "    -c:v h264_videotoolbox -profile:v high -g 30 -tune zerolatency \\"
    echo "    -f rtsp -rtsp_transport tcp rtsp://localhost:8554/demo"
fi
echo ""

# Test 3: MediaMTX
echo "📡 Test 3: RTSP Server"
if docker logs vi_mediamtx 2>&1 | grep -q "publishing to path"; then
    echo "✅ MediaMTX receiving stream"
else
    echo "⚠️  MediaMTX not receiving stream yet"
fi
echo ""

# Test 4: Frigate
echo "🎯 Test 4: Frigate Detection"
if docker logs vi_frigate 2>&1 | tail -50 | grep -q "Capture process started"; then
    echo "✅ Frigate capturing video"
else
    echo "⚠️  Frigate not capturing yet"
    echo "Try: docker restart vi_frigate"
fi
echo ""

# Test 5: MQTT
echo "📬 Test 5: MQTT Broker"
if nc -z localhost 1883 2>/dev/null; then
    echo "✅ MQTT broker accessible"
else
    echo "❌ MQTT broker not accessible"
fi
echo ""

# Summary
echo "📊 Summary"
echo "=========="
echo ""
echo "Next steps:"
echo "  1. Ensure FFmpeg is streaming (Test 2)"
echo "  2. Run: make cli"
echo "  3. Move in front of webcam"
echo "  4. Watch for detection events!"
echo ""
echo "Open Frigate UI: http://localhost:5000"
echo ""
