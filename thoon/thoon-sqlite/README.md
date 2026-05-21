# thoon-sqlite

Framework-agnostic SQLite + FTS5 helpers for Python LLM-agent
frameworks. Built with PyO3.

## Status

Phase 3 placeholder. Real implementation lands after Phases 1–2. Will
provide:

- Connection pool (sync + async)
- Prepared-statement cache
- FTS5 virtual-table conveniences
- WAL mode with safe fallback
- Batch insert helpers

This crate owns no schema. Hermes' session schema lives in
[`hermes-thoon-sessiondb`][hp].

[hp]: ../hermes-thoon-sessiondb/

## Build

```bash
cd thoon/thoon-sqlite
cargo build
```

## Usage (Python — planned)

```python
import thoon_sqlite

pool = thoon_sqlite.Pool("messages.db", wal=True)
with pool.connection() as conn:
    conn.execute("INSERT INTO messages(role, content) VALUES(?, ?)", ("user", "hi"))
```

## License

Apache-2.0.
