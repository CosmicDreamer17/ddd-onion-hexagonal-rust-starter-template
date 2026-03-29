verify:
	./scripts/verify.sh

migrate:
	cd backend && cargo sqlx migrate run

dev:
	@lsof -ti:3000,3001 | xargs kill -9 || true
	(cd backend && cargo run -p api) & (cd frontend && npm run dev)
