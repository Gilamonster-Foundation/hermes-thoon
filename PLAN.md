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

## Contribution Posture

`hermes-thoon` is **not a long-term divergent fork.** Every crate in
`hermes-rust/` is an *optional* Rust variation of an existing Hermes
package, designed so that upstream NousResearch/hermes-agent can cherry-pick
the work on their own schedule, with no obligation to take it.

That posture imposes hard discipline on how commits are shaped:

1. **Pure additions when possible.** New work goes under `hermes-rust/`
   and ships with its own Python compat wrapper. The upstream Python tree
   stays untouched.
2. **One-line wire-ins.** When a Python file in the upstream tree must
   change to consume the Rust accelerator, the change is a **single
   import-line swap** to the compat wrapper. No reformatting, no
   refactoring, no behavioural drift bundled in.
3. **Compat wrapper owns the fallback.** Each crate ships a Python module
   (e.g. `hermes_toolreg_compat`) that tries to import the Rust extension
   and otherwise re-exports the unchanged upstream implementation. The
   try / except / fallback logic lives in the wrapper, not in upstream's
   source.
4. **Atomic per-crate commits.** One crate's contribution = one
   cherry-pickable commit (or a small, ordered series). No mixing crates
   in a single commit.
5. **Match upstream conventions.** Commit message format, code style,
   license headers, line endings — follow whatever upstream uses, so
   their reviewers see no friction.

The acceptance test for any change on `thoon` is: *could we open a PR to
NousResearch/hermes-agent with just this one commit, and would it stand
alone?* If no, the commit is malformed and must be re-shaped.

## Branch Model

- **`main`** — pristine mirror of `upstream/main`. Auto-synced daily by
  `.github/workflows/upstream-sync.yml`. **Never edited directly.**
- **`thoon`** — stable mainline of this fork. Periodically merges from
  `main` (or rebases against it inside short-lived phase branches).
- **Phase branches** — short-lived (hours to days) feature branches off
  `thoon` for each crate's implementation work. Rebased onto `thoon`
  before opening their PR.

A drift check (`.github/workflows/target-lib-check.yml`) runs after each
successful `upstream-sync` and opens an issue whenever a target Python
file has changed in upstream — that's our weekly "look at the target lib"
signal, triggered by the actual sync event rather than wall-clock cadence.

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

### Automation

- **`upstream-sync.yml`** (daily, 06:00 UTC) — fast-forwards `origin/main`
  to `upstream/main`. Fails loudly if `main` has diverged (it shouldn't,
  since `main` is never edited directly).
- **`target-lib-check.yml`** (triggered by completed `upstream-sync`) —
  diffs the upstream Python files we plan to accelerate against `thoon`.
  Opens or updates an issue listing what changed and links the diff, so
  drift is visible before the next phase PR.

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
