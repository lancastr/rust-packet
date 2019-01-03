use byteorder::{BigEndian, WriteBytesExt};
use std::io::Cursor;

use crate::buffer::{self, Buffer};
use crate::builder::{Builder as Build, Finalization};
use crate::error::*;
use crate::icmp::builder;
use crate::icmp::timestamp::Packet;
use crate::icmp::Kind;
use crate::packet::{AsPacket, AsPacketMut};

/// Information Request/Reply builder.
#[derive(Debug)]
pub struct Builder<B: Buffer = buffer::Dynamic> {
    buffer: B,
    finalizer: Finalization,

    kind: bool,
}

impl<B: Buffer> Build<B> for Builder<B> {
    fn with(mut buffer: B) -> Result<Self> {
        buffer.next(20)?;

        Ok(Builder {
            buffer: buffer,
            finalizer: Default::default(),

            kind: false,
        })
    }

    fn finalizer(&mut self) -> &mut Finalization {
        &mut self.finalizer
    }

    fn build(mut self) -> Result<B::Inner> {
        if !self.kind {
            return Err(ErrorKind::InvalidPacket.into());
        }

        builder::prepare(&mut self.finalizer, &self.buffer);

        let mut buffer = self.buffer.into_inner();
        self.finalizer.finalize(buffer.as_mut())?;
        Ok(buffer)
    }
}

impl Default for Builder<buffer::Dynamic> {
    fn default() -> Self {
        Builder::with(buffer::Dynamic::default()).unwrap()
    }
}

impl<'a, B: Buffer> AsPacket<'a, Packet<&'a [u8]>> for Builder<B> {
    fn as_packet(&self) -> Result<Packet<&[u8]>> {
        Packet::new(self.buffer.data())
    }
}

impl<'a, B: Buffer> AsPacketMut<'a, Packet<&'a mut [u8]>> for Builder<B> {
    fn as_packet_mut(&mut self) -> Result<Packet<&mut [u8]>> {
        Packet::new(self.buffer.data_mut())
    }
}

impl<B: Buffer> Builder<B> {
    /// Make it a request.
    pub fn request(mut self) -> Result<Self> {
        self.kind = true;
        self.buffer.data_mut()[0] = Kind::TimestampRequest.into();

        Ok(self)
    }

    /// Make it a reply.
    pub fn reply(mut self) -> Result<Self> {
        self.kind = true;
        self.buffer.data_mut()[0] = Kind::TimestampReply.into();

        Ok(self)
    }

    /// Packet identifier.
    pub fn identifier(mut self, value: u16) -> Result<Self> {
        Cursor::new(&mut self.buffer.data_mut()[4..]).write_u16::<BigEndian>(value)?;

        Ok(self)
    }

    /// Packet sequence.
    pub fn sequence(mut self, value: u16) -> Result<Self> {
        Cursor::new(&mut self.buffer.data_mut()[6..]).write_u16::<BigEndian>(value)?;

        Ok(self)
    }

    /// Creation timestamp.
    pub fn originate(mut self, value: u32) -> Result<Self> {
        Cursor::new(&mut self.buffer.data_mut()[8..]).write_u32::<BigEndian>(value)?;

        Ok(self)
    }

    /// Reception timestamp.
    pub fn receive(mut self, value: u32) -> Result<Self> {
        Cursor::new(&mut self.buffer.data_mut()[12..]).write_u32::<BigEndian>(value)?;

        Ok(self)
    }

    /// Transmission timestamp.
    pub fn transmit(mut self, value: u32) -> Result<Self> {
        Cursor::new(&mut self.buffer.data_mut()[16..]).write_u32::<BigEndian>(value)?;

        Ok(self)
    }
}
