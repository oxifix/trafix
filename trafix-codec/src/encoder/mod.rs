//! Implementation of the Message encoder.

use bytes::{BufMut, Bytes, BytesMut};

use crate::message::{Body, Header, field::Field};

/// Computes the running FIX checksum (tag 10) while encoding.
#[derive(Default)]
struct Digest {
    checksum: u8,
}

impl Digest {
    /// Updates the running checksum using the contents of a [`BytesMut`].
    ///
    /// This performs modulo-256 addition across all bytes, matching the FIX
    /// checksum algorithm.
    pub fn push(&mut self, input: &BytesMut) {
        for &b in input.as_ref() {
            self.checksum = self.checksum.wrapping_add(b);
        }
    }
}

/// ASCII SOH delimiter (0x01) used as field terminator in FIX messages.
const SOH: u8 = b'\x01';

/// Average bytes per field in a FIX Message. We can safely assume that the average number of bytes
/// per field is around 15 bytes as per our measurements.
const AVERAGE_BYTES_PER_FIELD: usize = 15;

/// Encodes a full FIX message (header + body + trailer) into a final wire-format `Bytes` buffer
/// during which fields `BodyLength` and `Checksum` are calculated and set.
pub(crate) fn encode(header: &Header, body: &Body) -> Bytes {
    let regular_fields = encode_regular_fields(header, body);
    let message = encode_framing_headers(header, &regular_fields);
    finalize_message(message)
}

/// Encodes all regular fields (`MsgType`, optional header fields, body fields)
/// starting at tag 35 and ending before tag 10.
#[must_use]
fn encode_regular_fields(header: &Header, body: &Body) -> BytesMut {
    // reserving the capacity, counting that each field has AT LEAST 4 bytes b"X=Y\x01" to
    // reduce the number of resizings.
    //
    // +1 represents the MsgType that's outside the fields vec
    let mut message = BytesMut::with_capacity(
        (header.fields.len() + body.fields.len() + 1) * AVERAGE_BYTES_PER_FIELD,
    );

    // MsgType with included SOH char
    message.extend_from_slice(
        Field::Custom {
            tag: 35,
            value: header.msg_type.into(),
        }
        .encode()
        .as_ref(),
    );
    message.put_u8(SOH);

    // Optional header fields
    for field in &header.fields {
        // field with included SOH char.. x=ab\x01
        let mut field_soh = field.encode();
        field_soh.push(SOH);

        // encode the field into the message
        message.extend_from_slice(field_soh.as_ref());
    }

    // Body fields
    for field in &body.fields {
        // field with included SOH char.. x=ab\x01
        let mut field_soh = field.encode();
        field_soh.push(SOH);

        // encode the field into the message
        message.extend_from_slice(field_soh.as_ref());
    }

    message
}

/// Prepends `8=BeginString` and `9=BodyLength` fields to the provided bytes buffer.
#[must_use]
fn encode_framing_headers(header: &Header, regular_fields: &BytesMut) -> BytesMut {
    // 3 * the average bytes per field representing fields: BeginString, BodyLength, Checksum
    let mut message = BytesMut::with_capacity(regular_fields.len() + (3 * AVERAGE_BYTES_PER_FIELD));

    // BeginString with included SOH char
    message.extend_from_slice(
        Field::Custom {
            tag: 8,
            value: header.begin_string.into(),
        }
        .encode()
        .as_ref(),
    );
    message.put_u8(SOH);

    // BodyLength with included SOH char
    message.extend_from_slice(
        Field::Custom {
            tag: 9,
            value: format!("{}", regular_fields.len()).into_bytes(),
        }
        .encode()
        .as_ref(),
    );
    message.put_u8(SOH);

    // append the all the regular fields
    message.extend_from_slice(regular_fields);

    message
}

/// Appends the trailer (`10=CheckSum` field) to the provided bytes buffer and finalizes the
/// FIX message buffer.
fn finalize_message(mut message: BytesMut) -> Bytes {
    let mut digest = Digest::default();
    digest.push(&message);

    // Checksum with included SOH char
    let mut checksum_soh = Field::Custom {
        tag: 10,
        value: format!("{}", digest.checksum).into_bytes(),
    }
    .encode();
    checksum_soh.push(SOH);

    // encode the Checksum into the message
    message.put(checksum_soh.as_ref());

    message.freeze()
}

#[cfg(test)]
mod test {
    use bytes::Bytes;

    use crate::{
        encoder::encode,
        message::{
            Body, Header,
            field::{
                Field,
                value::{begin_string::BeginString, msg_type::MsgType},
            },
        },
    };

    /// Converts a bytes FIX frame to a `String`, making it human-readable by replacing the SOH
    /// character with '|'.
    fn humanize(encoded_message: &Bytes) -> String {
        String::from_utf8_lossy(encoded_message).replace(super::SOH as char, "|")
    }

    #[test]
    fn message_with_minimal_header() {
        let header = Header {
            begin_string: BeginString::FIX44,
            msg_type: MsgType::Logon,
            fields: Vec::new(),
        };

        let body = Body { fields: Vec::new() };

        let encoded_message = encode(&header, &body);

        insta::assert_snapshot!(humanize(&encoded_message), @"8=FIX.4.4|9=5|35=A|10=180|");
    }

    #[test]
    fn message_with_optional_header_fields() {
        let mut header = Header {
            begin_string: BeginString::FIX44,
            msg_type: MsgType::Logon,
            fields: Vec::new(),
        };

        let body = Body { fields: Vec::new() };

        // add optional header field
        header.fields.push(Field::Custom {
            tag: 144,
            value: Vec::from(b"value144"),
        });

        let encoded_message = encode(&header, &body);

        insta::assert_snapshot!(humanize(&encoded_message), @"8=FIX.4.4|9=18|35=A|144=value144|10=117|");
    }

    #[test]
    fn message_with_header_and_body_fields() {
        let mut header = Header {
            begin_string: BeginString::FIX44,
            msg_type: MsgType::Logon,
            fields: Vec::new(),
        };

        let mut body = Body { fields: Vec::new() };

        // add optional header field
        header.fields.push(Field::Custom {
            tag: 144,
            value: Vec::from(b"value144"),
        });

        // add a body field
        body.fields.push(Field::Custom {
            tag: 1234,
            value: Vec::from(b"value1234"),
        });

        // add a body field
        body.fields.push(Field::Custom {
            tag: 12345,
            value: Vec::from(b"value12345"),
        });

        let encoded_message = encode(&header, &body);

        insta::assert_snapshot!(humanize(&encoded_message), @"8=FIX.4.4|9=50|35=A|144=value144|1234=value1234|12345=value12345|10=185|");
    }
}
