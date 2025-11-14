//! Implementation of the message module.

pub mod field;

use bytes::Bytes;

use crate::{
    encoder,
    message::field::{
        Field,
        value::{begin_string::BeginString, msg_type::MsgType},
    },
};

/// Represents the header section of a FIX message.
///
/// The header always contains the protocol [`BeginString`] (tag 8)
/// and the message type [`MsgType`] (tag 35), and may include
/// additional session or routing fields.
pub struct Header {
    /// The `BeginString` identifying the FIX protocol version.
    #[allow(dead_code)]
    pub(crate) begin_string: BeginString,

    /// The `MsgType` indicating the business purpose of the message (message type).
    #[allow(dead_code)]
    pub(crate) msg_type: MsgType,

    /// Optional additional header fields.
    pub(crate) fields: Vec<Field>,
}

/// Comment
#[derive(Default)]
pub struct Body {
    /// Comment
    pub(crate) fields: Vec<Field>,
}

/// Represents a complete owned, structured FIX message composed of a header and body.
///
/// The header holds protocol and session metadata, while the body
/// carries message-specific fields defined by the message type.
pub struct Message {
    /// The message header containing version, type, and optional routing fields.
    header: Header,

    /// The message body forming the message business content.
    body: Body,
}

impl Message {
    /// Creates a new [`MessageBuilder`] initialized with the required
    /// [`BeginString`] and [`MsgType`] header fields.
    ///
    /// Example usage:
    /// ```
    /// use trafix_codec::message::{
    ///     Message,
    ///     field::{
    ///         Field,
    ///         value::{begin_string::BeginString, msg_type::MsgType},
    ///     },
    /// };
    ///
    /// let builder = Message::builder(BeginString::FIX44, MsgType::Logon);
    /// ```
    #[must_use]
    pub fn builder(begin_string: BeginString, msg_type: MsgType) -> MessageBuilder<false> {
        let header = Header {
            begin_string,
            msg_type,
            fields: Vec::new(),
        };

        MessageBuilder {
            inner: Message {
                header,
                body: Body::default(),
            },
        }
    }

    /// Comment
    #[must_use]
    pub fn encode(self) -> Bytes {
        encoder::encode(&self.header, &self.body)
    }
}

/// Generic builder for constructing [`Message`] instances.
///
/// The builder supports chaining calls to add header or body fields.
/// Type-state (`IS_INIT`) tracks whether at least one body field was added,
/// allowing [`MessageBuilder::build()`] to only be available after initialization.
pub struct MessageBuilder<const IS_INIT: bool> {
    /// The message being constructed.
    inner: Message,
}

impl<const IS_INIT: bool> MessageBuilder<IS_INIT> {
    /// Adds a field to the message header.
    #[must_use]
    pub fn with_header(mut self, field: Field) -> Self {
        self.inner.header.fields.push(field);

        self
    }

    /// Adds a field to the message body.
    ///
    /// Each call appends a new [`Field`] in order of insertion.
    /// Once at least one field has been added, the builder transitions
    /// to an initialized state, enabling [`build`](Self::build).
    #[must_use]
    pub fn with_field(mut self, field: Field) -> MessageBuilder<true> {
        self.inner.body.fields.push(field);

        MessageBuilder { inner: self.inner }
    }
}

impl MessageBuilder<true> {
    /// Finalizes and returns the fully constructed [`Message`].
    ///
    /// Example usage:
    /// ```
    /// use trafix_codec::message::{
    ///     Message,
    ///     field::{
    ///         Field,
    ///         value::{begin_string::BeginString, msg_type::MsgType},
    ///     },
    /// };
    ///
    /// let msg = Message::builder(BeginString::FIX44, MsgType::Logout)
    ///     .with_field(Field::Custom { tag: 58, value: b"Bye".to_vec() })
    ///     .build();
    /// ```
    #[must_use]
    pub fn build(self) -> Message {
        self.inner
    }
}

#[cfg(test)]
mod test {
    use crate::message::{
        Message,
        field::{
            Field,
            value::{begin_string::BeginString, msg_type::MsgType},
        },
    };

    #[test]
    fn basic_builder() {
        let builder = Message::builder(BeginString::FIX44, MsgType::Logon);

        // header
        assert_eq!(builder.inner.header.begin_string, BeginString::FIX44);
        assert_eq!(builder.inner.header.msg_type, MsgType::Logon);

        // body
        assert_eq!(builder.inner.body.fields.len(), 0);
    }

    #[test]
    fn simple_message() {
        let builder = Message::builder(BeginString::FIX44, MsgType::Logout);

        let custom_header_field = Field::Custom {
            tag: 22,
            value: b"custom_header_field".to_vec(),
        };

        let custom_body_field1 = Field::Custom {
            tag: 40000,
            value: b"custom_body_field1".to_vec(),
        };

        let custom_body_field2 = Field::Custom {
            tag: 50000,
            value: b"custom_body_field2".to_vec(),
        };

        let msg = builder
            .with_header(custom_header_field.clone())
            .with_field(custom_body_field1.clone())
            .with_field(custom_body_field2.clone())
            .build();

        // auto-header
        assert_eq!(msg.header.begin_string, BeginString::FIX44);
        assert_eq!(msg.header.msg_type, MsgType::Logout);

        // custom header
        assert_eq!(msg.header.fields.clone().len(), 1);
        assert_eq!(msg.header.fields[0], custom_header_field);

        // body
        assert_eq!(msg.body.fields.len(), 2);
        assert_eq!(msg.body.fields[0], custom_body_field1);
        assert_eq!(msg.body.fields[1], custom_body_field2);
    }
}
