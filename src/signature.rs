use core::mem::MaybeUninit;

use solana_nostd_keccak::HASH_LENGTH;

use crate::{hash::Hash, pubkey::WinternitzPubkey};

#[repr(C)]
#[derive(PartialEq)]
pub struct WinternitzSignature {
    data: [[u8; HASH_LENGTH]; 32],
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
                            .map(|byte| format!("{:02x}", byte))
                            .collect::<String>()
                    })
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

impl WinternitzSignature {
    pub fn recover_pubkey<H: Hash>(&self, message: &[u8]) -> WinternitzPubkey {
        let digest = H::hash(message);
        let digest_iter = digest.iter().take(32); // Always take 32 items as that's our fixed outer dimension

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
