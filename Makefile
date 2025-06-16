.PHONY: dev-up

dev-up:
	docker compose -f docker/docker-compose.dev.yml up -d