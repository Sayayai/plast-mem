# Plast Mem Development Context

## Project Overview

Plast Mem is an experimental llm memory layer for cyber waifu. The project is not yet stable, and limited documentation.

## How to Use This Documentation

When working on Plast Mem, follow this decision tree to navigate the codebase and make changes efficiently:

### Starting a Task

**First, understand what type of change you're making:**
- Is it a new feature? → Check docs/CHANGE_GUIDE.md for similar patterns
- Is it a refactor? → Check docs/ARCHITECTURE.md for design principles
- Is it a bug fix? → Read relevant crate README.md files

### Understanding Change Impact

**Before making changes, trace the impact:**

**Dependency flow pattern:**
```
API endpoint → Server handler → Core service → Entity/DB
     ↑              ↑              ↑
   HTTP           DTOs          Business Logic
```

**Steps:**
1. **Read the crate's README.md** to understand responsibilities
2. **Check docs/ARCHITECTURE.md** for layer dependencies
3. **Find all callers** with `grep -r "fn_name" crates/`
4. **Check trait implementations** in `plastmem_core/src/`
5. **Verify DB schema** in `plastmem_entities/src/`

### Quick Component Summary

- **plastmem**: Entry program - initializes tracing, DB, migrations, job storage, spawns worker and server
- **plastmem_core**: Core domain logic
  - `memory/episodic.rs` - hybrid retrieval with FSRS re-ranking
  - `memory/creation.rs` - episode generation and FSRS initialization
  - `message_queue/` - queue operations, segmentation rules, boundary detection, pending reviews
- **plastmem_migration**: Database table migrations
- **plastmem_entities**: Database table entities (Sea-ORM)
- **plastmem_ai**: AI SDK wrapper - embeddings, cosine similarity, text generation, structured output
- **plastmem_shared**: Reusable utilities (env, error)
- **plastmem_worker**: Background tasks worker
  - `event_segmentation.rs` - job dispatch
  - `memory_review.rs` - LLM-based review and FSRS update
- **plastmem_server**: HTTP server and API handlers

## Key Runtime Flows

- **Memory creation**: `crates/server/src/api/add_message.rs` → `MessageQueue::push` → `EventSegmentationJob` → dual-channel boundary detection → episode generation (LLM structured output: title + summary) → `EpisodicMemory` with surprise-based FSRS stability boost
- **Memory retrieval**: `crates/server/src/api/retrieve_memory.rs` → `EpisodicMemory::retrieve` (BM25 + vector RRF × FSRS retrievability) → records pending review in `MessageQueue`
- **FSRS review update**: segmentation triggers `MemoryReviewJob` when pending reviews exist → LLM evaluates relevance (Again/Hard/Good/Easy) → FSRS parameter update in `crates/worker/src/jobs/memory_review.rs`

## Context Files

Load these additional context files when working on specific areas:

- `docs/ARCHITECTURE.md` - System-wide architecture and design principles
- `docs/ENVIRONMENT.md` - Environment variables and configuration
- `docs/CHANGE_GUIDE.md` - Step-by-step guides for common changes
- `docs/architecture/fsrs.md` - FSRS algorithm, parameters, and memory scheduling
- `crates/core/README.md` - Core domain logic and memory operations
- `crates/ai/README.md` - AI/LLM integration, embeddings, and structured output
- `crates/server/README.md` - HTTP API and handlers
- `crates/worker/README.md` - Background job processing

## Implementation Strategy

When implementing new features:

1. **Start with types** - Define structs/enums in `plastmem_entities` or `plastmem_core`
2. **Add core logic** - Implement business logic in `plastmem_core`
3. **Wire up API** - Add HTTP handlers in `plastmem_server`
4. **Add background jobs** - If needed, create job handlers in `plastmem_worker`

**Incremental Development**: Make small, testable changes. The codebase uses compile-time checks extensively—use `cargo check` frequently.

## Testing Conventions

- **Unit tests**: Add to `crates/<name>/src/` with `#[cfg(test)]` modules
- **Integration tests**: Add to `crates/<name>/tests/` or workspace `tests/`
- **Database tests**: Use `#[tokio::test]` with test database setup
- **AI mocking**: Tests should mock LLM calls; use fixtures for embedding vectors

## Development Notes

- **FSRS is central**: Most memory operations involve FSRS parameters (stability, difficulty, retrievability)
- **Dual-channel detection**: Event segmentation uses both statistical and LLM-based boundary detection
- **Queue-based architecture**: Messages flow through queues; operations are often async
- **LLM costs matter**: AI calls are expensive; the system uses embeddings for first-stage retrieval

## File Reference

| File | Purpose |
|------|---------|
| `docs/ARCHITECTURE.md` | System-wide architecture and design principles |
| `docs/architecture/fsrs.md` | FSRS algorithm and memory scheduling |
| `crates/core/src/memory/episodic.rs` | Memory retrieval with hybrid ranking |
| `crates/core/src/memory/creation.rs` | Episode generation logic |
| `crates/core/src/message_queue/` | Queue operations and segmentation |
| `crates/worker/src/jobs/memory_review.rs` | LLM review and FSRS updates |
| `crates/worker/src/jobs/event_segmentation.rs` | Event segmentation job dispatch |
| `crates/server/src/api/add_message.rs` | Message ingestion API |
| `crates/server/src/api/retrieve_memory.rs` | Memory retrieval API |

## Build and Test Commands

```bash
# Basic commands
cargo build
cargo test
cargo check

# Check specific crate
cargo check -p plastmem_core
cargo test -p plastmem_core

# Run with logging
RUST_LOG=debug cargo run
```

## Remember

- The codebase follows predictable patterns. Most changes follow the same flow: API → Handler → Core → DB
- When in doubt about FSRS, check `docs/architecture/fsrs.md` and `crates/core/src/memory/episodic.rs`
- Memory operations are either: creation (with segmentation), retrieval (with review queue), or review (FSRS update)
- Prefer reading existing implementations over guessing patterns
