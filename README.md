# AI-Autonomous Rust + Next.js Monorepo Starter

A high-performance, strictly bounded Monorepo template optimized for AI-agent maintenance. Built with a Hexagonal/DDD Rust backend and a Next.js frontend.

## 🏗 Architecture

### Backend (Rust)
Located in `/backend`, organized as a Cargo Workspace:
- **`crates/domain`**: Pure business logic. Zero dependencies. Use Newtypes for IDs.
- **`crates/application`**: Ports (Traits) and Use Cases.
- **`crates/infra`**: SQLx + SQLite adapters. Handles migrations and physical storage.
- **`crates/api`**: Axum web server, DTOs, and CORS configuration.

### Frontend (Next.js)
Located in `/frontend`:
- **App Router**: Modern Next.js patterns.
- **Tailwind CSS**: Utility-first styling.
- **Zero-Drift Types**: TypeScript bindings are automatically generated from Rust structs via `ts-rs`.

## 🚀 Quick Start

### Prerequisites
- Rust (latest stable)
- Node.js & npm
- SQLx CLI (optional, for manual migrations)

### Setup & Run
1. **Install dependencies**:
   ```bash
   cd frontend && npm install
   ```
2. **Run Development Mode**:
   ```bash
   make dev
   ```
   This starts the Axum backend (Port 3000) and Next.js frontend (Port 3001) concurrently.

### 🛠 Tooling & Scripts
- `make verify`: Runs architectural checks, clippy, and tests.
- `make dev`: Concurrently runs backend and frontend with auto-cleanup.
- `./scripts/export-types.sh`: Manual trigger to sync Rust types to Frontend.

## 🤖 AI-Agent Protocols
This repository contains specific instructions for AI agents in `GEMINI.md` and `CLAUDE.md`. 
- **Domain Purity**: Never import `infra` into `domain`.
- **Type Safety**: Always use Newtypes for IDs; never raw Strings.
- **Verification**: `scripts/verify.sh` is the source of truth for completion.

## 📡 API Endpoints
- `POST /register`: Registers a new user. Expects `CreateUserCommand` JSON.
