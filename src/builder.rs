use std::fmt;

use crate::buffer::Buffer;
use crate::error::*;

/// A finalizer used by builders to complete building the packet, this is
/// usually used to calculate the checksum and update length fields after the
/// whole packet has been created.
pub trait Finalizer {
    /// Run the finalizer on the given buffer.
    fn finalize(self: Box<Self>, buffer: &mut [u8]) -> Result<()>;
}

impl<F: FnOnce(&mut [u8]) -> Result<()>> Finalizer for F {
    fn finalize(self: Box<F>, buffer: &mut [u8]) -> Result<()> {
        let f = *self;
        f(buffer)
    }
}

/// Takes care of grouping finalizers through the builder chain.
pub struct Finalization(Vec<Box<Finalizer>>);

impl Default for Finalization {
    fn default() -> Self {
        Finalization(Vec::new())
    }
}

impl fmt::Debug for Finalization {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("builder::Finalization").field("length", &self.0.len()).finish()
    }
}

impl Finalization {
    /// Add a new finalizer.
    pub fn add<F: FnOnce(&mut [u8]) -> Result<()> + 'static>(&mut self, finalizer: F) {
        self.0.push(Box::new(finalizer));
    }

    /// Add a serie of finalizers.
    pub fn extend<I: IntoIterator<Item = Box<Finalizer>>>(&mut self, finalizers: I) {
        self.0.extend(finalizers.into_iter());
    }

    /// Finalize a buffer.
    pub fn finalize(self, buffer: &mut [u8]) -> Result<()> {
        for finalizer in self.0.into_iter().rev() {
            finalizer.finalize(buffer)?;
        }

        Ok(())
    }
}

impl IntoIterator for Finalization {
    type Item = Box<Finalizer>;
    type IntoIter = ::std::vec::IntoIter<Box<Finalizer>>;

    fn into_iter(self) -> ::std::vec::IntoIter<Box<Finalizer>> {
        self.0.into_iter()
    }
}

impl Into<Vec<Box<Finalizer>>> for Finalization {
    fn into(self) -> Vec<Box<Finalizer>> {
        self.0
    }
}

/// A packet `Builder`.
pub trait Builder<B: Buffer> {
    /// Create a new packet `Builder` with the given buffer.
    fn with(buffer: B) -> Result<Self>
    where
        Self: Sized;

    /// Access the finalizers.
    fn finalizer(&mut self) -> &mut Finalization;

    /// Build the packet.
    fn build(self) -> Result<B::Inner>;
}
