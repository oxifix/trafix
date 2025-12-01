//! Defines the [`BeginString`] enumeration, representing the FIX **8 `BeginString`**
//! field value.

// TODO(kfejzic): Limit visibility to crate once standards are introduced.

use crate::message::field::value::FromFixBytes;

/// Represents the FIX protocol version (`8`) field value.
///
/// This field value determines the message format and version-specific rules
/// that apply to subsequent tags in the message.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BeginString {
    /// FIX.4.4 protocol version (`8=FIX.4.4`).
    FIX44,
}

impl BeginString {
    #[must_use]
    pub const fn tag() -> u16 {
        8
    }
}

impl From<BeginString> for &'static [u8] {
    /// Converts a [`BeginString`] variant into its **static byte slice**
    /// representation.
    ///
    /// This form avoids allocation and is suitable for direct use
    /// when writing FIX messages to a buffer or network stream.
    ///
    /// Example usage:
    /// ```
    /// use trafix_codec::message::field::value::begin_string::BeginString;
    /// let bytes: &'static [u8] = BeginString::FIX44.into();
    /// assert_eq!(bytes, b"FIX.4.4");
    /// ```
    fn from(val: BeginString) -> Self {
        match val {
            BeginString::FIX44 => b"FIX.4.4",
        }
    }
}

impl From<BeginString> for Vec<u8> {
    /// Converts a [`BeginString`] variant into an **owned `Vec<u8>`**
    /// containing its byte representation.
    ///
    /// Example usage:
    /// ```
    /// use trafix_codec::message::field::value::begin_string::BeginString;
    /// let bytes: Vec<u8> = BeginString::FIX44.into();
    /// assert_eq!(bytes, b"FIX.4.4");
    /// ```
    fn from(val: BeginString) -> Self {
        <&[u8]>::from(val).to_vec()
    }
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum ParseError<'input> {
    #[error("unsupported fix version: {}", String::from_utf8_lossy(.0))]
    Unsupported(&'input [u8]),
}

impl FromFixBytes for BeginString {
    type Error<'input> = ParseError<'input>;

    fn from_fix_bytes(bytes: &[u8]) -> Result<Self, Self::Error<'_>>
    where
        Self: Sized,
    {
        if bytes == <&[u8]>::from(BeginString::FIX44) {
            Ok(BeginString::FIX44)
        } else {
            Err(ParseError::Unsupported(bytes))
        }
    }
}
