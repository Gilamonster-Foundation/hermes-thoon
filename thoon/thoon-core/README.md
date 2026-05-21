# thoon-core

Shared error type and protocol primitives used by the rest of the
`thoon-*` workspace. Framework-agnostic; no Hermes-specific types.

## Status

Foundational crate. Reused by `thoon-toolreg`, `thoon-fileops`,
`thoon-sqlite`.

## Usage (Rust)

```rust
use thoon_core::{Result, ThoonError};

fn open_thing(path: &str) -> Result<Vec<u8>> {
    std::fs::read(path).map_err(|e| ThoonError::Io(e.to_string()))
}
```

No Python surface — `thoon-core` is a pure-Rust crate.

## License

Apache-2.0. See repo root.
