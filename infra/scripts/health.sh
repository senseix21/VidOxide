#!/usr/bin/env bash
set -euo pipefail
curl -sf http://localhost:5000 >/dev/null && echo "Frigate OK" || { echo "Frigate UI not up"; exit 1; }