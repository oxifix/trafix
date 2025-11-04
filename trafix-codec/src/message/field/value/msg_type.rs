//! Defines the [`MsgType`] enumeration representing the FIX **35 `MsgType`** field value.

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

impl From<MsgType> for Vec<u8> {
    /// Converts a [`MsgType`] variant into its FIX wire representation.
    ///
    /// Returns the single-byte ASCII code that identifies the message type,
    /// suitable for direct use in encoding.
    ///
    /// ```
    /// use trafix_codec::message::field::value::msg_type::MsgType;
    /// let bytes: Vec<u8> = MsgType::Logon.into();
    /// assert_eq!(bytes, b"A");
    /// ```
    fn from(val: MsgType) -> Self {
        match val {
            MsgType::Logon => b"A".to_vec(),
            MsgType::Heartbeat => b"0".to_vec(),
            MsgType::TestRequest => b"1".to_vec(),
            MsgType::ResendRequest => b"2".to_vec(),
            MsgType::Reject => b"3".to_vec(),
            MsgType::SequenceReset => b"4".to_vec(),
            MsgType::Logout => b"5".to_vec(),
        }
    }
}
