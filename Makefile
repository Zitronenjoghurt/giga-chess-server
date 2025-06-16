.PHONY: up shell dev-up dev-shell

up:
	docker compose -f docker/docker-compose.prod.yml up -d

shell:
	docker compose -f docker/docker-compose.prod.yml exec -it app /bin/bash

dev-up:
	docker compose -f docker/docker-compose.dev.yml up -d

dev-shell:
	docker compose -f docker/docker-compose.dev.yml exec -it app-dev /bin/bash