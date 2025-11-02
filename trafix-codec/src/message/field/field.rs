//! Comment

use crate::message::field::value::{
    aliases::{BodyLength, Checksum, MsgSeqNum, SenderCompID, TargetCompID},
    begin_string::BeginString,
    msg_type::MsgType,
};

/// Comment
#[derive(Clone, Debug, PartialEq)]
pub enum Field {
    /// Comment
    BeginString(BeginString),

    /// Comment
    BodyLength(BodyLength),

    /// Comment
    Checksum(Checksum),

    /// Comment
    MsgSeqNum(MsgSeqNum),

    /// Comment
    MsgType(MsgType),

    /// Comment
    SenderCompID(SenderCompID),

    /// Comment
    SendingTime,

    /// Comment
    TargetCompID(TargetCompID),

    /// Comment
    Custom { tag: u16, value: Vec<u8> },
}

impl From<Field> for Vec<u8> {
    fn from(val: Field) -> Self {
        match val {
            Field::BeginString(begin_string) => todo!(),
            Field::BodyLength(body_length) => todo!(),
            Field::Checksum(checksum) => todo!(),
            Field::MsgSeqNum(msg_seq_num) => todo!(),
            Field::MsgType(msg_type) => todo!(),
            Field::SenderCompID(sender_comp_id) => todo!(),
            Field::SendingTime => todo!(),
            Field::TargetCompID(target_comp_id) => todo!(),
            Field::Custom { tag, value } => todo!(),
        }
    }
}
