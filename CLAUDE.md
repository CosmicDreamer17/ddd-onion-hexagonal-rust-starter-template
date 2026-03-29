# AI-Autonomous Project Guidelines

## Mandates
1. ARCHITECTURE: Hexagonal + DDD.
2. DOMAIN PURITY: `backend/crates/domain` MUST have ZERO dependencies on other crates. Never import `infra` into `domain`.
3. ERROR HANDLING: All fallible operations must return a `DomainError` enum from `backend/crates/domain`.
4. WORKFLOW: Always run `make verify` before committing.
5. TYPE BRIDGE: All Rust structs intended for API exposure must derive `TS` from `ts-rs`.

## Rules of the Road
- Always use /plan mode before modifying the Domain layer.
- Never use raw Strings for domain IDs; use Newtypes.
- Architecture is verified via `scripts/verify.sh`.
