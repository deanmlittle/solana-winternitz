use std::mem::MaybeUninit;

use solana_nostd_keccak::hash;

use crate::pubkey::WinternitzPubkey;

#[repr(C)]
#[derive(PartialEq)]
pub struct WinternitzSignature([[u8;crate::HASH_LENGTH];32]);

impl From<[[u8;crate::HASH_LENGTH];32]> for WinternitzSignature {
    fn from(seeds: [[u8;crate::HASH_LENGTH];32]) -> Self {
        Self(seeds)
    }
}

impl From<[u8;crate::HASH_LENGTH*32]> for WinternitzSignature {
    fn from(value: [u8;crate::HASH_LENGTH*32]) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

impl Into<[u8;crate::HASH_LENGTH*32]> for WinternitzSignature {
    fn into(self) -> [u8;crate::HASH_LENGTH*32] {
        unsafe { core::mem::transmute(self) }
    }
}

impl core::fmt::Debug for WinternitzSignature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WinternitzSignature")
            .field(&self.0.iter().map(|hash| {
                hash.iter().map(|byte| format!("{:02x}", byte)).collect::<String>()
            }).collect::<Vec<_>>())
            .finish()
    }
}

impl WinternitzSignature {
    pub fn recover_pubkey(&self, message: &[u8]) -> WinternitzPubkey {
        let digest = hash(message);
        
        let mut signature = MaybeUninit::<[[u8;crate::HASH_LENGTH];32]>::uninit();
        let signature_ptr = signature.as_mut_ptr() as *mut [u8;crate::HASH_LENGTH];

        digest.iter().zip(self.0.iter()).enumerate().for_each(|(i,(n,seed))| {
            let mut hashed_value = *seed; // Start with each private key scalar
            for _ in 0..*n {
                hashed_value = *hash(&hashed_value).split_first_chunk::<{ crate::HASH_LENGTH }>().expect("Invalid length").0;
            }
            unsafe {
                signature_ptr.add(i).write(hashed_value);
            }
        });

        WinternitzPubkey::from(unsafe { signature.assume_init() })
    }

    pub fn verify(&self, message: &[u8], pubkey: &WinternitzPubkey) -> bool {
        self.recover_pubkey(message).eq(pubkey)
    }
}