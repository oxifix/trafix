//! Implementation of a lightweight, stateful FIX checksum (Digest) calculator.

/// The [`Digest`] maintains a running checksum by performing modulo-256 addition over all
/// processed bytes, exactly as defined by the FIX checksum algorithm. This is typically used while
/// encoding and decoding FIX messages.
///
/// # Example
///
/// ```ignore
/// let mut digest = Digest::default();
/// digest.push(&[1, 2, 3]);
/// let checksum = digest.checksum();
///
/// // (1 + 2 + 3) % 256 = 6
/// assert_eq!(checksum, 6);
///
/// // (1 + 2 + 3 + 251) % 256 = 257 % 256 = 1
/// digest.push(251);
/// assert_eq!(checksum, 1);
/// ```
#[derive(Default)]
pub(crate) struct Digest {
    /// Accumulated checksum value computed as the modulo-256 sum of all bytes
    /// processed, per the FIX protocol checksum definition.
    checksum: u8,
}

impl Digest {
    /// Updates the running checksum using the contents of a [`BytesMut`].
    ///
    /// This performs modulo-256 addition across all bytes, matching the FIX
    /// checksum algorithm.
    pub fn push(&mut self, input: &impl AsRef<[u8]>) {
        for &b in input.as_ref() {
            self.checksum = self.checksum.wrapping_add(b);
        }
    }

    /// Returns the calculated checksum of bytes pushed so far.
    pub fn checksum(&self) -> u8 {
        self.checksum
    }
}
