//! Comment

pub mod field;

use crate::message::field::{
    Field,
    value::{begin_string::BeginString, msg_type::MsgType},
};

/// Comment
pub struct Header {
    /// Comment
    begin_string: BeginString,

    /// Comment
    msg_type: MsgType,

    /// Comment
    fields: Option<Vec<Field>>,
}

/// Comment
pub struct Message {
    /// Comment
    header: Header,

    /// Comment
    body: Vec<Field>,
}

impl Message {
    /// Comment
    #[must_use]
    pub fn builder(begin_string: BeginString, msg_type: MsgType) -> MessageBuilder<false> {
        let header = Header {
            begin_string,
            msg_type,
            fields: None,
        };

        MessageBuilder {
            inner: Message {
                header,
                body: Vec::new(),
            },
        }
    }
}

/// Comment
pub struct MessageBuilder<const IS_INIT: bool> {
    /// Comment
    inner: Message,
}

impl<const IS_INIT: bool> MessageBuilder<IS_INIT> {
    /// Comment
    #[must_use]
    pub fn with_header(mut self, field: Field) -> Self {
        if let Some(fields) = &mut self.inner.header.fields {
            fields.push(field);
        } else {
            self.inner.header.fields = Some(Vec::new());
            return self.with_header(field);
        }

        self
    }

    /// Comment
    #[must_use]
    pub fn with_field(mut self, field: Field) -> MessageBuilder<true> {
        self.inner.body.push(field);

        MessageBuilder { inner: self.inner }
    }
}

impl MessageBuilder<true> {
    /// Comment
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
        assert_eq!(builder.inner.body.len(), 0);
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
        assert_eq!(msg.header.fields.clone().unwrap().len(), 1);
        assert_eq!(msg.header.fields.unwrap()[0], custom_header_field);

        // body
        assert_eq!(msg.body.len(), 2);
        assert_eq!(msg.body[0], custom_body_field1);
        assert_eq!(msg.body[1], custom_body_field2);
    }
}
