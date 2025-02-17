use core::fmt::Write;
use core::mem::MaybeUninit;
use crate::hash::Hash;
use crate::pubkey::WinternitzPubkey;

use solana_nostd_keccak::HASH_LENGTH;

#[repr(C)]
#[derive(PartialEq)]
pub struct WinternitzSignature {
    pub data: [[u8; HASH_LENGTH]; 32],
}

impl From<[[u8; HASH_LENGTH]; 32]> for WinternitzSignature {
    fn from(seeds: [[u8; HASH_LENGTH]; 32]) -> Self {
        Self { data: seeds }
    }
}

impl From<WinternitzSignature> for [[u8; HASH_LENGTH]; 32] {
    fn from(sig: WinternitzSignature) -> Self {
        sig.data
    }
}

impl core::fmt::Debug for WinternitzSignature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WinternitzSignature")
            .field(
                &self
                    .data
                    .iter()
                    .map(|hash| {
                        hash.iter()
                            .fold(String::with_capacity(HASH_LENGTH * 2), |mut acc, byte| {
                                write!(acc, "{:02x}", byte).unwrap();
                                acc
                            })
                    })
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

impl WinternitzSignature {
    pub fn recover_pubkey<H: Hash>(&self, message: &[u8]) -> WinternitzPubkey {
        let digest = H::hash(message);
        self.recover_pubkey_prehashed::<H>(&digest)
    }

    #[inline(always)]
    pub fn recover_pubkey_prehashed<H: Hash>(&self, hash: &[u8;32]) -> WinternitzPubkey {
        let digest_iter = hash.iter().take(32); // Always take 32 items as that's our fixed outer dimension

        let mut signature = MaybeUninit::<[[u8; HASH_LENGTH]; 32]>::uninit();
        let signature_ptr = signature.as_mut_ptr() as *mut [u8; HASH_LENGTH];

        digest_iter
            .zip(self.data.iter())
            .enumerate()
            .for_each(|(i, (n, seed))| {
                let mut hashed_value = *seed;
                for _ in 0..*n {
                    hashed_value = H::hash(&hashed_value);
                }
                unsafe {
                    signature_ptr.add(i).write(hashed_value);
                }
            });

        WinternitzPubkey::from(unsafe { signature.assume_init() })
    }

    pub fn verify<H: Hash>(&self, message: &[u8], pubkey: &WinternitzPubkey) -> bool {
        self.recover_pubkey::<H>(message).eq(pubkey)
    }
}
