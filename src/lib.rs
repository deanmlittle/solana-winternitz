#[cfg(not(target_os = "solana"))]
pub mod privkey;
pub mod pubkey;
pub mod signature;
#[cfg(test)]
pub mod tests;

pub const HASH_LENGTH: usize = 28;