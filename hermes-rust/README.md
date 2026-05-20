# hermes-rust

Rust acceleration workspace for the Hermes Agent.

See [`../PLAN.md`](../PLAN.md) for the full design and rollout plan.

## Phase 1 quickstart

```bash
cd hermes-toolreg
maturin develop
python -c "import hermes_toolreg; print(hermes_toolreg.version())"
```

## Layout

| Crate              | Phase | Status        |
| ------------------ | :---: | ------------- |
| `hermes-core`      |   -   | shared types  |
| `hermes-toolreg`   |   1   | hello-world   |
| `hermes-fileops`   |   2   | placeholder   |
| `hermes-sessiondb` |   3   | placeholder   |
| `hermes-msgproc`   |   4   | deferred      |
