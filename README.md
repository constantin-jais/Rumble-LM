# Rumble LM

**Layer:** Rumble — Product  
**Role:** sovereign learning and facilitation platform  
**Mission:** help groups learn, discuss, and decide from source-grounded AI content in reliable interactive sessions.

---

## Purpose

`rumble-lm` combines grounded knowledge work with live group engagement: documents become study material, quizzes, prompts, activities, summaries, and facilitated sessions.

The product outcome is not “chat with an LLM”; it is better learning and better collective understanding.

## Owns

- Learning/facilitation session UX for learners, facilitators, and participants.
- Source-grounded study content, activities, quizzes, and live interactions.
- Group engagement mechanics: participation, feedback, timing, scoring when relevant.
- Sovereign/BYO-key product experience and RGPD-aware operation.

## Does Not Own

- Generic model hosting or provider abstraction as infrastructure.
- Agentic orchestration internals: belongs to `cos-matic`.
- Raw ingestion/extraction: belongs to `wrench-loader`.
- Memory/storage/distribution primitives: belongs to Gear.
- A generic chatbot interface disconnected from learning outcomes.

## Allowed Dependencies

- Uses Bolt for orchestration when sessions need planning, generation, or agentic facilitation.
- Uses Wrench for document ingestion, source extraction, and validation.
- Uses Gear for memory, artifact integrity, provenance, and reproducible deployment paths.

## Product Vision Challenge

`rumble-lm` must be judged by learning outcomes, groundedness, session reliability, and group engagement — not by model novelty.
