//! Comment

pub mod value;

use crate::message::field::value::aliases::{MsgSeqNum, SenderCompID, SendingTime, TargetCompID};

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
    MsgSeqNum(MsgSeqNum) = 34 => msg_seq_num format!("{msg_seq_num}").into_bytes(),

    /// Comment
    SenderCompID(SenderCompID) = 49 => sender_comp_id sender_comp_id.clone(),

    /// Comment
    SendingTime(SendingTime) = 52 => sending_time sending_time.clone(),

    /// Comment
    TargetCompID(TargetCompID) = 56 => target_comp_id target_comp_id.clone()
}

#[cfg(test)]
mod test {
    use crate::message::field::{
        Field,
        value::aliases::{MsgSeqNum, SenderCompID, SendingTime, TargetCompID},
    };

    #[test]
    fn tag() {
        let msg_seq_num_field = Field::MsgSeqNum(0);
        assert_eq!(msg_seq_num_field.tag(), 34);

        let sender_comp_id_field = Field::SenderCompID(SenderCompID::new());
        assert_eq!(sender_comp_id_field.tag(), 49);

        let sending_time_field = Field::SendingTime(SendingTime::new());
        assert_eq!(sending_time_field.tag(), 52);

        let target_comp_id_field = Field::TargetCompID(TargetCompID::new());
        assert_eq!(target_comp_id_field.tag(), 56);
    }

    #[test]
    fn value() {
        let target_comp_id = TargetCompID::from(b"trafix-codec");
        let target_comp_id_field = Field::TargetCompID(target_comp_id.clone());

        assert_eq!(target_comp_id_field.tag(), 56);
        assert_eq!(target_comp_id_field.value(), target_comp_id);
    }

    #[test]
    fn encode() {
        let msg_seq_num: MsgSeqNum = 4;
        let msg_seq_num_field = Field::MsgSeqNum(msg_seq_num);

        assert_eq!(msg_seq_num_field.tag(), 34);
        assert_eq!(
            msg_seq_num_field.value(),
            format!("{msg_seq_num}").into_bytes()
        );

        // b"34=4"
        assert_eq!(
            msg_seq_num_field.encode(),
            format!("34={msg_seq_num}").into_bytes()
        );
    }

    #[test]
    fn custom_field() {
        let tag = 62000;
        let value = b"trafix-codec".to_vec();

        let custom_field = Field::Custom {
            tag,
            value: value.clone(),
        };

        assert_eq!(custom_field.tag(), tag);
        assert_eq!(custom_field.value(), value.clone());

        let mut encoded = Vec::from(tag.to_string().as_bytes());
        encoded.extend(b"=");
        encoded.extend(value);

        // b"62000=trafix-codec"
        assert_eq!(custom_field.encode(), encoded);
    }
}
