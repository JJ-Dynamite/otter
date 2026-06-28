.PHONY: dev build run stop clean lint test
dev:
	docker-compose up --build
build:
	docker-compose build
run:
	docker-compose up -d
stop:
	docker-compose down
clean:
	docker-compose down -v && rm -rf backend/target frontend/node_modules frontend/.next
lint:
	cd backend && cargo clippy -- -D warnings && cd ../frontend && npm run lint
test:
	cd backend && cargo test && cd ../frontend && npm test
