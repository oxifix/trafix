//! Implementation of the value module.

use crate::decoder::num::ParseFixInt;

pub mod aliases;
pub mod begin_string;
pub mod msg_type;

/// Trait that abstracts conversion from bytes to values of FIX message fields.
// TODO(nfejzic): this trait might be obsolete if we decide to wrap used types (i.e. newtype
// pattern) and implement traits from std such as [`TryFrom`] instead.
pub(crate) trait FromFixBytes {
    /// Error returned on failed conversion.
    type Error<'lifetime>;

    /// Parses the input and returns an instance of self.
    fn from_fix_bytes(bytes: &[u8]) -> Result<Self, Self::Error<'_>>
    where
        Self: Sized;
}

impl<T> FromFixBytes for T
where
    T: ParseFixInt,
{
    type Error<'unused> = crate::decoder::num::ParseIntError;

    fn from_fix_bytes(bytes: &[u8]) -> Result<Self, Self::Error<'_>>
    where
        Self: Sized,
    {
        Self::parse_fix_int(bytes)
    }
}
