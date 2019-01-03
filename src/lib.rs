#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate bitflags;
extern crate byteorder;
extern crate eui48;

mod error;
pub use crate::error::*;

/// Packet size traits.
#[macro_use]
pub mod size;
pub use crate::size::Size;

mod packet;
pub use crate::packet::{AsPacket, AsPacketMut, Packet, PacketMut};

/// Buffer abstractions, dynamic buffers and static buffers.
pub mod buffer;
pub use crate::buffer::Buffer;

/// Packet builder abstractions.
pub mod builder;
pub use crate::builder::Builder;

/// Ethernet packet parser and builder.
pub mod ether;

/// IPv4 and IPv6 packet parser and builder.
pub mod ip;

/// ICMP packet parser and builder.
pub mod icmp;

/// TCP packet parser and builder.
pub mod tcp;

/// UDP packet parser and builder.
pub mod udp;
