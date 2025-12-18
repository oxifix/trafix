#![warn(clippy::pedantic)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]
#![forbid(unsafe_code)]

//! `trafix-codec` is a low-level library for high-performance parsing,
//! encoding, and validation of FIX messages.

mod digest;

pub(crate) mod constants;
pub(crate) mod decoder;
pub(crate) mod encoder;
pub mod message;
