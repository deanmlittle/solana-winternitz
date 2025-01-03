use solana_nostd_keccak::hash;

#[cfg(not(target_os = "solana"))]
use crate::privkey::WinternitzPrivkey;

#[repr(C)]
#[derive(PartialEq)]
pub struct WinternitzPubkey([[u8;crate::HASH_LENGTH];32]);

impl core::fmt::Debug for WinternitzPubkey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WinternitzPubkey")
            .field(&self.0.iter().map(|hash| {
                hash.iter().map(|byte| format!("{:02x}", byte)).collect::<String>()
            }).collect::<Vec<_>>())
            .finish()
    }
}

impl WinternitzPubkey {
    pub fn hash(&self) -> [u8;32] {
        hash(&self.0.concat())
    }
}

impl From<[u8;crate::HASH_LENGTH*32]> for WinternitzPubkey {
    fn from(value: [u8;crate::HASH_LENGTH*32]) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

impl Into<[u8;crate::HASH_LENGTH*32]> for WinternitzPubkey {
    fn into(self) -> [u8;crate::HASH_LENGTH*32] {
        unsafe { core::mem::transmute(self) }
    }
}

impl From<[[u8;crate::HASH_LENGTH];32]> for WinternitzPubkey {
    fn from(seeds: [[u8;crate::HASH_LENGTH];32]) -> Self {
        Self(seeds)
    }
}

#[cfg(not(target_os = "solana"))]
impl From<WinternitzPrivkey> for WinternitzPubkey {
    fn from(key: WinternitzPrivkey) -> Self {
        key.pubkey()
    }
}