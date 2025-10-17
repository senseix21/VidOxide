# video-intel Makefile - delegates to infra/Makefile
.PHONY: up down logs cli agent fetch help

up:
	@$(MAKE) -f infra/Makefile up

down:
	@$(MAKE) -f infra/Makefile down

logs:
	@$(MAKE) -f infra/Makefile logs

cli:
	@$(MAKE) -f infra/Makefile cli

agent:
	@$(MAKE) -f infra/Makefile agent

fetch:
	@$(MAKE) -f infra/Makefile fetch

help:
	@echo "video-intel - FFmpeg → MediaMTX → Frigate → MQTT → Rust"
	@echo ""
	@echo "Usage:"
	@echo "  make fetch   - download sample demo.mp4"
	@echo "  make up      - start all services (builds containers)"
	@echo "  make logs    - tail Frigate/MediaMTX/Mosquitto logs"
	@echo "  make cli     - run frigate-cli locally"
	@echo "  make agent   - run frigate-agent locally"
	@echo "  make down    - stop all services & remove volumes"
	@echo ""
	@echo "Access:"
	@echo "  Frigate UI:  http://localhost:5000"
	@echo "  RTSP stream: rtsp://localhost:8554/demo"
	@echo "  Agent API:   http://localhost:8080/healthz"
