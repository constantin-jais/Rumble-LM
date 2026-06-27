# Presto-Matic

A sovereign, self-hostable collaborative learning platform — **NotebookLM × Kahoot**:
auto-generated, _source-grounded_ study content (quiz, flashcards, mind maps,
summaries) delivered in **real-time collaborative sessions** (200+ participants).

- **Sovereign / BYO** — self-host on your own infrastructure with your own AI keys.
  Defaults to Clever Cloud + Clever AI (EU, RGPD).
- **Grounded** — every generated item is traceable to its source, and verified by
  an agentic grounding checker (the wedge: trust).
- **Live** — host a session, participants join by link, answer grounded quizzes,
  watch a live leaderboard and a real-time comprehension heatmap.

> Status: `v0` — scaffold. The first milestone is a live-session **tracer-bullet**
> (host → Biscuit join link → 200 participants → one grounded question →
> real-time aggregation → leaderboard, load-tested). See `docs/specs/`.

## Workspace

- `crates/core` — shared Rust client/protocol core (→ native via UniFFI, → wasm for web).
- `crates/server` — the backend (axum; WebSocket session engine, RAG, generation).

## Stack

Rust · axum / tokio · PostgreSQL + pgvector · Cellar (S3) · Redis / Materia KV ·
Pulsar · Biscuit auth (+ OIDC / Keycloak) · OpenAI-compatible AI (Clever AI default).

## License

MIT © Constantin Jais
