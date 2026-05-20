# thoon

Two layers of code that live inside this fork:

## `thoon-*` — generic primitives

Standalone Rust crates with PyO3 bindings. Apache-2.0. Usable by any
Python LLM-agent framework, not just Hermes. Publishable independently
to crates.io and PyPI.

| Crate           | Status      | Description                              |
| --------------- | ----------- | ---------------------------------------- |
| `thoon-core`    | shared      | Error type + protocol primitives         |
| `thoon-toolreg` | hello-world | Concurrent tool registry primitive       |
| `thoon-fileops` | placeholder | Fast regex search / mmap I/O / patch     |
| `thoon-sqlite`  | placeholder | SQLite + FTS5 helpers                    |

## `hermes-thoon-*` — Hermes plugin glue

Pure-Python packages that adapt a `thoon-*` primitive to a specific
Hermes upstream contract and register via the `hermes_agent.plugins`
entry-point group. Each one is small enough to be cherry-picked into
NousResearch/hermes-agent on its own.

| Package                   | Phase | Wraps             | Status      |
| ------------------------- | :---: | ----------------- | ----------- |
| `hermes-thoon-toolreg`    |   1   | `thoon-toolreg`   | hello-world |
| `hermes-thoon-fileops`    |   2   | `thoon-fileops`   | placeholder |
| `hermes-thoon-sessiondb`  |   3   | `thoon-sqlite`    | placeholder |
| `hermes-thoon-msgproc`    |   4   | (none — deferred) | placeholder |

See [`../PLAN.md`](../PLAN.md) for the full design.

## Phase 1 quickstart

```bash
cd thoon-toolreg
maturin develop
python -c "import thoon_toolreg; print(thoon_toolreg.version())"
```
