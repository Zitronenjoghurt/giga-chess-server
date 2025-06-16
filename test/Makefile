.PHONY: up test clean

up:
	docker compose up -d

test:
	docker compose exec -T test-app cargo test -- --nocapture --test-threads=1

clean:
	docker compose down -v
	docker image prune -f