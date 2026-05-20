#![forbid(unsafe_code)]

//! `thoon-toolreg` — generic concurrent tool registry primitive.
//!
//! Framework-agnostic. Provides the storage and dispatch primitives a
//! tool registry needs; Hermes-specific shape (the `ToolRegistry` class,
//! its method signatures, the way schemas are emitted) lives in the
//! `hermes-thoon-toolreg` adapter package.
//!
//! Currently exposes only `version()` so the maturin + PyO3 toolchain
//! can be validated end-to-end. Real implementation lands in Phase 1.

use pyo3::prelude::*;

const THOON_TOOLREG_VERSION: &str = env!("CARGO_PKG_VERSION");

#[pyfunction]
fn version() -> &'static str {
    THOON_TOOLREG_VERSION
}

#[pymodule]
fn thoon_toolreg(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add("__version__", THOON_TOOLREG_VERSION)?;
    Ok(())
}
