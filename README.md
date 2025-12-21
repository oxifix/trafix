# trafix

A Rust-based implementation of the Financial Information eXchange (FIX) engine featuring 2 subcrates: trafix-codec: Low-level library for high-performance parsing, encoding, and validation of FIX messages. trafix-engine: Full-featured FIX engine with session management, transports, and persistence built on top of trafix-codec.
# Project Structure

This project is organized as a multirepo, consisting of three separate crates:

3. **trafix** — an umbrella crate wrapping the 2 below.
1. **trafix-codec** — a low-level library for high-performance parsing, encoding, and validation of FIX messages.  
2. **trafix-engine** — a full-featured FIX engine with session management, transports, and persistence built on top of `trafix-codec`.

## trafix-codec

**trafix-codec** is the foundation crate: it provides control over FIX messages manipulation.

### Key Features

- **Message creation**: Version-agnostic FIX message creation using a safe builder.
  - Generic builder for constructing Message instances.
  - Allowing of custom fields addition.
  - Forbidding of message building until all the required fields are provided.

- **Message decoding**: Version-agnostic FIX message decoding with a focus on correctness and performance.
  - SOH-based frame parsing (`tag=value\x01`)
  - Support for FIX session-level messages
  - Validation of `BeginString (8)`, `BodyLength (9)`, and `CheckSum (10)`

- **Message encoding**: Deterministic FIX message serialization with automatic framing.
  - Correct handling of `BeginString (8)`, `BodyLength (9)`, and `CheckSum (10)`
  - Supports arbitrary field order while enforcing FIX wire rules
  - Efficient buffer-based encoding (`Bytes` / `BytesMut`)

---

### Getting Started

Add `trafix-codec` to your `Cargo.toml`:

```toml
[dependencies]
trafix-codec = "0.1.0"
```

---

### Example: full usage

```rust
use trafix_codec::message::{
    Message,
    field::{
        Field,
        value::{begin_string::BeginString, msg_type::MsgType},
    },
};

fn main() {
    // Create the FIX message builder
    let builder = Message::builder(BeginString::FIX44, MsgType::Logout);

    // Create a custom field (to be added in header)
    let custom_header_field = Field::Custom {
        tag: 22,
        value: b"custom_header_field".to_vec(),
    };

    // Create a custom field (to be added in body)
    let custom_body_field1 = Field::Custom {
        tag: 40000,
        value: b"custom_body_field1".to_vec(),
    };

    // Create a custom field (to be added in body)
    let custom_body_field2 = Field::Custom {
        tag: 50000,
        value: b"custom_body_field2".to_vec(),
    };

    // build the message after adding the needed fields.
    let message = builder
        .with_header(custom_header_field.clone())
        .with_field(custom_body_field1.clone())
        .with_field(custom_body_field2.clone())
        .build();

    // encode the message into bytes
    let encoded_message = message.encode();
    println!("{:?}", encoded_message);

    // decoding example; decoing and encoding a message results in the same bytes output.

    // Raw FIX message
    let input = "8=FIX.4.4\x019=148\x0135=A\x0134=1080\x0149=TESTBUY1\x0152=20180920-18:14:19.508\x0156=TESTSELL1\x0111=636730640278898634\x0115=USD\x0121=2\x0138=7000\x0140=1\x0154=1\x0155=MSFT\x0160=20180920-18:14:19.492\x0110=089\x01";

    // decode the input
    let message2 = Message::decode(input).expect("Input is valid");

    // encode the message into bytes
    let encoded_message2 = message2.encode();
    println!("{:?}", encoded_message2);
}
```
