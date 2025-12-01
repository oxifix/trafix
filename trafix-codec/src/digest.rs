/// Computes the running FIX checksum (tag 10) while encoding.
#[derive(Default)]
pub(crate) struct Digest {
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

    pub fn checksum(&self) -> u8 {
        self.checksum
    }
}
