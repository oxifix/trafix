//! Comment

/// Comment
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BeginString {
    /// Comment
    FIX44,
}

impl From<BeginString> for Vec<u8> {
    fn from(val: BeginString) -> Self {
        match val {
            BeginString::FIX44 => b"FIX.4.4".to_vec(),
        }
    }
}
