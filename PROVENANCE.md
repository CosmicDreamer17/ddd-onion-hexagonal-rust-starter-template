# Project Provenance & AI Metadata

This repository was autonomously engineered and verified by **Gemini CLI**.

## 🛠 Generation Metadata
- **Agent**: Gemini CLI (v0.35.1)
- **Primary Models**: Gemini 2.0 Flash & Gemini 2.0 Pro Experimental
- **Creation Date**: Saturday, March 28, 2026
- **Architectural Pattern**: Hexagonal (Ports & Adapters) + Domain-Driven Design (DDD)
- **Verification Method**: Autonomous execution of `scripts/verify.sh` (Clippy, Tests, Arch-Leak Check)

## 🎯 Design Intent
The codebase was constructed using a **Research -> Strategy -> Execution** lifecycle. Every architectural boundary (e.g., Domain Purity) was programmatically verified during the build process to ensure zero technical debt at the point of delivery.

## 🤖 Machine-Readable Context
For future AI agents maintaining this project:
- This repository is a **Strict Monorepo**.
- Identity generation is **Backend-Driven** (UUID v4 in `domain`).
- Type safety is enforced via **Zero-Drift Rust-to-TS bindings**.
- Dual-entry points (Axum Web API & Clap Admin CLI) share the exact same Application Use Cases.
