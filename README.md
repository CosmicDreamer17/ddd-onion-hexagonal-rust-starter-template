# AI-Autonomous Rust + Next.js Monorepo Starter

A high-performance, strictly bounded Monorepo template optimized for AI-agent maintenance.

## 🏗 Architecture (Hexagonal + DDD)

### Backend (Rust Workspace in `/backend`)
- **`crates/domain`**: Pure business logic. Zero dependencies.
- **`crates/application`**: Ports (Traits) and Use Cases.
- **`crates/infra`**: SQLx + SQLite adapters + Migrations (`/migrations`).
- **`crates/api`**: Axum server + CORS + Tracing.

### Frontend (Next.js App in `/frontend`)
- **Types**: Zero-drift bindings generated via `ts-rs` into `/frontend/types/generated/`.

## 🚀 Operations

### 1. Development (Local)
```bash
make dev
```
- **Frontend (Next.js)**: [http://localhost:3000](http://localhost:3000)
- **Backend (Axum)**: [http://localhost:3001](http://localhost:3001)

### 2. Verification (CI/CD)
```bash
make verify
```
Runs architectural leak checks, formatting, clippy, and workspace tests.

### 3. Sync Types
```bash
./scripts/export-types.sh
```
Triggers `ts-rs` export and moves bindings to the frontend.

## 🤖 Instructions for AI Agents
This repository is optimized for autonomous maintenance. 
- Refer to `GEMINI.md` or `CLAUDE.md` for specific architectural constraints.
- Always use backend-driven identity generation (UUID).
- Maintain Hexagonal isolation: `domain` -> `application` -> `infra` -> `api`.
