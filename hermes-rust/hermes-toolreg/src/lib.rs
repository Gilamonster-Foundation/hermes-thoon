#![forbid(unsafe_code)]

//! Phase 1 — Rust tool registry and dispatch for the Hermes Agent.
//!
//! This file currently exposes only `version()` so that the maturin +
//! PyO3 toolchain can be validated end-to-end before real work lands.
//! The actual registry, schema generation, and dispatcher follow in
//! subsequent PRs against the `thoon` branch.

use pyo3::prelude::*;

const HERMES_TOOLREG_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Return the hermes-toolreg crate version.
#[pyfunction]
fn version() -> &'static str {
    HERMES_TOOLREG_VERSION
}

#[pymodule]
fn hermes_toolreg(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add("__version__", HERMES_TOOLREG_VERSION)?;
    Ok(())
}
