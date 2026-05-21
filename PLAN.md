# PLAN — Hermes Agent Rust Acceleration via PyO3

This branch (`thoon`) tracks the design and rollout of Rust-accelerated
hot paths for the Hermes Agent codebase via PyO3 + maturin, shipped as
optional plugins that the existing Hermes plugin system discovers.

The Python tree in this repository remains a fork of
[NousResearch/hermes-agent](https://github.com/NousResearch/hermes-agent).
All new work lives under `thoon/` and is structured so that upstream
can cherry-pick any individual crate or plugin on its own schedule.

## Goal

3–10× throughput on the agent's hot paths while preserving 100% API
compatibility with the upstream Python interfaces.

## Two-Layer Architecture

| Layer            | Brand          | What it is                                       | Audience              |
| ---------------- | -------------- | ------------------------------------------------ | --------------------- |
| **`thoon-*`**    | `thoon`        | Generic Rust primitives with PyO3 bindings       | Any Python LLM agent  |
| **`hermes-thoon-*`** | `hermes-thoon` | Hermes plugin glue adapting a `thoon-*` primitive | Hermes specifically  |

The `thoon-*` crates are framework-agnostic and publishable independently
to crates.io and PyPI. The `hermes-thoon-*` packages are small Python
shims that register via Hermes' existing `hermes_agent.plugins`
entry-point group and adapt a `thoon-*` primitive to a specific Hermes
contract.

### Why two layers

1. **Decoupling.** Generic primitives can be adopted by other agents
   (gilabot, drake-pool's hermes worker, other Python LLM frameworks)
   without taking a Hermes dependency.
2. **Cherry-pick clarity.** The Hermes-facing surface is a small Python
   plugin; the Rust complexity lives behind a framework-neutral library
   boundary. Upstream sees "an optional plugin that registers a faster
   ToolRegistry implementation," not a fork-wide Rust intrusion.
3. **Reviewability.** Hermes maintainers reviewing a cherry-pick can
   focus on the plugin glue (50–100 lines of Python) and trust the
   primitive's behaviour from its own test suite.

## Contribution Posture

`hermes-thoon` is **not a long-term divergent fork.** Every package in
`thoon/` is designed so that upstream NousResearch/hermes-agent can
cherry-pick the work on their own schedule, with no obligation to take
it.

That posture imposes hard discipline on how commits are shaped:

1. **Pure additions when possible.** New work goes under `thoon/` and
   ships as standalone pip-installable packages. The upstream Python
   tree stays untouched.
2. **No new flag system.** We reuse Hermes' existing
   `plugins.enabled` / `plugins.disabled` config knobs. Users
   enable / disable a `thoon-*` accelerator by installing or
   uninstalling its `hermes-thoon-*` plugin, or by listing it in
   their `config.yaml`.
3. **One-line wire-ins if needed at all.** If a Hermes Python file
   must change to consume an accelerator, the change is a **single
   `invoke_hook(...)` call** at the slot point, with the existing
   in-file implementation as the default. No reformatting, no
   refactoring, no behavioural drift bundled in.
4. **Atomic per-package commits.** One package = one cherry-pickable
   commit (or a small ordered series). No mixing packages.
5. **Match upstream conventions.** Commit message format, code style,
   license headers, line endings — follow whatever upstream uses.

The acceptance test for any change on `thoon` is: *could we open a PR
to NousResearch/hermes-agent with just this one commit, and would it
stand alone?* If no, the commit is malformed.

## Branch Model

- **`main`** — curated mirror of `NousResearch/hermes-agent`. Tracks
  the **most recent upstream release tag** matching `v2026.*`, not
  upstream's `main` head. Auto-synced daily by
  `.github/workflows/upstream-sync.yml`. **Never edited directly.**
- **`thoon`** — stable mainline of this fork. Periodically merges from
  `main` (or rebases in short-lived phase branches).
- **Phase branches** — short-lived (hours to days) off `thoon`, one
  per package's implementation work.

### Why "latest release tag" instead of "upstream head" or "latest green CI"

Upstream's `main` is frequently in a transiently broken state — for
example, `tests.yml` succeeded on only 1 of the last 50 runs on
`NousResearch/hermes-agent:main` at the time we set this up
(stale `test_all_seven_plugins_present_in_registry`, an
`_UpdateOutputStream` isinstance flake, and others). Tracking head
would inherit that breakage. But tracking "latest green CI" also
proved unreliable: upstream's tests.yml is too noisy to be a clean
gate, and even tagged releases sometimes have failing CI runs.

So the rule we settled on: **track upstream release tags**. Tags are
the only commits NousResearch explicitly declares stable — that's a
better signal than CI churn. The cost is a slightly larger lag
(0–7 days, since they cut releases roughly weekly).

Our origin/main may sit ahead of the latest tag for short periods —
the workflow simply waits for a newer tag and stays put until then.
We never auto-rewind.

A drift check (`.github/workflows/target-lib-check.yml`) runs after
each successful `upstream-sync` and opens an issue when any target
Python file in upstream has changed.

## Phases

### Phase 0 — Upstream Plugin Contract for Accelerators (prerequisite)

Before any Rust ships, we propose a small upstream PR that establishes
the plugin contract for accelerator-style plugins:

- Add `"accelerator"` plugin kind to `_VALID_PLUGIN_KINDS` in
  `hermes_cli/plugins.py` (or extend `backend` semantics).
- Add slot hooks at three call sites:
  - `tools/registry.py` — slot for `tool_registry`
  - `tools/file_operations.py` — slot for `file_operations`
  - `hermes_state.py` — slot for `session_db`
- Each slot uses `plugins.invoke_hook("provide_<slot>")` and takes the
  first non-`None` result; defaults to the existing in-file
  implementation when no plugin claims the slot.

This is itself useful to upstream even without our Rust — it opens
Hermes to alternative backends generically (Cython, Go via cgo,
alternate SQLite wrappers, anyone). Total surface: ~50 lines of
indirection across three Python files.

If upstream declines the slot hooks, we maintain the same indirection
on `thoon` and the cherry-pick discipline still applies, just to fewer
files.

### Phase 1 — `thoon-toolreg` + `hermes-thoon-toolreg`

`thoon-toolreg` (Rust):
- `ToolRegistry` primitive — concurrent hash map via `dashmap`.
- JSON Schema generation via `schemars`.
- Direct function-pointer dispatch.
- Argument validation via `jsonschema`.
- Framework-neutral API; no Hermes types.

`hermes-thoon-toolreg` (Python):
- Imports `thoon_toolreg`.
- `register(ctx)` adapts thoon's primitive to Hermes' ToolRegistry
  contract (`tools/registry.py::ToolRegistry`).
- Provides the `provide_tool_registry` hook.

Targets: 10× faster tool discovery, 2× faster dispatch.

### Phase 2 — `thoon-fileops` + `hermes-thoon-fileops`

`thoon-fileops` (Rust):
- `search_content` — regex search via `grep-regex` + `ignore`.
- `search_files` — glob search via `globset`.
- `read_file` — mmap-backed reading with offset / limit.
- `write_file` — atomic write with optional backup.
- `patch_file` — unified-diff application via `diffy`.

`hermes-thoon-fileops` (Python):
- Adapts thoon's primitives to `tools/file_operations.py`'s API.

Targets: 5× faster search, 3× faster I/O.

### Phase 3 — `thoon-sqlite` + `hermes-thoon-sessiondb`

`thoon-sqlite` (Rust):
- Connection pool (sync + async).
- Prepared-statement cache.
- FTS5 conveniences.
- WAL mode with safe fallback.
- Framework-neutral; no Hermes schema.

`hermes-thoon-sessiondb` (Python):
- Adapts thoon-sqlite to Hermes' `SessionDB` contract in
  `hermes_state.py`. Owns the Hermes-specific schema and FTS5 layout.

Targets: 3× faster writes, 5× faster FTS5 search.

### Phase 4 — `hermes-thoon-msgproc` (deferred, glue-only)

- `sanitize_surrogates` — SIMD UTF-8 validation.
- `repair_message_sequence` — state machine for role alternation.
- `sanitize_tool_call_arguments` — JSON repair via `serde_json`.
- `IterationBudget` — lock-free atomic counters.

No generic `thoon-*` primitive — the Hermes message shape is specific
enough that a generic layer adds no value. Revisit only after Phases
1–3 ship and benchmarks confirm message-loop overhead is the next
real bottleneck.

## Build & Compatibility Strategy

- **Maturin + PyO3** for `thoon-*` Rust crates.
- **Setuptools** for `hermes-thoon-*` Python packages.
- **MSRV: Rust 1.75+.**
- **Plugin enablement** through Hermes' existing `plugins.enabled` /
  `plugins.disabled` config knobs. No new env var system.
- **Fallback** is the existing pure-Python implementation when no
  `hermes-thoon-*` plugin claims a slot.

## CI Strategy

- **Behavioural parity** — same test suite must pass with and without
  each `hermes-thoon-*` plugin installed. Add per-plugin matrix entries
  as plugins ship.
- **Cherry-pick smoke test** — verify each `hermes-thoon-*` package
  can be `pip install`ed against an unmodified upstream checkout.

### Push hook governance

Per the workspace rule "hooks must mirror pipelines":

- `.githooks/pre-push` runs `cargo fmt --check`, `cargo clippy -- -D warnings`,
  and `cargo check --all` from `thoon/`.
- `.github/workflows/thoon-ci.yml` runs the same checks on push to
  `thoon` and on PR.
- Both files carry cross-reference comments so any edit to one signals
  the need to update the other.

Install the hook once per clone:

```bash
git config core.hooksPath .githooks
```

### Automation

- **`upstream-sync.yml`** (daily 06:00 UTC) — fetches upstream tags,
  picks the highest-versioned tag matching `v2026.*` reachable from
  `upstream/main`, verifies it is a descendant of our current
  `origin/main`, then fast-forwards. Quiet no-op when no newer tag
  exists. Aborts loudly if lineage checks fail.
- **`target-lib-check.yml`** (triggered by completed `upstream-sync`) —
  diffs target Python files and opens/updates a drift issue.
- **`thoon-ci.yml`** (push to `thoon` or PR touching `thoon/**`) —
  validates the `thoon/` Rust workspace and Python plugins.

## Workspace Layout

```
thoon/
├── Cargo.toml                       # Rust workspace (thoon-* members only)
├── README.md                        # layer overview + quickstart
├── .gitignore
│
├── thoon-core/                      # Rust: shared types + errors
├── thoon-toolreg/                   # Rust: tool registry primitive  (Phase 1, hello-world)
├── thoon-fileops/                   # Rust: file ops primitives      (Phase 2 placeholder)
├── thoon-sqlite/                    # Rust: sqlite + FTS5 helpers    (Phase 3 placeholder)
│
├── hermes-thoon-toolreg/            # Python plugin                  (Phase 1 stub)
├── hermes-thoon-fileops/            # Python plugin                  (Phase 2 placeholder)
├── hermes-thoon-sessiondb/          # Python plugin                  (Phase 3 placeholder)
└── hermes-thoon-msgproc/            # Python plugin (deferred)
```

## Phase 1 Quickstart

```bash
cd thoon/thoon-toolreg
maturin develop
python -c "import thoon_toolreg; print(thoon_toolreg.version())"
```

## Risk & Mitigation

| Risk                                | Mitigation                                                |
| ----------------------------------- | --------------------------------------------------------- |
| Upstream rejects Phase 0 hooks      | Maintain the indirection on `thoon` only; cherry-picks of `hermes-thoon-*` plugins still apply (with a small wire-in patch) |
| PyO3 version drift                  | Pin `pyo3` in workspace `Cargo.toml`; CI matrix Python 3.10–3.12 |
| Memory safety bugs                  | `#![forbid(unsafe_code)]` except at the explicit FFI boundary |
| `thoon-*` API churn breaks plugins  | Each `thoon-*` crate has its own `0.x` semver track; `hermes-thoon-*` pins to compatible versions |
| Behavioural divergence              | Per-plugin parity test in CI                              |
