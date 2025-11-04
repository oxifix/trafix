//! Defines the [`BeginString`] value type, representing the FIX protocol version
//! used in the message header.
//!
//! The `BeginString` corresponds to FIX tag **8**, which identifies the
//! protocol version of the message. It is always the **first field** in
//! every FIX message.

// TODO: Limit visibility to crate once standards are introduced.

/// Represents the FIX protocol version specified in tag **8 `BeginString`**.
///
/// This field determines the message format and version-specific rules
/// that apply to subsequent tags in the message.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BeginString {
    /// FIX.4.4 protocol version (`8=FIX.4.4`).
    FIX44,
}

impl From<BeginString> for Vec<u8> {
    /// Converts a [`BeginString`] variant into its byte representation,
    /// suitable for direct inclusion in the encoded FIX message.
    ///
    /// ```
    /// use trafix_codec::message::field::value::begin_string::BeginString;
    /// let bytes: Vec<u8> = BeginString::FIX44.into();
    /// assert_eq!(bytes, b"FIX.4.4");
    /// ```
    fn from(val: BeginString) -> Self {
        match val {
            BeginString::FIX44 => b"FIX.4.4".to_vec(),
        }
    }
}
