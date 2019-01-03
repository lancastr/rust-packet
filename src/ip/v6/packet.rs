use byteorder::{BigEndian, ReadBytesExt};
use std::fmt;
use std::net::Ipv4Addr;

use crate::error::*;
use crate::ip::v4::checksum;
use crate::ip::v4::option;
use crate::ip::v4::Flags;
use crate::ip::Protocol;
use crate::packet::{AsPacket, AsPacketMut, Packet as P, PacketMut as PM};
use crate::size;

/// IPv6 packet parser.
#[derive(Clone)]
pub struct Packet<B> {
    buffer: B,
}

sized!(Packet,
	header {
		min:  0,
		max:  0,
		size: 0,
	}

	payload {
		min:  0,
		max:  0,
		size: 0,
	});

impl<B: AsRef<[u8]>> fmt::Debug for Packet<B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ip::v6::Packet").finish()
    }
}

impl<B: AsRef<[u8]>> Packet<B> {
    /// Create an IPv6 packet without checking the buffer.
    pub fn unchecked(buffer: B) -> Packet<B> {
        Packet { buffer }
    }

    /// Parse an IPv6 packet without checking the payload.
    pub fn no_payload(buffer: B) -> Result<Packet<B>> {
        use crate::size::header::Min;

        let packet = Packet::unchecked(buffer);

        if packet.buffer.as_ref().len() < Self::min() {
            return Err(ErrorKind::SmallBuffer.into());
        }

        if packet.buffer.as_ref()[0] >> 4 != 6 {
            return Err(ErrorKind::InvalidPacket.into());
        }

        Err(ErrorKind::InvalidPacket.into())
    }

    /// Parse an IPv6 packet, checking the buffer contents are correct.
    pub fn new(buffer: B) -> Result<Packet<B>> {
        Packet::no_payload(buffer)
    }
}

impl<B: AsRef<[u8]>> Packet<B> {
    /// Convert the packet to its owned version.
    ///
    /// # Notes
    ///
    /// It would be nice if `ToOwned` could be implemented, but `Packet` already
    /// implements `Clone` and the impl would conflict.
    pub fn to_owned(&self) -> Packet<Vec<u8>> {
        Packet::unchecked(self.buffer.as_ref().to_vec())
    }
}

impl<B: AsRef<[u8]>> AsRef<[u8]> for Packet<B> {
    fn as_ref(&self) -> &[u8] {
        &[]
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> AsMut<[u8]> for Packet<B> {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut []
    }
}

impl<'a, B: AsRef<[u8]>> AsPacket<'a, Packet<&'a [u8]>> for B {
    fn as_packet(&self) -> Result<Packet<&[u8]>> {
        Packet::new(self.as_ref())
    }
}

impl<'a, B: AsRef<[u8]> + AsMut<[u8]>> AsPacketMut<'a, Packet<&'a mut [u8]>> for B {
    fn as_packet_mut(&mut self) -> Result<Packet<&mut [u8]>> {
        Packet::new(self.as_mut())
    }
}

impl<B: AsRef<[u8]>> P for Packet<B> {
    fn split(&self) -> (&[u8], &[u8]) {
        (&[], &[])
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> PM for Packet<B> {
    fn split_mut(&mut self) -> (&mut [u8], &mut [u8]) {
        (&mut [], &mut [])
    }
}
