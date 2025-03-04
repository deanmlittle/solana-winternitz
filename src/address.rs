use arraystring::{typenum::U44, ArrayString};
use core::fmt::Write;

pub struct WinternitzAddress(pub [u8; 32]);

impl From<[u8; 32]> for WinternitzAddress {
    fn from(value: [u8; 32]) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

impl TryFrom<&str> for WinternitzAddress {
    type Error = fd_bs58::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(WinternitzAddress::from(fd_bs58::decode_32(value)?))
    }
}

impl core::fmt::Debug for WinternitzAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WinternitzAddress")
            .field(&fd_bs58::encode_32(self.0))
            .finish()
    }
}

impl core::fmt::Display for WinternitzAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&fd_bs58::encode_32(self.0))
    }
}

impl WinternitzAddress {
    pub fn to_array_string(&self) -> ArrayString<U44> {
        let mut array_str = ArrayString::<U44>::new();
        // Write the encoded address to the array string
        write!(&mut array_str, "{}", self).expect("Writing to ArrayString failed");
        array_str
    }
}
