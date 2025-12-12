//! Decoder for messages in FIX protocol.

use crate::decoder::num::ParseFixInt as _;
use crate::digest::Digest;
use crate::message::field::Field;
use crate::message::field::value::FromFixBytes;
use crate::message::field::value::begin_string::BeginString;
use crate::message::field::value::msg_type::MsgType;
use crate::{constants, message::Message};

/// Length of the SOH character.
const SOH_LEN: usize = 1;

/// Lengths of the equals ('=') character.
const EQ_LEN: usize = 1;

/// Length of the tag for checksum ('10').
const CKSUM_TAG_LEN: usize = 2;

/// Extension trait for utility functions on [`Result`] type.
trait ResultExt<T> {
    /// Wraps the inner [`Result::Err`] with [`Error::BadValue`].
    fn or_bad_value(self) -> Result<T, Error>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: ToString,
{
    fn or_bad_value(self) -> Result<T, Error> {
        self.map_err(|inner| Error::BadValue(inner.to_string()))
    }
}

/// Possible errors during decoding of [`Message`]s.
#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    /// Message did not contain mandatory field.
    #[error("message is missing mandatory field '{}'", .0)]
    MissingMandatoryField(&'static str),

    /// Message contained checksum before end.
    #[error("checksum reached but message contains more fields")]
    UnexpectedChecksum,

    /// Message checksum does not match with what we calculated.
    #[error(
        "calculated and expected checksums don't match 'calculated({calculated}) != ({expected})'"
    )]
    ChecksumMismatch { calculated: u8, expected: u8 },

    /// Message contains invalid tag values.
    #[error("invalid tag: {}", .0)]
    BadTag(u16),

    /// Message body length does not match what was received.
    #[error("expected body length {expected} but received {received} bytes")]
    BodyLength { received: usize, expected: usize },

    /// Message contains invalid bytes.
    #[error("encountered error while parsing tokens: {}", .0)]
    Lexer(#[from] LexError),

    /// Message contains invalid values.
    #[error("Invalid value: {}", .0)]
    BadValue(String),
}

/// Errors that represent failures to decode symbols during lexing of FIX messages.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum LexError {
    /// Found different byte than what was expected.
    #[error("Expected '{expected}' but got {but_got}")]
    Unexpected { expected: u8, but_got: u8 },

    /// EOI reached but not expected.
    #[error("Unexpected end of input")]
    EOI,

    /// Expected EOI but more input was found.
    #[error("Expected end of input, but got {}", .0)]
    ExpectedEOI(u8),

    /// Tag contains bytes that are not ASCII decimal digits.
    #[error("Tag contains characters other than ascii 0-9 digits.")]
    MalformedTag,
}

/// Lexer reads the FIX message bytes and extracts tags and values from them.
struct Lexer<'input> {
    /// Byte slice containing FIX Message.
    input: &'input [u8],

    /// Current position in the input byte slice.
    cursor: usize,
}

impl<'input> Lexer<'input> {
    /// Skip expected byte if more bytes available.
    fn skip_or_eoi(&mut self, expected: u8) -> Result<Option<u8>, LexError> {
        match self.input.get(self.cursor) {
            None => Ok(None),
            Some(_) => self.skip(expected),
        }
    }

    /// Skip expected byte or error on EOI.
    fn skip(&mut self, expected: u8) -> Result<Option<u8>, LexError> {
        match self.input.get(self.cursor) {
            // got a byte that does not match with expected one
            Some(byte) if *byte != expected => Err(LexError::Unexpected {
                expected,
                but_got: *byte,
            }),

            // got a byte and it matches the expected one, so skip it
            Some(byte) => {
                self.cursor += 1;
                Ok(Some(*byte))
            }

            // got EOI, but expected a byte
            None => Err(LexError::EOI),
        }
    }

    /// Tries to lex out the tag of field in FIX Message.
    ///
    /// # Errors
    ///
    /// Returns an error on invalid tag, or if some other token is encountered.
    fn tag(&mut self) -> Result<u16, LexError> {
        let start = self.cursor;

        while let Some(byte) = self.input.get(self.cursor)
            && byte.is_ascii_digit()
        {
            self.cursor += 1;
        }

        // INVARIANT: cursor is on equals sign
        let end = self.cursor;
        self.skip(constants::EQUALS)?;

        let tag_bytes = self.input.get(start..end).ok_or(LexError::EOI)?;

        u16::parse_fix_int(tag_bytes).map_err(|_| LexError::MalformedTag)
    }

    /// Tries to lex out the value of field in FIX Message.
    ///
    /// # Errors
    ///
    /// Returns an error on invalid value, or if some other token is encountered.
    fn value(&mut self) -> Result<&'input [u8], LexError> {
        // INVARIANT: Cursor position right after '=' character
        let start = self.cursor;

        while let Some(byte) = self.input.get(self.cursor)
            && *byte != constants::SOH
        {
            self.cursor += 1;
        }

        // INVARIANT: We're either on SOH, or EOI
        let end = self.cursor;
        self.skip_or_eoi(constants::SOH)?;

        self.input.get(start..end).ok_or(LexError::EOI)
    }
}

