/// Possible errors during decoding of [`Message`]s.
#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("message is missing mandatory field '{}'", .0)]
    MissingMandatoryField(&'static str),

    #[error("message contains duplicate of field '{}'", .0)]
    DuplicateField(&'static str),

    #[error("Message starts with an invalid begin string {}", String::from_utf8_lossy(.0))]
    InvalidBeginString(Vec<u8>),

    #[error("message contains invalid checksum value '{}'", String::from_utf8_lossy(.0))]
    InvalidChecksum(Vec<u8>),

    #[error(
        "calculated and expected checksums don't match 'calculated({calculated}) != ({expected})'"
    )]
    ChecksumMismatch { calculated: u8, expected: u8 },

    #[error("unexpected empty field in message")]
    UnexpectedEmptyField,

    #[error("Invalid tag value")]
    BadTag,
}

#[derive(Default)]
struct Digest {
    checksum: u8,
}

impl Digest {
    fn push(&mut self, input: &[u8]) {
        for b in input {
            self.checksum = self.checksum.overflowing_add(*b).0;
        }
    }

    fn byte(&mut self, byte: u8) {
        self.checksum = self.checksum.overflowing_add(byte).0;
    }
}

#[derive(Default)]
struct Parser {
    digest: Digest,
}

impl Parser {
    fn expect_tag<'input>(
        &mut self,
        input: &'input [u8],
        tag: Option<&[u8]>,
    ) -> Result<(&'input [u8], &'input [u8]), Error> {
        let mut split = input.split(|b| *b == b'=');

        let tag_predicate = |t| tag.is_none_or(|expected| expected == t);

        match dbg!((split.next(), split.next(), split.next())) {
            (Some(found_tag), Some(value), None) if tag_predicate(found_tag) => {
                self.digest.push(input);
                Ok((found_tag, value))
            }
            _ => Err(Error::InvalidBeginString(input.into())),
        }
    }

    #[inline]
    fn parse_begin_string<'input>(&mut self, input: &'input [u8]) -> Result<&'input [u8], Error> {
        self.expect_tag(input, Some(b"8"))
            .map(|(_tag, value)| value)
    }

    #[inline]
    fn parse_body_length<'input>(&mut self, input: &'input [u8]) -> Result<&'input [u8], Error> {
        self.expect_tag(input, Some(b"9"))
            .map(|(_tag, value)| value)
    }

    #[inline]
    fn parse_message_type<'input>(&mut self, input: &'input [u8]) -> Result<&'input [u8], Error> {
        self.expect_tag(input, Some(b"35"))
            .map(|(_tag, value)| value)
    }

    fn parse_field(&mut self, input: &[u8]) -> Result<Field, Error> {
        let (tag, value) = self.expect_tag(input, None)?;

        let tag = str::from_utf8(tag).map_err(|_| Error::BadTag)?;
        let tag = tag.parse().map_err(|_| Error::BadTag)?;

        Ok(Field {
            tag,
            value: value.into(),
        })
    }

    fn parse_checksum(&mut self, input: &[u8]) -> Result<u8, Error> {
        let mut split = input.split(|b| *b == b'=');

        match (split.next(), split.next(), split.next()) {
            (Some(tag), Some(value), None) if tag == b"10" => {
                let value =
                    str::from_utf8(value).map_err(|_| Error::InvalidChecksum(value.to_vec()))?;
                let value: u8 = value
                    .parse()
                    .map_err(|_| Error::InvalidChecksum(value.into()))?;
                Ok(value)
            }
            _ => Err(Error::InvalidChecksum(input.to_vec())),
        }
    }
}

/// Represents a single message in FIX protocol. Instance of this type guarantees a valid FIX
/// message.
#[derive(Debug)]
pub struct Message {
    #[allow(dead_code)]
    fields: Vec<Field>,
}

impl Message {
    const SOH: u8 = b'\x01';

    /// Decodes a [`Message`] from a byte array-like object. The byte array must be trimmed (i.e.
    /// no whitespace as prefix and/or sufix), and must contain exactly one message. Otherwise,
    /// parsing will fail and return an error.
    pub fn decode(bytes: impl AsRef<[u8]>) -> Result<Message, Error> {
        let mut parser = Parser::default();
        let bytes = bytes.as_ref();

        let mut split = bytes.split(|byte| *byte == Self::SOH).peekable();

        let Some(next) = split.next() else {
            return Err(Error::MissingMandatoryField("begin string"));
        };

        let begin_string = parser.parse_begin_string(next)?;
        parser.digest.byte(Self::SOH);

        let Some(next) = split.next() else {
            return Err(Error::MissingMandatoryField("body length"));
        };
        let body_length = parser.parse_body_length(next)?;
        parser.digest.byte(Self::SOH);

        let Some(next) = split.next() else {
            return Err(Error::MissingMandatoryField("message type"));
        };

        let msg_type = parser.parse_message_type(next)?;
        parser.digest.byte(Self::SOH);

        let mut checksum: Option<u8> = None;

        let mut fields = vec![
            Field {
                tag: 8,
                value: begin_string.into(),
            },
            Field {
                tag: 9,
                value: body_length.into(),
            },
        ];

        while let Some(next) = split.next() {
            if next.starts_with(b"10=") {
                if checksum.is_none() {
                    checksum = Some(parser.parse_checksum(next)?);
                } else {
                    return Err(Error::DuplicateField("checksum"));
                }
            } else if !next.is_empty() {
                let field = parser.parse_field(next)?;
                parser.digest.byte(Self::SOH);
                fields.push(field);
            } else {
                // empty field indicates either we have two SOHs next to each other, other we hit
                // the last SOH
                if split.peek().is_some() {
                    return Err(Error::UnexpectedEmptyField);
                }
            }
        }

        let Some(expected_checksum) = checksum else {
            return Err(Error::MissingMandatoryField("checksum"));
        };

        let calculated_checksum = parser.digest.checksum;

        if expected_checksum != calculated_checksum {
            return Err(Error::ChecksumMismatch {
                calculated: calculated_checksum,
                expected: expected_checksum,
            });
        }

        fields.push(Field {
            tag: 10,
            value: vec![expected_checksum],
        });

        fields.push(Field {
            tag: 35,
            value: msg_type.into(),
        });

        Ok(Message { fields })
    }
}

#[derive(Debug)]
pub struct Field {
    #[allow(dead_code)]
    tag: u16,
    #[allow(dead_code)]
    value: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use crate::decoder::{Error, Message};

    #[test]
    fn parse_valid_message() {
        let input = "8=FIX.4.4\x019=148\x0135=D\x0134=1080\x0149=TESTBUY1\x0152=20180920-18:14:19.508\x0156=TESTSELL1\x0111=636730640278898634\x0115=USD\x0121=2\x0138=7000\x0140=1\x0154=1\x0155=MSFT\x0160=20180920-18:14:19.492\x0110=092\x01";

        let message = Message::decode(input).expect("valid message must be decoded");

        assert_eq!(message.fields.len(), 16);
    }

    #[test]
    fn bad_checksum() {
        let input = "8=FIX.4.4\x019=148\x0135=D\x0134=1080\x0149=TESTBUY1\x0152=20180920-18:14:19.508\x0156=TESTSELL1\x0111=636730640278898634\x0115=USD\x0121=2\x0138=7000\x0140=1\x0154=1\x0155=MSFT\x0160=20180920-18:14:19.492\x0110=000\x01";

        let error = Message::decode(input).expect_err("checksum is not valid");

        assert!(matches!(error, Error::ChecksumMismatch { .. }));
    }
}
