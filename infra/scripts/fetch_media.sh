#!/usr/bin/env bash
set -euo pipefail
mkdir -p "$(dirname "$0")/../../media"
cd "$(dirname "$0")/../../media"
if [ ! -f demo.mp4 ]; then
  echo "Fetching sample demo.mp4 (small MP4 clip)…"
  curl -L -o demo.mp4 https://filesamples.com/samples/video/mp4/sample_640x360.mp4
  echo "Saved to media/demo.mp4"
else
  echo "media/demo.mp4 already exists"
fi