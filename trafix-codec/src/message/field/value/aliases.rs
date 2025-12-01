//! Definitions of type aliases used for common FIX field values.
//!
//! These aliases provide clearer semantic meaning when working with
//! strongly typed [`Field`](crate::message::field::Field) variants.

use std::convert::Infallible;

use crate::message::field::value::FromFixBytes;

/// Represents the `MsgSeqNum` (`34`).
///
/// This value increments with each message within a FIX session,
/// ensuring ordering and detection of missing or duplicated messages.
pub type MsgSeqNum = u64;

/// Represents the `SenderCompID` (`49`).
///
/// Identifies the sender of the FIX message (typically the firm,
/// system, or gateway). Stored as raw bytes to preserve any
/// non-UTF-8 or fixed-width encodings.
pub type SenderCompID = Vec<u8>;

/// Represents the `SendingTime` (`52`).
///
/// Timestamp indicating when the message was sent.
// TODO(kfejzic): Replace with a more specific time type, adhering to the
// FIXs SendingTime ruling: YYYYMMDD-HH:MM:SS[.sss]
pub type SendingTime = Vec<u8>;

/// Represents the `TargetCompID` (`56`).
///
/// Identifies the intended recipient of the FIX message.
/// Stored as raw bytes for full fidelity with on-wire data.
pub type TargetCompID = Vec<u8>;

impl FromFixBytes for Vec<u8> {
    type Error<'unused> = Infallible;

    fn from_fix_bytes(bytes: &[u8]) -> Result<Self, Self::Error<'_>>
    where
        Self: Sized,
    {
        Ok(bytes.into())
    }
}
