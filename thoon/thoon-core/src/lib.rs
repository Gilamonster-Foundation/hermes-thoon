// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

//! Shared error type and protocol primitives for `thoon-*` crates.
//!
//! Generic across LLM-agent frameworks. No Hermes-specific types here.
//! Anything coupled to Hermes' contracts lives in the `hermes-thoon-*`
//! adapter packages.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ThoonError {
    #[error("io error: {0}")]
    Io(String),

    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, ThoonError>;
