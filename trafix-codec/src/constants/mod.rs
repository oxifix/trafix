//! Constants used during encoding and decoding.

/// ASCII SOH delimiter (0x01) used as field terminator in FIX messages.
pub(crate) const SOH: u8 = b'\x01';

/// ASCII equals character (=) used as delimiter between tag and value in a single field.
pub(crate) const EQUALS: u8 = b'=';
