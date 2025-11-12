//! Comment

use bytes::{BufMut, Bytes, BytesMut};

use crate::message::{Body, Header, field::Field};

/// Comment
#[derive(Default)]
struct Digest {
    checksum: u8,
}

impl Digest {
    /// Comment
    fn push<'a, I>(&mut self, input: I)
    where
        I: IntoIterator<Item = &'a u8>,
    {
        for &b in input {
            self.checksum = self.checksum.overflowing_add(b).0;
        }
    }
}

/// Comment
#[derive(Default)]
pub(crate) struct Encoder {
    /// Comment
    digest: Digest,
}

impl Encoder {
    /// Comment
    const SOH: u8 = b'\x01';

    /// Comment
    pub(crate) fn encode(mut self, header: &Header, body: &Body) -> Bytes {
        // encode message header (except 8-BeginString, 9-BodyLength), and message body
        let regular_fields = Self::encode_regular_fields(header, body);

        // encode 8-BeginString, 9-BodyLength fields
        let message = Self::encode_framing_headers(header, &regular_fields);

        // encode 10-Checksum field and return frozen message
        self.encode_checksum(message)
    }

    /// Comment
    #[must_use]
    fn encode_regular_fields(header: &Header, body: &Body) -> BytesMut {
        // reserving the capacity, counting that each field has AT LEAST 4 bytes b"X=Y\x01" to
        // reduce the number of resizings. We can safely assume that the average number of bytes
        // per field is around 15 bytes as per our measurements.
        //
        // +1 represents the MsgType that's outside the fields vec
        let mut message =
            BytesMut::with_capacity((header.fields.len() + body.fields.len() + 1) * 15);

        // MsgType with included SOH char
        message.extend_from_slice(
            Field::Custom {
                tag: 35,
                value: header.msg_type.into(),
            }
            .encode()
            .as_ref(),
        );
        message.put_u8(Self::SOH);

        // Optional header fields
        for field in &header.fields {
            // field with included SOH char.. x=ab\x01
            let mut field_soh = field.encode();
            field_soh.push(Self::SOH);

            // encode the field into the message
            message.extend_from_slice(field_soh.as_ref());
        }

        // Body fields
        for field in &body.fields {
            // field with included SOH char.. x=ab\x01
            let mut field_soh = field.encode();
            field_soh.push(Self::SOH);

            // encode the field into the message
            message.extend_from_slice(field_soh.as_ref());
        }

        message
    }

    /// Comment
    #[must_use]
    fn encode_framing_headers(header: &Header, regular_fields: &BytesMut) -> BytesMut {
        // 3 * 15 (average bytes per field) (BeginString, BodyLength, Checksum)
        let mut message = BytesMut::with_capacity(regular_fields.len() + 45);

        // BeginString with included SOH char
        message.extend_from_slice(
            Field::Custom {
                tag: 8,
                value: header.begin_string.into(),
            }
            .encode()
            .as_ref(),
        );
        message.put_u8(Self::SOH);

        // BodyLength with included SOH char
        message.extend_from_slice(
            Field::Custom {
                tag: 9,
                value: format!("{}", regular_fields.len()).into_bytes(),
            }
            .encode()
            .as_ref(),
        );
        message.put_u8(Self::SOH);

        // append the all the regular fields
        message.extend_from_slice(regular_fields);

        message
    }

    /// Comment
    fn encode_checksum(&mut self, mut message: BytesMut) -> Bytes {
        self.digest.push(&message);

        // Checksum with included SOH char
        let mut checksum_soh = Field::Custom {
            tag: 10,
            value: format!("{}", self.digest.checksum).into_bytes(),
        }
        .encode();
        checksum_soh.push(Self::SOH);

        // encode the Checksum into the message
        message.put(checksum_soh.as_ref());

        message.freeze()
    }
}

// #[cfg(test)]
// mod test {
//     use crate::{
//         encoder::Encoder,
//         message::{
//             Body, Header,
//             field::{
//                 Field,
//                 value::{begin_string::BeginString, msg_type::MsgType},
//             },
//         },
//     };
//
//     #[test]
//     fn test() {
//         let header = Header {
//             begin_string: BeginString::FIX44,
//             msg_type: MsgType::Logon,
//             fields: Vec::new(),
//         };
//
//         let field = Field::Custom {
//             tag: 144,
//             value: Vec::from(b"value144"),
//         };
//
//         let mut body = Body { fields: Vec::new() };
//         body.fields.push(field);
//
//         let mut encoder = Encoder::default();
//         encoder.encode_header(&header);
//         encoder.encode_body(&body);
//
//         assert_eq!(encoder.body_length, 18);
//     }
// }
