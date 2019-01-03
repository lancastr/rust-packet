mod kind;
pub use self::kind::Kind;

/// ICMP codes.
pub mod code;

mod packet;
pub use self::packet::Packet;

mod builder;
pub use self::builder::Builder;

/// Echo Request/Reply.
pub mod echo;

/// Information Request/Reply.
pub mod information;

/// Parmeter Problem.
pub mod parameter_problem;

/// Source Quench, Destination Unreachable and Time Exceeded.
pub mod previous;

/// Redirect Message.
pub mod redirect_message;

/// Timestamp Request/Reply.
pub mod timestamp;

/// Calculate the checksum for an ICMP packet.
pub fn checksum(buffer: &[u8]) -> u16 {
    use byteorder::{BigEndian, ReadBytesExt};
    use std::io::Cursor;

    let mut result = 0xffffu32;
    let mut buffer = Cursor::new(buffer);

    while let Ok(value) = buffer.read_u16::<BigEndian>() {
        // Skip checksum field.
        if buffer.position() == 4 {
            continue;
        }

        result += u32::from(value);

        if result > 0xffff {
            result -= 0xffff;
        }
    }

    !result as u16
}
