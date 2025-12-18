#![warn(clippy::pedantic)]
#![warn(missing_docs)]
#![forbid(unsafe_code)]

//! `trafix-codec` is a low-level library for high-performance parsing,
//! encoding, and validation of FIX messages.

mod digest;

pub(crate) mod constants;
pub(crate) mod decoder;
pub mod encoder;
pub mod message;
