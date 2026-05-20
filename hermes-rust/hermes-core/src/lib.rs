#![forbid(unsafe_code)]

//! Shared types and error mapping for the hermes-rust workspace.
//!
//! Each accelerator crate (`hermes-toolreg`, `hermes-fileops`,
//! `hermes-sessiondb`, `hermes-msgproc`) depends on this crate for its
//! common error enum and a future set of shared types (session ids,
//! tool-call envelopes, etc.).

use thiserror::Error;

#[derive(Debug, Error)]
pub enum HermesError {
    #[error("io error: {0}")]
    Io(String),

    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, HermesError>;
