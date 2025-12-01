#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]

//! `trafix-codec` is a low-level library for high-performance parsing,
//! encoding, and validation of FIX messages.

mod digest;

pub mod constants;
pub mod decoder;
pub mod encoder;
pub mod message;
