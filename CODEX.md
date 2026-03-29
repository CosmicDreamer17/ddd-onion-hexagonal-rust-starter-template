# Codex / OpenAI Onboarding

**CRITICAL**: Read [AI.md](./AI.md) for the full architectural mandates and identity rules before modifying this repository.

## 🛠 Project Context
- **Architecture**: Hexagonal + DDD Monorepo
- **Primary Stack**: Rust (Backend), Next.js (Frontend)
- **Verification**: Run `make verify` to ensure shippability.

## 📡 Key Commands
- **Build**: `cd backend && cargo build`
- **Test**: `cd backend && cargo test --workspace`
- **Admin CLI**: `make admin ARGS="health"`
