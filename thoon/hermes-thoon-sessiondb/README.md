# hermes-thoon-sessiondb

Hermes Agent plugin that swaps in a Rust-backed session storage layer,
backed by the [`thoon-sqlite`][primitive] primitive. Owns Hermes'
session schema and FTS5 layout.

[primitive]: ../thoon-sqlite/

## Status

Phase 3 placeholder.

## Install (when Phase 3 ships)

```bash
pip install thoon-sqlite hermes-thoon-sessiondb
```

```yaml
# ~/.hermes/config.yaml
plugins:
  enabled: [hermes-thoon-sessiondb]
```

## Usage

This package is a plugin, not a library. Once installed and enabled,
it activates automatically when Hermes starts.

## License

Apache-2.0.
