/// IPv4 flags.
pub mod flag;
pub use self::flag::Flags;

/// IPv4 option parser and builder.
pub mod option;
pub use self::option::Option;

mod packet;
pub use self::packet::Packet;

mod builder;
pub use self::builder::Builder;

/// Calculate the checksum for an IPv4 packet.
pub fn checksum(buffer: &[u8]) -> u16 {
    use byteorder::{BigEndian, ReadBytesExt};
    use std::io::Cursor;

    let mut result = 0xffffu32;
    let mut buffer = Cursor::new(buffer);

    while let Ok(value) = buffer.read_u16::<BigEndian>() {
        // Skip checksum field.
        if buffer.position() == 12 {
            continue;
        }

        result += u32::from(value);

        if result > 0xffff {
            result -= 0xffff;
        }
    }

    !result as u16
}
