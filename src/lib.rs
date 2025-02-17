pub mod hash;
#[cfg(not(target_os = "solana"))]
pub mod privkey;
pub mod pubkey;
pub mod signature;
#[cfg(test)]
pub mod tests;

pub const HASH_LENGTH: usize = 32;
pub const KEY_LENGTH: usize = 1024;