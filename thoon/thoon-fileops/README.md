# thoon-fileops

Fast, framework-agnostic file-operation primitives for Python LLM-agent
frameworks. Built with PyO3.

## Status

Phase 2 placeholder. Real implementation lands once Phase 1
(`thoon-toolreg`) is wired into Hermes. Will provide:

- `search_content` — regex search via `grep-regex` + `ignore`
  (replaces `rg` / `grep` shell-outs)
- `search_files` — glob search via `globset` (replaces `find`)
- `read_file` — mmap-backed reading with offset / limit
- `write_file` — atomic write with optional backup
- `patch_file` — unified-diff application via `diffy`

## Build

```bash
cd thoon/thoon-fileops
cargo build
```

A `pyproject.toml` + maturin wiring is added in the Phase 2 PR.

## Usage (Python — planned)

```python
import thoon_fileops

# Replaces a shell-out to `rg "TODO" --type py`
hits = thoon_fileops.search_content(
    root=".",
    pattern=r"TODO",
    file_globs=["*.py"],
)
for h in hits:
    print(f"{h.path}:{h.line}: {h.text}")
```

## License

Apache-2.0.
