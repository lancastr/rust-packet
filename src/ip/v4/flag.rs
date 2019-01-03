bitflags! {
    /// IPv4 packet flags.
    pub struct Flags: u16 {
        /// Do not fragment packets.
        const DONT_FRAGMENT = 0b010;

        /// More fragments are waiting.
        const MORE_FRAGMENTS = 0b100;
    }
}

pub const DONT_FRAGMENT: Flags = Flags::DONT_FRAGMENT;
pub const MORE_FRAGMENTS: Flags = Flags::MORE_FRAGMENTS;
