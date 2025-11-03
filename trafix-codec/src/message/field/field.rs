//! Comment

use crate::message::field::value::aliases::{MsgSeqNum, SenderCompID, TargetCompID};

/// Comment
#[derive(Clone, Debug, PartialEq)]
pub enum Field {
    /// Comment
    MsgSeqNum(MsgSeqNum),

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
            Field::MsgSeqNum(msg_seq_num) => {
                let mut vec = b"34=".to_vec();
                vec.extend_from_slice(msg_seq_num.to_string().as_bytes());
                vec
            }

            Field::SenderCompID(sender_comp_id) => {
                let mut vec = b"49=".to_vec();
                vec.extend(&sender_comp_id);
                vec
            }

            Field::SendingTime => b"52=".to_vec(),

            Field::TargetCompID(target_comp_id) => {
                let mut vec = b"56=".to_vec();
                vec.extend(&target_comp_id);
                vec
            }

            Field::Custom { tag, value } => {
                let mut vec = Vec::from(tag.to_string().as_bytes());
                vec.extend(b"=");
                vec.extend(value);

                vec
            }
        }
    }
}
