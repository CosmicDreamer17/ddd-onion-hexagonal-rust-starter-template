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

> **🤖 Note for AI Agents**: If you are bootstrapping this project, refer to the [Agentic Bootstrap](./AI.md#🚀-agentic-bootstrap-creating-a-new-project) section in `AI.md` for mandatory initialization steps.

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

## 🤖 Agentic Maintenance
This repository is "AI-Autonomous Ready." It includes specialized markdown files in the root:
- **`AI.md` (Master)**: The single source of truth for architectural mandates and quality-of-life instructions for all AI agents (Gemini, Claude, Cursor/OpenAI).
- **Tool Entry Points**: `GEMINI.md`, `CLAUDE.md`, `CODEX.md`, and `.cursorrules` provide tool-specific onboarding while referencing the master rules.
- **Backend Identity**: Identity generation is backend-driven (UUID v4) to ensure integrity across agentic sessions.

## 🛠 Provenance
This repository was **autonomously engineered and verified** by **Gemini CLI** (Gemini 3 Flash Preview) on **Saturday, March 28, 2026**. See [PROVENANCE.md](./PROVENANCE.md) for full metadata.
