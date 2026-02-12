# Tool Result Format (TODO)

Format for `retrieve_memory` API response when used as a tool result for LLM consumption.

## Design Principles

1. **Token-efficient**: Minimize formatting overhead
2. **Hierarchical**: Summary first, details on demand
3. **Self-describing**: Metadata helps LLM prioritize

## Output Format

```markdown
## Memory 1 [rank: 1, score: 0.92, key moment]
**When:** 2 days ago
**Summary:** User switching careers from Python to Rust due to performance requirements at new job.

**Details:**
- user: "I've been doing Python for 5 years but my new team is all Rust"
- assistant: "That's a big shift. What prompted it?"
- user: "The trading system needs microsecond latency, Python can't cut it"
- user: "Also I need to learn it within 3 months or I'm screwed"

## Memory 2 [rank: 2, score: 0.85]
**When:** yesterday
**Summary:** User prefers dark mode interfaces and finds light mode straining.

## Memory 3 [rank: 3, score: 0.74]
**When:** last week
**Summary:** User mentioned living in Tokyo for 3 years before moving to Singapore.
```

## Detail Inclusion Rules

### Default Behavior

| Rank | Surprise Score | Include Details? | Rationale |
|------|----------------|------------------|-----------|
| 1-2 | ≥ 0.7 | ✅ | Top relevant + high surprise = key moment |
| 1-2 | < 0.7 | ❌ | Top relevant but routine information |
| 3-5 | any | ❌ | Context references, summaries suffice |

### Detail Level

```rust
pub enum DetailLevel {
    /// Smart allocation based on surprise (default)
    /// Ranks 1-2 with surprise ≥ 0.7 get details
    Auto,
    /// No details for any memory
    None,
    /// Only first memory gets details (if surprising)
    Low,
    /// All returned memories get full details
    High,
}

pub struct RetrieveMemory {
    pub query: String,
    /// Maximum number of memories to return
    #[serde(default = "default_limit")]
    pub limit: usize,  // default: 5
    /// Detail level for message inclusion
    #[serde(default)]
    pub detail: DetailLevel,  // default: Auto
}
```

| `detail` | Behavior |
|----------|----------|
| `"auto"` / omitted | Default: ranks 1-2 with `surprise ≥ 0.7` get details |
| `"none"` | No details for any memory |
| `"low"` | Only rank 1 gets details (if surprising) |
| `"high"` | All returned memories get full details |

## Field Selection

### Included Fields

| Field | Purpose |
|-------|---------|
| `rank` | Position in results (1-5) |
| `score` | Relevance score (0.0-1.0), helps LLM judge priority |
| `key moment` | Flag when `surprise ≥ 0.7`, signals high-importance memory |
| `When` | Relative time (e.g., "2 days ago"), easier for LLM than timestamps |
| `Summary` | The `content` field from EpisodicMemory |
| `Details` | Full `messages` array, only for qualifying memories |

### Excluded Fields

| Field | Exclusion Reason |
|-------|-----------------|
| `id`, `conversation_id` | Internal identifiers, not useful for LLM reasoning |
| `embedding` | Vector data, meaningless in text context |
| `stability`, `difficulty` | Internal FSRS parameters, noisy for LLM |
| `messages` (for non-detailed) | Redundant with summary, consumes tokens |
| Exact timestamps | Relative time is more natural for LLM |

## Format Rationale

### Why Markdown over JSON/XML?

| Aspect | Markdown | JSON | XML |
|--------|----------|------|-----|
| Token overhead | Low (~20 tokens) | Medium (~30 tokens) | High (~35 tokens) |
| Human readability | Good | Poor | Poor |
| LLM familiarity | Very high | High | Medium |
| Nesting overhead | None | Braces/quotes | Tag pairs |

### Why not include details for all?

- **Token budget**: 5 memories × full conversation = potentially thousands of tokens
- **Signal-to-noise**: Most relevant memories are often routine; high-surprise memories contain the "aha" insights
- **LLM attention**: "key moment" label naturally guides focus

## Example Scenarios

### Casual Query
```
POST /api/v0/retrieve_memory
{ "query": "how are you" }

// Returns: 5 summaries, 0 details (no memories qualify as key moments)
```

### Deep Context Needed
```
POST /api/v0/retrieve_memory
{ "query": "what should I learn next", "detail": "high" }

// Returns: all memories with full details
```

### Explicit Summary Only
```
POST /api/v0/retrieve_memory
{ "query": "remind me what we discussed", "detail": "none" }

// Returns: 5 summaries only, no details for any
```

### Minimal Detail
```
POST /api/v0/retrieve_memory
{ "query": "quick reminder", "detail": "low" }

// Returns: only rank 1 gets details (if surprising)
```
