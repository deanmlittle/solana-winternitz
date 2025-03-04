pub trait WinternitzHash {
    fn hash(msg: &[u8]) -> [u8; 32];
    fn hashd(msg: &[u8]) -> [u8; 32];
    fn hashv(msg: &[&[u8]]) -> [u8; 32];
    fn hash_pair(a: &[u8], b: &[u8]) -> [u8; 32];
}

pub struct WinternitzKeccak;

impl WinternitzHash for WinternitzKeccak {
    #[inline(always)]
    fn hash(msg: &[u8]) -> [u8; 32] {
        solana_nostd_keccak::hash(msg)
    }

    #[inline(always)]
    fn hashd(msg: &[u8]) -> [u8; 32] {
        Self::hash(&Self::hash(msg))
    }

    #[inline(always)]
    fn hashv(msg: &[&[u8]]) -> [u8; 32] {
        solana_nostd_keccak::hashv(msg)
    }

    #[inline(always)]
    fn hash_pair(a: &[u8], b: &[u8]) -> [u8; 32] {
        Self::hashv(&[a, b])
    }
}

pub struct WinternitzSha256;

impl WinternitzHash for WinternitzSha256 {
    #[inline(always)]
    fn hash(msg: &[u8]) -> [u8; 32] {
        solana_nostd_sha256::hash(msg)
    }

    #[inline(always)]
    fn hashd(msg: &[u8]) -> [u8; 32] {
        Self::hash(&Self::hash(msg))
    }

    #[inline(always)]
    fn hashv(msg: &[&[u8]]) -> [u8; 32] {
        solana_nostd_sha256::hashv(msg)
    }

    #[inline(always)]
    fn hash_pair(a: &[u8], b: &[u8]) -> [u8; 32] {
        Self::hashv(&[a, b])
    }
}
