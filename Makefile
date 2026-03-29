verify:
	./scripts/verify.sh

migrate:
	cd backend && cargo sqlx migrate run

dev:
	(cd backend && cargo run -p api) & (cd frontend && npm run dev)
