# Claude Code Onboarding

**CRITICAL**: Read [AI.md](./AI.md) for the full architectural mandates and identity rules before modifying this repository.

## 🛠 Project Context
- **Primary Language**: Rust (Backend), TypeScript (Frontend)
- **Architecture**: Hexagonal + DDD Monorepo
- **Verification**: `make verify`

## 📡 Key Commands
- **Build**: `cd backend && cargo build`
- **Test**: `cd backend && cargo test --workspace`
- **Admin CLI**: `make admin ARGS="health"`
