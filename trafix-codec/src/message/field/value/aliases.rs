//! Definitions of type aliases used for common FIX field values.
//!
//! These aliases provide clearer semantic meaning when working with
//! strongly typed [`Field`](crate::message::field::Field) variants.

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
