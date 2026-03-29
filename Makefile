verify:
	./scripts/verify.sh

migrate:
	cd backend && cargo sqlx migrate run

dev:
	@-pkill -f "target/debug/api" || true
	@-pkill -f "next-dev" || true
	(cd backend && cargo run -p api) & (cd frontend && npm run dev)
