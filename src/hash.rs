use crate::HASH_LENGTH;

pub trait Hash {
    fn hash(data: &[u8]) -> [u8; HASH_LENGTH];
    fn hashv(data: &[&[u8]]) -> [u8; HASH_LENGTH];
}

pub struct Sha256;
impl Hash for Sha256 {
    fn hash(data: &[u8]) -> [u8; HASH_LENGTH] {
        solana_nostd_sha256::hash(data)
    }

    fn hashv(data: &[&[u8]]) -> [u8; HASH_LENGTH] {
        solana_nostd_sha256::hashv(data)
    }
}

pub struct Keccak;
impl Hash for Keccak {
    fn hash(data: &[u8]) -> [u8; HASH_LENGTH] {
        solana_nostd_keccak::hash(data)
    }

    fn hashv(data: &[&[u8]]) -> [u8; HASH_LENGTH] {
        solana_nostd_keccak::hashv(data)
    }
}
