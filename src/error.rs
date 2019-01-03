error_chain! {
    errors {
        /// The buffer is too small.
        SmallBuffer { }

        /// The packet is invalid.
        InvalidPacket { }

        /// The value is invalid for the field.
        InvalidValue { }

        /// The value has already been defined.
        AlreadyDefined { }
    }

    foreign_links {
        Io(::std::io::Error);
        Nul(::std::ffi::NulError);
    }
}
