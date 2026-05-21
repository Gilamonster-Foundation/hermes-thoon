# thoon-toolreg

Generic concurrent tool registry primitive for Python LLM-agent
frameworks. Built with PyO3.

This crate is **framework-neutral** — it provides the storage and
dispatch primitives any tool registry needs. Hermes-specific shape
(the `ToolRegistry` class, its method signatures, schema-emission
conventions) lives in the [`hermes-thoon-toolreg`][hp] adapter package.

[hp]: ../hermes-thoon-toolreg/

## Status

Phase 1 — currently exposes only `version()` to validate the maturin +
PyO3 build chain end-to-end. Real implementation (concurrent map,
`schemars` schema generation, fast dispatch) lands next.

## Build

```bash
cd thoon/thoon-toolreg
maturin develop
```

## Usage (Python)

```python
import thoon_toolreg

# Currently the only exported symbol — confirms the wheel is loadable.
print(thoon_toolreg.version())   # "0.5.20260520"
```

Once Phase 1 implementation lands, the Python API will look roughly
like this:

```python
import thoon_toolreg

reg = thoon_toolreg.ToolRegistry()
reg.register("greet", schema={"type": "object"}, fn=lambda **kw: "hi")
result = reg.dispatch("greet", {})
```

## License

Apache-2.0.
