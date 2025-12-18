//! Implementation of the field module.

pub mod value;

use crate::message::field::value::aliases::{MsgSeqNum, SenderCompID, SendingTime, TargetCompID};

/// Macro that generates the [`Field`] enum and its core utility methods.
///
/// Each macro entry defines:
/// - the enum variant name,
/// - the Rust type for its value,
/// - the FIX tag number,
/// - a match binding + expression returning the serialized value.
///
/// The macro expands into:
/// - the [`Field`] enum,
/// - a [`Field::tag`] method returning the tag number,
/// - a [`Field::value`] method returning the encoded byte value,
/// - and a [`Field::encode`] method producing the `"tag=value"` byte sequence.
macro_rules! fields_macro {
    ($($(#[$($attrs:tt)*])* $variant:ident($type:ty) = $tag:literal => $match:ident $expr:expr),+) => {
        /// Represents a single FIX field.
        ///
        /// Each variant corresponds to a strongly-typed FIX tag, such as
        /// `MsgSeqNum(34)` or `SenderCompID(49)`. Fields not covered by
        /// predefined variants can be represented using [`Field::Custom`].
        #[derive(Debug, Clone, PartialEq)]
        pub enum Field {
            $(
            $(#[$($attrs)*])*
            $variant($type)
            ),+,

            /// Represents an arbitrary or user-defined FIX field not covered
            /// by the predefined variants.
            ///
            /// Useful for extension tags, firm-specific fields, or when
            /// working with non-standard message structures.
            Custom {
                /// Tag of the custom field.
                tag: u16,
                /// Contents of the custom field.
                value: Vec<u8>
            }
        }

        impl Field {
            /// Tries to construct a new [`Field`] from the given tag and value.
            ///
            /// # Errors
            ///
            /// This function might return error if invalid values are passed for the given tag.
            pub fn try_new(tag: u16, bytes: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
                use value::FromFixBytes;

                match tag {
                    $(
                    $tag => Ok(Self::$variant(<$type as FromFixBytes>::from_fix_bytes(bytes)?)),
                    )*
                    other => Ok(Field::Custom {
                        tag: other,
                        value: bytes.into(),
                    })
                }
            }

            /// Returns the numeric FIX tag associated with this field.
            ///
            /// Example usage:
            /// ```
            /// use trafix_codec::message::field::{Field, value::aliases::MsgSeqNum};
            /// let f = Field::MsgSeqNum(1);
            /// assert_eq!(f.tag(), 34);
            /// ```
            #[must_use]
            pub fn tag(&self) -> u16 {
                match self {
                    $(
                    Field::$variant(_) => $tag
                    ),+,

                    Field::Custom { tag, .. } => { *tag }
                }
            }

            /// Returns the serialized value of the field as raw bytes.
            ///
            /// For predefined fields, this returns their encoded textual
            /// representation (e.g. integer → ASCII). For custom fields, the
            /// original byte vector is cloned.
            #[must_use]
            pub fn value(&self) -> Vec<u8> {
                match self {
                    $(
                    Field::$variant($match) => $expr
                    ),+,

                    Field::Custom { value, .. } => { value.clone() }
                }
            }

            /// Serializes the field into its `"tag=value"` representation.
            ///
            /// This does **not** append the SOH delimiter; it only produces
            /// the byte content for a single field. The encoder is
            /// responsible for joining fields with SOH (`0x01`).
            ///
            /// ```
            /// use trafix_codec::message::field::{Field, value::aliases::MsgSeqNum};
            /// let f = Field::MsgSeqNum(4);
            /// assert_eq!(f.encode(), b"34=4".to_vec());
            /// ```
            #[must_use]
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
    /// Message sequence number (`34`).
    ///
    /// Used to identify message ordering within a FIX session.
    MsgSeqNum(MsgSeqNum) = 34 => msg_seq_num format!("{msg_seq_num}").into_bytes(),

    /// Sender company or system identifier (`49`).
    ///
    /// Identifies the sender of the message in a FIX session.
    SenderCompID(SenderCompID) = 49 => sender_comp_id sender_comp_id.clone(),

    /// Message sending time (`52`).
    ///
    /// Timestamp representing when the message was sent.
    SendingTime(SendingTime) = 52 => sending_time sending_time.clone(),

    /// Target company or system identifier (`56`).
    ///
    /// Identifies the intended recipient of the message in a FIX session.
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
