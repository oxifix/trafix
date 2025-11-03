//! Comment

use crate::message::field::value::aliases::{MsgSeqNum, SenderCompID, SendingTime, TargetCompID};

pub mod value;

/// Comment
macro_rules! fields_macro {
    ($($(#[$($attrs:tt)*])* $variant:ident($type:ty) = $tag:literal => $match:ident $expr:expr),+) => {
        /// Comment
        #[derive(Debug, Clone, PartialEq)]
        pub enum Field {
            $(
            $(#[$($attrs)*])*
            $variant($type)
            ),+,

            /// Comment
            Custom { tag: u16, value: Vec<u8> }
        }

        impl Field {
            /// Comment
            pub fn tag(&self) -> u16 {
                match self {
                    $(
                    Field::$variant(_) => $tag
                    ),+,

                    Field::Custom { tag, .. } => { *tag }
                }
            }

            /// Comment
            pub fn value(&self) -> Vec<u8> {
                match self {
                    $(
                    Field::$variant($match) => $expr
                    ),+,

                    Field::Custom { value, .. } => { value.clone() }
                }
            }

            /// Comment
            pub fn encode(&self) -> Vec<u8> {
                match self {
                    $(
                    Field::$variant($match) => {
                        let tag = $tag;
                        let mut val = $expr;

                        let mut field = format!("{tag}=").into_bytes();
                        field.append(&mut val);

                        field
                    }
                    ),+,

                    Field::Custom { tag, value } => {
                        let mut field = format!("{tag}=").into_bytes();
                        field.append(&mut value.clone());

                        field
                    }
                }
            }
        }
    };
}

fields_macro! {
    /// Comment
    MsgSeqNum(MsgSeqNum) = 44 => msg_seq_num format!("{msg_seq_num}").into_bytes(),

    /// Comment
    SenderCompID(SenderCompID) = 44 => sender_comp_id sender_comp_id.clone(),

    /// Comment
    SendingTime(SendingTime) = 44 => sending_time sending_time.clone(),

    /// Comment
    TargetCompID(TargetCompID) = 44 => target_comp_id target_comp_id.clone()
}
