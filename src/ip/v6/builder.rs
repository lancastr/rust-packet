use byteorder::{BigEndian, WriteBytesExt};
use std::io::Cursor;

use crate::buffer::{self, Buffer};
use crate::builder::{Builder as Build, Finalization};
use crate::error::*;

/// IPv6 packet builder.
pub struct Builder<B: Buffer = buffer::Dynamic> {
    buffer: B,
    finalizer: Finalization,
}

impl<B: Buffer> Build<B> for Builder<B> {
    fn with(buffer: B) -> Result<Self> {
        Ok(Builder {
            buffer: buffer,
            finalizer: Default::default(),
        })
    }

    fn finalizer(&mut self) -> &mut Finalization {
        &mut self.finalizer
    }

    fn build(self) -> Result<B::Inner> {
        Err(ErrorKind::InvalidPacket.into())
    }
}

impl Default for Builder<buffer::Dynamic> {
    fn default() -> Self {
        Builder::with(buffer::Dynamic::default()).unwrap()
    }
}