impl<'slice> From<&'slice [u8]> for Lexer<'slice> {
    fn from(value: &'slice [u8]) -> Self {
        Self {
            input: value,
            cursor: 0,
        }
    }
}

/// Decodes a [`Message`] from a byte array-like object. The byte array must be trimmed (i.e.
/// no whitespace as prefix and/or sufix), and must contain exactly one message. Otherwise,
/// parsing will fail and return an error.
///
/// For now, this decodes a message with fixed (no pun intended) expectations regarding protocol
/// version and message layout. That means that arbitrary protocol requirements cannot be expressed
/// in this decoder function.
///
/// # Errors
///
/// Returns an [`Error`] on malformed message formats.
pub fn decode(bytes: impl AsRef<[u8]>) -> Result<Message, Error> {
    let bytes = bytes.as_ref();
    let mut lexer = Lexer::from(bytes);

    let tag = lexer.tag()?;
    let value = lexer.value()?;

    if tag != BeginString::tag() {
        return Err(Error::BadTag(tag));
    }

    let begin_string = BeginString::from_fix_bytes(value).or_bad_value()?;

    let tag = lexer.tag()?;
    let value = lexer.value()?;

    if tag != 9 {
        return Err(Error::MissingMandatoryField("body length"));
    }

    let body_length = usize::parse_fix_int(value).or_bad_value()?;
    let body_start_cursor = lexer.cursor;

    let tag = lexer.tag()?;

    if tag != MsgType::tag() {
        return Err(Error::BadTag(tag));
    }

    let value = lexer.value()?;
    let msg_type = MsgType::from_fix_bytes(value).or_bad_value()?;

    let builder = Message::builder(begin_string, msg_type);

    let mut builder = match (lexer.tag(), lexer.value()) {
        (Ok(tag), Ok(value)) => builder.with_field(Field::try_new(tag, value).or_bad_value()?),
        (Err(error), _) | (Ok(_), Err(error)) => return Err(Error::Lexer(error)),
    };

    while let Ok(tag) = lexer.tag() {
        let value = lexer.value()?;

        if tag == 10 {
            // checksum reached
            if lexer.tag().is_ok() {
                // there must be no fields after checksum!
                return Err(Error::UnexpectedChecksum);
            }

            let cursor_before_checksum =
                lexer.cursor - SOH_LEN - value.len() - EQ_LEN - CKSUM_TAG_LEN;

            // at this point we can calculate the body length:
            let received_body_length = cursor_before_checksum - body_start_cursor;

            if received_body_length != body_length {
                return Err(Error::BodyLength {
                    received: received_body_length,
                    expected: body_length,
                });
            }

            let calculated_checksum = {
                let mut digest = Digest::default();
                // cursor is right after the value of checksum, so for checksum we calculate all
                // bytes up to cursor - number of digits in value - 1 equals sign - 2 digits (10)
                let bytes_up_to_checksum = &bytes[..cursor_before_checksum];
                digest.push(&bytes_up_to_checksum);

                digest.checksum()
            };

            let expected_checksum = u8::parse_fix_int(value).or_bad_value()?;

            if calculated_checksum != expected_checksum {
                return Err(Error::ChecksumMismatch {
                    calculated: calculated_checksum,
                    expected: expected_checksum,
                });
            }
        } else {
            builder = builder.with_field(Field::try_new(tag, value).or_bad_value()?);
        }
    }

    let message = builder.build();
    Ok(message)
}

#[cfg(test)]
mod tests {
    use crate::decoder::decode::Error;
    use crate::message::Message;

    #[test]
    fn parse_valid_message() {
        let input = "8=FIX.4.4\x019=148\x0135=A\x0134=1080\x0149=TESTBUY1\x0152=20180920-18:14:19.508\x0156=TESTSELL1\x0111=636730640278898634\x0115=USD\x0121=2\x0138=7000\x0140=1\x0154=1\x0155=MSFT\x0160=20180920-18:14:19.492\x0110=089\x01";

        let decode_result = Message::decode(input);

        assert!(
            decode_result.is_ok(),
            "message decoding failed: {}",
            decode_result.unwrap_err()
        );
    }

    #[test]
    fn bad_checksum() {
        let input = "8=FIX.4.4\x019=148\x0135=A\x0134=1080\x0149=TESTBUY1\x0152=20180920-18:14:19.508\x0156=TESTSELL1\x0111=636730640278898634\x0115=USD\x0121=2\x0138=7000\x0140=1\x0154=1\x0155=MSFT\x0160=20180920-18:14:19.492\x0110=000\x01";

        let error = Message::decode(input).expect_err("checksum is not valid");

        assert!(matches!(error, Error::ChecksumMismatch { .. }));
    }

    #[test]
    fn bad_body_length() {
        let input = "8=FIX.4.4\x019=042\x0135=A\x0134=1080\x0149=TESTBUY1\x0152=20180920-18:14:19.508\x0156=TESTSELL1\x0111=636730640278898634\x0115=USD\x0121=2\x0138=7000\x0140=1\x0154=1\x0155=MSFT\x0160=20180920-18:14:19.492\x0110=089\x01";

        let error = Message::decode(input).expect_err("body length does not match");

        assert!(matches!(
            error,
            Error::BodyLength {
                expected: 42,
                received: 148
            }
        ));
    }
}
