#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

pub mod diagnostic;
mod id;
pub mod prelude;
pub mod signal;
pub mod slot;
pub mod transport;

pub use id::Id;
pub use id::IdBuilder;
pub use id::PduFormat;
pub use id::Pgn;
