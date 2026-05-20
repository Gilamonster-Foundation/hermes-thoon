# PLAN — Hermes Agent Rust Acceleration via PyO3

This branch (`thoon`) tracks the design and rollout of Rust-accelerated
hot paths for the Hermes Agent codebase via PyO3 + maturin.

The Python tree in this repository remains a fork of
[NousResearch/hermes-agent](https://github.com/NousResearch/hermes-agent).
The Rust workspace lives under `hermes-rust/` and is feature-gated so that
the existing Python implementations remain authoritative and fully usable
when Rust is unavailable.

## Goal

3–10× throughput on the agent's hot paths while preserving 100% API
compatibility with the upstream Python interfaces.

## Priority Order

1. **`hermes-toolreg`** — tool registry, schema generation, dispatch.
   Touched on every tool call; the biggest UX win.
2. **`hermes-fileops`** — file read / write / search / patch.
   Replaces shell-outs to `rg` / `grep` / `find`.
3. **`hermes-sessiondb`** — SQLite + FTS5 session store.
   Write throughput and full-text search.
4. **`hermes-msgproc`** — message sanitization, repair, budgeting.
   **Deferred.** Smallest per-call cost; revisit after the first three ship.

## Crate Layout

```
hermes-rust/
├── Cargo.toml                  # Workspace root
├── hermes-core/                # Shared error mapping + types
├── hermes-toolreg/             # Phase 1 — registry & dispatch
├── hermes-fileops/             # Phase 2 — file operations
├── hermes-sessiondb/           # Phase 3 — session DB
└── hermes-msgproc/             # Phase 4 — deferred
```

Each crate compiles to its own Python extension module via PyO3, and ships
its own `pyproject.toml` so it can be built and released independently
with `maturin`.

## Python Hot Paths Targeted

| File                       | Lines | Phase | Notes                                       |
| -------------------------- | ----- | ----- | ------------------------------------------- |
| `tools/registry.py`        |   589 | 1     | Tool registry, schema validation            |
| `model_tools.py`           |   923 | 1     | Tool discovery, dispatch, schema generation |
| `tools/file_operations.py` |  1825 | 2     | File read/write/search/patch                |
| `hermes_state.py`          |  3238 | 3     | SQLite session storage with FTS5            |
| `run_agent.py`             |  4123 | 4     | Message processing hot path                 |

Line counts are from `main` at branch time.

## Build & Compatibility Strategy

- **Maturin + PyO3.** PEP 517/518 compliant; standard in the PyO3 ecosystem.
- **MSRV: Rust 1.75+.** Aligned with current PyO3 stable.
- **Feature flags.** `rust-toolreg`, `rust-fileops`, `rust-sessiondb`,
  `rust-msgproc` — enabled incrementally.
- **Python fallback.** `HERMES_RUST=0` env var forces the pure-Python
  implementations; this is the CI matrix for confirming behavioural parity.
- **GIL.** Release in long-running Rust ops (search, DB writes), acquire
  for Python callbacks.
- **Errors.** PyO3 `PyErr` maps to Python exceptions; `hermes-core`
  defines a shared `HermesError` enum that lowers to the relevant Python
  exception subclass.

## Phases

### Phase 1 — `hermes-toolreg` (target: weeks 1–2)

- `ToolRegistry` — concurrent hash map via `dashmap`.
- `ToolSchema` — JSON Schema generation via `schemars`.
- `ToolDispatcher` — direct function-pointer call, no Python reflection
  in the steady-state path.
- `ArgumentValidator` — JSON Schema validation via `jsonschema`.
- Replace: `tools/registry.py::ToolRegistry`,
  `model_tools.py::get_tool_definitions`, `model_tools.py::handle_function_call`.
- Benchmark targets: 10× faster tool discovery, 2× faster dispatch.

### Phase 2 — `hermes-fileops` (target: weeks 3–4)

- `search_content` — regex search via `grep-regex` + `ignore` crates
  (replaces `rg` shell-out).
- `search_files` — glob search via `globset` (replaces `find` shell-out).
- `read_file` — mmap-backed reading with offset / limit.
- `write_file` — atomic write with optional backup.
- `patch_file` — unified diff application via `diffy`.
- Benchmark targets: 5× faster search, 3× faster I/O.

### Phase 3 — `hermes-sessiondb` (target: weeks 5–6)

- Connection pool via `sqlx` (async) and `rusqlite` (sync).
- FTS5 virtual-table tuning and prepared-statement caching.
- WAL mode with safe fallback.
- Batch inserts for message history.
- Benchmark targets: 3× faster writes, 5× faster FTS5 search.

### Phase 4 — `hermes-msgproc` (deferred)

- `sanitize_surrogates` — SIMD UTF-8 validation.
- `repair_message_sequence` — state machine for role alternation.
- `sanitize_tool_call_arguments` — JSON repair via `serde_json`.
- `IterationBudget` — lock-free atomic counters.
- Benchmark target: 2× faster per-iteration overhead.
- Status: **deferred** — revisited only after Phases 1–3 ship and we have
  real benchmark data confirming msgproc is the next bottleneck.

## Build Commands

```bash
# Phase 1 — just toolreg
cd hermes-rust/hermes-toolreg && maturin develop

# All available crates once they exist
cd hermes-rust && maturin develop --features all

# Release wheels
cd hermes-rust/hermes-toolreg && maturin build --release
```

## CI Strategy

- `HERMES_RUST=1` — runs the test suite against the Rust modules.
- `HERMES_RUST=0` — runs the same suite against pure Python.
- Both must pass on every PR. Behavioural parity is gating.

## Risk & Mitigation

| Risk                               | Mitigation                                                  |
| ---------------------------------- | ----------------------------------------------------------- |
| Rust toolchain unavailable on host | Pure Python fallback via `HERMES_RUST=0`                    |
| PyO3 version drift                 | Pin `pyo3` in `Cargo.toml`; test matrix across Python 3.10–3.12 |
| Memory safety bugs                 | `#![forbid(unsafe_code)]` except at the explicit FFI boundary |
| Wheel size growth                  | Each crate is independently buildable / releasable          |
| Behavioural divergence             | CI runs the same suite with `HERMES_RUST=0` and `HERMES_RUST=1` |

## What's in this Initial Commit

- This document.
- The `hermes-rust/` workspace skeleton.
- A buildable `hermes-toolreg` PyO3 stub exposing `version()` so that
  `maturin develop` from `hermes-rust/hermes-toolreg/` produces a working
  extension. This proves the toolchain end-to-end before real work lands.
- Empty `hermes-core`, `hermes-fileops`, `hermes-sessiondb`,
  `hermes-msgproc` crates that compile but expose nothing yet.

Real implementations land in follow-up PRs against `thoon`.
