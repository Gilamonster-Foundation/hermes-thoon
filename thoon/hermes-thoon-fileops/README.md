# hermes-thoon-fileops

Hermes Agent plugin that swaps in Rust-backed file operations, backed
by the [`thoon-fileops`][primitive] primitive.

[primitive]: ../thoon-fileops/

## Status

Phase 2 placeholder. Real adapter wiring lands once `thoon-fileops`
exposes its search / read / write / patch primitives.

## Install (when Phase 2 ships)

```bash
pip install thoon-fileops hermes-thoon-fileops
```

```yaml
# ~/.hermes/config.yaml
plugins:
  enabled: [hermes-thoon-fileops]
```

## Usage

This package is a plugin, not a library. Once installed and enabled,
it activates automatically when Hermes starts.

## License

Apache-2.0.
