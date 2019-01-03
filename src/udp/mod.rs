mod packet;
pub use self::packet::Packet;

mod builder;
pub use self::builder::Builder;

use crate::ip;
use crate::ip::Protocol;

/// Calculate the checksum for a UDP packet.
///
/// # Note
///
/// Since the checksum for UDP packets includes a pseudo-header based on the
/// enclosing IP packet, one has to be given.
pub fn checksum<B: AsRef<[u8]>>(ip: &ip::Packet<B>, buffer: &[u8]) -> u16 {
    use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
    use std::io::Cursor;

    let mut prefix = [0u8; 40];
    match *ip {
        ip::Packet::V4(ref packet) => {
            prefix[0..4].copy_from_slice(&packet.source().octets());
            prefix[4..8].copy_from_slice(&packet.destination().octets());

            prefix[9] = Protocol::Udp.into();
            Cursor::new(&mut prefix[10..]).write_u16::<BigEndian>(buffer.len() as u16).unwrap();
        }

        ip::Packet::V6(ref _packet) => {
            unimplemented!();
        }
    };

    let mut result = 0xffffu32;
    let mut buffer = Cursor::new(buffer);
    let mut prefix = match *ip {
        ip::Packet::V4(_) => Cursor::new(&prefix[0..12]),

        ip::Packet::V6(_) => Cursor::new(&prefix[0..40]),
    };

    while let Ok(value) = prefix.read_u16::<BigEndian>() {
        result += u32::from(value);

        if result > 0xffff {
            result -= 0xffff;
        }
    }

    while let Ok(value) = buffer.read_u16::<BigEndian>() {
        // Skip checksum field.
        if buffer.position() == 8 {
            continue;
        }

        result += u32::from(value);

        if result > 0xffff {
            result -= 0xffff;
        }
    }

    if let Ok(value) = buffer.read_u8() {
        let last = u32::from(value) << 8;
        result += last;

        if result > 0xffff {
            result -= 0xffff;
        }
    }

    !result as u16
}
