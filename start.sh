#!/bin/bash
# Complete startup sequence for video-intel

set -e

echo "🎥 VIDEO INTELLIGENCE - Complete Startup"
echo "========================================"
echo ""

# Step 1: Start Docker stack
echo "1️⃣ Starting Docker services..."
cd /Users/abuhamzah/Dev/rust/projects/erik/video-intel
make up > /dev/null 2>&1
echo "   ✅ Mosquitto, MediaMTX, Frigate started"
echo ""

# Step 2: Wait for services
echo "2️⃣ Waiting for services to initialize (10s)..."
sleep 10
echo "   ✅ Services ready"
echo ""

# Step 3: Start webcam stream
echo "3️⃣ Starting webcam stream..."
echo "   Run this command in a SEPARATE terminal:"
echo ""
echo "   ffmpeg -f avfoundation -framerate 30 -pixel_format uyvy422 -i \"0:0\" \\"
echo "     -vf scale=-2:720 -r 15 \\"
echo "     -c:v h264_videotoolbox -profile:v high -g 30 -tune zerolatency \\"
echo "     -f rtsp -rtsp_transport tcp rtsp://localhost:8554/demo"
echo ""
echo "   Keep that terminal running!"
echo ""

# Step 4: Wait for stream
echo "4️⃣ After starting FFmpeg, wait 10 seconds, then:"
echo "   - Restart Frigate: docker restart vi_frigate"
echo "   - Wait 15 seconds"
echo ""

# Step 5: Test
echo "5️⃣ Then test:"
echo "   - Open http://localhost:5000 (should show video)"
echo "   - Run: make cli (should show events when you move)"
echo ""

echo "📝 Summary:"
echo "   Terminal 1: This script (done)"
echo "   Terminal 2: FFmpeg webcam stream (start manually above)"
echo "   Terminal 3: make cli (after Frigate restart)"
echo ""
echo "🎯 Ready to start!"
