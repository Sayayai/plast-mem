# Plast Mem

Experimental Memory Layer designed for Waifu / Companion / Assistant

## About (from @kwaa)

This is my third attempt at a memory layer; the first two failed.

For this implementation, I've decided to build it publicly, hoping that interested people (like you, perhaps?) might join in.

At present, it is still far from being basically usable.

### Status

I plan to first write a minimal implementation referred on SimpleMem and Nemori, then build upon it with improvements.

During the `v0.0.x` stage, backward compatibility is not guaranteed at all.

## FAQ (from @kwaa)

### Why reinvent the wheel?

I want a memory layer that's easy to self-host, **not written in Python**, and not too rudimentary. I haven't found a suitable one yet, so I've decided to write my own.

### Is it related to Project AIRI's Alaya?

No, I'm currently developing this project on my own.

But I might draw inspiration from some of it - or I might not.

### Why use {{placeholder}}?

Let me answer them one by one:

- Rust
  - I previously used TypeScript, but ultimately found that I needed multithreading and background tasks. and I'm more familiar with apalis in Rust for this purpose.
  - I enjoy writing Rust code.
- ParadeDB (PostgreSQL + pg_search + pgvector)
  - I also considered LanceDB, but its website doesn't provide Rust examples for some parts.

## License

[MIT](LICENSE.md)
