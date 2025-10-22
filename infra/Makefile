.PHONY: up down logs cli agent monitor fetch

up:
	cd compose && docker compose up -d --build

logs:
	cd compose && docker compose logs -f frigate mediamtx mosquitto

cli:
	cd apps/frigate-cli && cargo run --release -- --broker-host localhost --broker-port 1883 --topic frigate/events

agent:
	cd apps/frigate-agent && cargo run --release

monitor:
	cd apps/detection-monitor && cargo run --release

down:
	cd compose && docker compose down -v

fetch:
	bash infra/scripts/fetch_media.sh