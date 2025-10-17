# video-intel
One-command stack: FFmpeg → MediaMTX (RTSP) → Frigate (detect) → MQTT → Rust CLI notifier.

## Quickstart
```bash
make fetch   # optional: download small sample clip
make up      # builds & starts everything
make logs    # tail Frigate/MediaMTX/Mosquitto
```
Open Frigate UI: http://localhost:5000

The CLI notifier is also built as a container; its logs appear in:
```bash
cd compose && docker compose logs -f vi_frigate_cli
```

## Local dev (Rust on host)
```bash
make cli     # runs apps/frigate-cli against localhost:1883
```

## Stop
```bash
make down
```