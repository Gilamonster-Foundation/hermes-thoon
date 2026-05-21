# hermes-thoon-toolreg

Hermes Agent plugin that swaps in a Rust-backed ToolRegistry
implementation, backed by the framework-neutral
[`thoon-toolreg`][primitive] primitive.

[primitive]: ../thoon-toolreg/

## Status

Phase 1 stub. The plugin loads, discovers the Rust extension, and logs
its version. The actual `ToolRegistry` swap activates once the upstream
accelerator-slot hook lands.

## Install

```bash
pip install thoon-toolreg hermes-thoon-toolreg
```

Hermes' plugin manager (`hermes_cli/plugins.py`) discovers it via the
`hermes_agent.plugins` entry-point group. Enable it through Hermes'
existing config knobs:

```yaml
# ~/.hermes/config.yaml
plugins:
  enabled: [hermes-thoon-toolreg]
```

## Usage

This package is a plugin, not a library you import directly. Once
installed and enabled, it activates automatically when Hermes starts.

To verify it's loaded:

```bash
hermes plugins list | grep hermes-thoon-toolreg
```

Or programmatically:

```python
import hermes_thoon_toolreg

# Plugin version + the underlying Rust primitive version
print(hermes_thoon_toolreg.__version__)
print(hermes_thoon_toolreg._thoon_version())
```

## License

Apache-2.0.
