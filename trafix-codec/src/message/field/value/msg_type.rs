//! Comment

/// Comment
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MsgType {
    /// Comment
    Logon,

    /// Comment
    Heartbeat,

    /// Comment
    TestRequest,

    /// Comment
    ResendRequest,

    /// Comment
    Reject,

    /// Comment
    SequenceReset,

    /// Comment
    Logout,
}

impl From<MsgType> for Vec<u8> {
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
