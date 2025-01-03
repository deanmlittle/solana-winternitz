use core::mem::MaybeUninit;
use rand::Rng;
use solana_nostd_keccak::hash;

use crate::{pubkey::WinternitzPubkey, signature::WinternitzSignature};

#[repr(C)]
pub struct WinternitzPrivkey([[u8;crate::HASH_LENGTH];32]);

impl core::fmt::Debug for WinternitzPrivkey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WinternitzPrivkey")
            .field(&self.0.iter().map(|hash| {
                hash.iter().map(|byte| format!("{:02x}", byte)).collect::<String>()
            }).collect::<Vec<_>>())
            .finish()
    }
}

impl Default for WinternitzPrivkey {
    fn default() -> Self {
        Self::generate()
    }
}

impl From<[u8;crate::HASH_LENGTH*32]> for WinternitzPrivkey {
    fn from(value: [u8;crate::HASH_LENGTH*32]) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

impl From<[[u8;crate::HASH_LENGTH];32]> for WinternitzPrivkey {
    fn from(seeds: [[u8;crate::HASH_LENGTH];32]) -> Self {
        Self(seeds)
    }
}

impl WinternitzPrivkey {
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();
        // Generate 32 random seed bytes
        let seeds: [[u8;crate::HASH_LENGTH]; 32] = rng.gen();

        Self(seeds)
    }

    pub fn pubkey(&self) -> WinternitzPubkey {
        WinternitzPubkey::from(self.0.map(|seed| {
            let mut hashed_value = seed; // Start with each private key scalar
            for _ in 0..256 { // Hash 256 times
                hashed_value = *hash(&hashed_value).split_first_chunk::< { crate::HASH_LENGTH }>().expect("Invalid length").0;
            }
            hashed_value
        }))
    }

    pub fn sign(&self, message: &[u8]) -> WinternitzSignature {
        let digest = hash(message);

        let mut signature = MaybeUninit::<[[u8;crate::HASH_LENGTH];32]>::uninit();
        let signature_ptr = signature.as_mut_ptr() as *mut [u8;crate::HASH_LENGTH];

        digest.iter().zip(self.0.iter()).enumerate().for_each(|(i,(n,seed))| {
            let mut hashed_value = *seed; // Start with each private key scalar
            for _ in 0..(256usize - *n as usize) {
                hashed_value = *hash(&hashed_value).split_first_chunk::<{ crate::HASH_LENGTH }>().expect("Invalid length").0;
            }
            unsafe {
                signature_ptr.add(i).write(hashed_value);
            }
        });
        
        WinternitzSignature::from(unsafe { signature.assume_init() })
    }
}