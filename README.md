# AI-Autonomous Rust + Next.js Monorepo Starter

A high-performance, strictly bounded Monorepo template optimized for AI-agent maintenance.

## 🚀 Quick Start (Create Your Own)

### Path A: Using GitHub CLI (Recommended)
Bootstrap a new repo from this template:
```bash
gh repo create my-new-app --template CosmicDreamer17/ddd-onion-hexagonal-rust-starter-template --public --clone
```

### Path B: Using `degit` (No GitHub CLI)
```bash
npx degit CosmicDreamer17/ddd-onion-hexagonal-rust-starter-template my-new-app
```

---

## 🏗 Architecture (Hexagonal + DDD)

### Backend (Rust Workspace in `/backend`)
- **`crates/domain`**: Pure business logic. Zero dependencies.
- **`crates/application`**: Ports (Traits) and Use Cases.
- **`crates/infra`**: SQLx + SQLite adapters + Migrations (`/migrations`).
- **`crates/api`**: Axum server + CORS + Tracing (Web Adapter).
- **`crates/cli`**: Clap-based CLI tool (Terminal Adapter).

### Frontend (Next.js App in `/frontend`)
- **Types**: Zero-drift bindings generated via `ts-rs` into `/frontend/types/generated/`.

## 📡 Operations

### 1. Development (Local)
```bash
make dev
```
- **Frontend (Next.js)**: [http://localhost:3000](http://localhost:3000)
- **Backend (Axum)**: [http://localhost:3001](http://localhost:3001)

### 2. Admin CLI (Direct Use-Case Execution)
Interact with the backend directly from the terminal without HTTP overhead:
```bash
make admin ARGS="register --email user@example.com --username admin"
make admin ARGS="health"
```

### 3. Verification (CI/CD)
```bash
make verify
```
Runs architectural leak checks, formatting, clippy, and workspace tests.

### 4. Sync Types
```bash
./scripts/export-types.sh
```
Triggers `ts-rs` export and moves bindings to the frontend.

## 🤖 Instructions for AI Agents
This repository is optimized for autonomous maintenance. 
- Refer to `GEMINI.md` or `CLAUDE.md` for specific architectural constraints.
- Always use backend-driven identity generation (UUID).
- Maintain Hexagonal isolation: `domain` -> `application` -> `infra` -> `api`/`cli`.
- **Pro-tip**: Use the `make admin` CLI adapter for quick use-case testing and scripting!

---

## 🛠 Provenance
This repository was **autonomously engineered and verified** by **Gemini CLI** (Gemini 2.0 Flash & Pro Experimental) on **Saturday, March 28, 2026**. See [PROVENANCE.md](./PROVENANCE.md) for full metadata.
