# Plast Mem Development Context

## Project Overview

Plast Mem is an experimental llm memory layer for cyber waifu. The project is not yet stable, and limited documentation.

## How to Use This Documentation

When working on Plast Mem, follow this decision tree to navigate the codebase and make changes efficiently:

### 1. Starting a Task

**First, understand what type of change you're making:**
- Is it a refactor/new feature? → Check docs/ARCHITECTURE.md for design principles
- Is it a bug fix? → Read relevant crate README.md files
- Is it about FSRS/retrieval/review? → Start with docs/architecture/fsrs.md and the current flow in `crates/core/src/memory/episodic.rs` and `crates/worker/src/jobs/memory_review.rs`

### 2. Understanding Change Impact

**Before making changes, trace the impact:**

1. **Read the crate's README.md** to understand responsibilities
2. **Check docs/ARCHITECTURE.md** for layer dependencies

### 3. Quick Component Summary

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

## Build and Test Commands

```bash
# Basic commands
cargo build
cargo test
cargo check
```
