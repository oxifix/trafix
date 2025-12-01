//! Defines the [`MsgType`] enumeration representing the FIX **35 `MsgType`** field value.

use crate::message::field::value::FromFixBytes;

/// Represents the FIX message type (`35`) field value.
///
/// Each variant corresponds to a well-known administrative message
/// used in FIX session-level communication.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MsgType {
    /// `Logon` message (`35=A`), representing a session initiation request.
    Logon,

    /// `Heartbeat` message (`35=0`), representing a session maintenance signal.
    Heartbeat,

    /// `TestRequest` message (`35=1`), representing a heartbeat request from the counterparty.
    TestRequest,

    /// `ResendRequest` message (`35=2`), representing a retransmission of missed messages request.
    ResendRequest,

    /// `Reject` message (`35=3`), representing a problem with a received message.
    Reject,

    /// `SequenceReset` message (`35=4`), represents the resetting of expected sequence numbers (or filling of gaps).
    SequenceReset,

    /// `Logout` message (`35=5`), representing a session termination (grafecul) request.
    Logout,
}

impl MsgType {
    pub const fn tag() -> u16 {
        35
    }
}

impl From<MsgType> for &'static [u8] {
    /// Converts a [`MsgType`] variant into its **static byte slice**
    /// representation, corresponding to the FIX wire value of tag **35**.
    ///
    /// This conversion is zero-allocation and suitable for direct use when
    /// encoding FIX messages.
    ///
    /// Example usage:
    /// ```
    /// use trafix_codec::message::field::value::msg_type::MsgType;
    /// let bytes: &'static [u8] = MsgType::Heartbeat.into();
    /// assert_eq!(bytes, b"0");
    /// ```
    fn from(val: MsgType) -> Self {
        match val {
            MsgType::Logon => b"A",
            MsgType::Heartbeat => b"0",
            MsgType::TestRequest => b"1",
            MsgType::ResendRequest => b"2",
            MsgType::Reject => b"3",
            MsgType::SequenceReset => b"4",
            MsgType::Logout => b"5",
        }
    }
}

impl From<MsgType> for Vec<u8> {
    /// Converts a [`MsgType`] variant into an **owned `Vec<u8>`**
    /// containing its FIX wire representation (tag **35** value).
    ///
    /// Example usage:
    /// ```
    /// use trafix_codec::message::field::value::msg_type::MsgType;
    /// let bytes: Vec<u8> = MsgType::Logout.into();
    /// assert_eq!(bytes, b"5");
    /// ```
    fn from(val: MsgType) -> Self {
        <&[u8]>::from(val).to_vec()
    }
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum ParseError<'input> {
    #[error("unsupported message type: {}", String::from_utf8_lossy(.0))]
    Unsupported(&'input [u8]),
}

impl FromFixBytes for MsgType {
    type Error<'input> = ParseError<'input>;

    fn from_fix_bytes<'bytes>(bytes: &'bytes [u8]) -> Result<Self, Self::Error<'bytes>>
    where
        Self: Sized,
    {
        match bytes {
            b"A" => Ok(MsgType::Logon),
            b"0" => Ok(MsgType::Heartbeat),
            b"1" => Ok(MsgType::TestRequest),
            b"2" => Ok(MsgType::ResendRequest),
            b"3" => Ok(MsgType::Reject),
            b"4" => Ok(MsgType::SequenceReset),
            b"5" => Ok(MsgType::Logout),
            other => Err(ParseError::Unsupported(other)),
        }
    }
}
