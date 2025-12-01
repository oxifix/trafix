#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub(crate) enum ParseIntError {
    /// Byte slice contained bytes that are not ASCII decimal digits.
    #[error("bytes contain values that are not decimal digits")]
    InvalidDigit,

    /// Byte slice contained integer representation larger than what fits into the primitive type.
    #[error("bytes contain number out of given number literal type's bounds")]
    Overflow,

    /// Byte slice is empty, which is an invalid integer.
    #[error("Unexpected empty input")]
    Empty,
}

/// Helper trait for parsing of integers from byte slices directly. Standard library exposes
/// parsing of integers for [`str`], but not for [`&[u8]`]. That is taken care of with this
/// extension trait.
pub(crate) trait ParseFixInt {
    /// Parses integer from byte slice, or returns a [`ParseIntError`] if byte slice does not
    /// contain valid integer.
    fn parse_fix_int<T>(bytes: T) -> Result<Self, ParseIntError>
    where
        Self: Sized,
        T: AsRef<[u8]>;
}

macro_rules! impl_for {
    ($type:ty, $is_signed:literal) => {
        impl ParseFixInt for $type {
            fn parse_fix_int<T>(bytes: T) -> Result<$type, ParseIntError>
            where
                Self: Sized,
                T: AsRef<[u8]>,
            {
                let mut bytes = bytes.as_ref();
                let mut value: $type = 0;
                let is_negative = if bytes.starts_with(b"-") {
                    if $is_signed {
                        bytes = bytes.get(1..).ok_or(ParseIntError::Empty)?;
                        true
                    } else {
                        return Err(ParseIntError::Overflow);
                    }
                } else {
                    false
                };

                for byte in bytes {
                    value = value.checked_mul(10).ok_or(ParseIntError::Overflow)?;

                    if !byte.is_ascii_digit() {
                        return Err(ParseIntError::InvalidDigit);
                    }

                    let to_add = (byte - b'0')
                        .try_into()
                        .expect("we checked for digits 0..=9");

                    value = if is_negative {
                        value.checked_sub(to_add).ok_or(ParseIntError::Overflow)?
                    } else {
                        value.checked_add(to_add).ok_or(ParseIntError::Overflow)?
                    };
                }

                Ok(value)
            }
        }
    };
}

impl_for!(u8, false);
impl_for!(i8, true);
impl_for!(u16, false);
impl_for!(i16, true);
impl_for!(u32, false);
impl_for!(i32, true);
impl_for!(u64, false);
impl_for!(i64, true);
impl_for!(u128, false);
impl_for!(i128, true);
impl_for!(usize, false);
impl_for!(isize, true);

#[cfg(test)]
mod tests {
    use super::{ParseFixInt as _, ParseIntError};

    #[test]
    fn parse_u8() {
        let value = u8::parse_fix_int(b"123");
        assert!(matches!(value, Ok(123)));

        let res = u8::parse_fix_int(b"001");
        assert!(matches!(res, Ok(1)));

        let res = u8::parse_fix_int(b"000");
        assert!(matches!(res, Ok(0)));

        let res = u8::parse_fix_int(b"256");
        assert!(matches!(res, Err(ParseIntError::Overflow)));

        let res = u8::parse_fix_int(b"1000");
        assert!(matches!(res, Err(ParseIntError::Overflow)));

        let res = u8::parse_fix_int(b"-100");
        assert!(matches!(res, Err(ParseIntError::Overflow)));
    }

    #[test]
    fn parse_i8() {
        let value = i8::parse_fix_int(b"123");
        assert!(matches!(value, Ok(123)));

        let res = i8::parse_fix_int(b"001");
        assert!(matches!(res, Ok(1)));

        let res = i8::parse_fix_int(b"000");
        assert!(matches!(res, Ok(0)));

        let res = i8::parse_fix_int(b"128");
        assert!(matches!(res, Err(ParseIntError::Overflow)));

        let res = i8::parse_fix_int(b"-128");
        assert_eq!(res, Ok(-128));

        let res = i8::parse_fix_int(b"-129");
        assert_eq!(res, Err(ParseIntError::Overflow));

        let res = i8::parse_fix_int(b"1000");
        assert_eq!(res, Err(ParseIntError::Overflow));

        let res = i8::parse_fix_int(b"-100");
        assert_eq!(res, Ok(-100));
    }

    #[test]
    fn non_digits() {
        let res = u8::parse_fix_int(b"abc");
        assert_eq!(res, Err(ParseIntError::InvalidDigit));

        let res = i8::parse_fix_int(b"abc");
        assert_eq!(res, Err(ParseIntError::InvalidDigit));
    }
}
