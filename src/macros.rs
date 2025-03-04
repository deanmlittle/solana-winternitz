#[macro_export]
macro_rules! winternitz_debug {
    ($type:ty, $name:expr) => {
        // Debug implementation (for {:?})
        impl core::fmt::Debug for $type {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                writeln!(f, "{} [", $name)?;
                for (i, scalar) in self.0.iter().enumerate() {
                    // Use the hex crate to encode the scalar to a hex string.
                    let hex_str = hex::encode(scalar);
                    write!(f, "  {}: {}", i, hex_str)?;
                    if i < self.0.len() - 1 {
                        writeln!(f, ",")?;
                    } else {
                        writeln!(f)?;
                    }
                }
                write!(f, "]")
            }
        }

        // Display implementation (for {})
        impl core::fmt::Display for $type {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{} [", $name)?;
                for (i, scalar) in self.0.iter().enumerate() {
                    let hex_str = hex::encode(scalar);
                    if i < self.0.len() - 1 {
                        write!(f, "{}: {}, ", i, hex_str)?;
                    } else {
                        write!(f, "{}: {}", i, hex_str)?;
                    }
                }
                write!(f, "]")
            }
        }
    };
}
