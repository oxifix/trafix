//! Defines the [`BeginString`] enumeration, representing the FIX **8 `BeginString`**
//! field value.

// TODO(kfejzic): Limit visibility to crate once standards are introduced.

/// Represents the FIX protocol version (`8`) field value.
///
/// This field value determines the message format and version-specific rules
/// that apply to subsequent tags in the message.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BeginString {
    /// FIX.4.4 protocol version (`8=FIX.4.4`).
    FIX44,
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
    /// let bytes: &'static [u8] = (BeginString::FIX44).into();
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
    /// This is useful when the FIX version string needs to be
    /// stored, cloned, or manipulated dynamically.
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
