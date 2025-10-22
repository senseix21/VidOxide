#!/bin/bash
# Stream webcam to MediaMTX for Frigate detection

echo "🎥 Starting webcam stream to rtsp://localhost:8554/demo"
echo "Press Ctrl+C to stop"
echo ""

ffmpeg -f v4l2 -framerate 30 -video_size 1280x720 -i /dev/video0 \
       -vf scale=-2:720,format=yuv420p -r 15 \
       -c:v libx264 -preset ultrafast -tune zerolatency -profile:v high -g 30 \
       -f rtsp -rtsp_transport tcp rtsp://localhost:8554/demo
