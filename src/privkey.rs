use crate::{
    hash::Hash, pubkey::WinternitzPubkey, signature::WinternitzSignature, HASH_LENGTH, KEY_LENGTH,
};
use core::mem::MaybeUninit;
use rand::Rng;

#[repr(C)]
pub struct WinternitzPrivkey {
    data: [[u8; HASH_LENGTH]; 32],
}

impl core::fmt::Debug for WinternitzPrivkey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WinternitzPrivkey")
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

impl Default for WinternitzPrivkey {
    fn default() -> Self {
        Self::generate()
    }
}

impl From<[u8; KEY_LENGTH]> for WinternitzPrivkey {
    fn from(value: [u8; KEY_LENGTH]) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

impl From<[[u8; HASH_LENGTH]; 32]> for WinternitzPrivkey {
    fn from(seeds: [[u8; HASH_LENGTH]; 32]) -> Self {
        Self { data: seeds }
    }
}

impl WinternitzPrivkey {
    pub fn generate() -> Self {
        let mut rng = rand::rng();
        // Generate 32 arrays of 32 random bytes each
        let seeds: [[u8; HASH_LENGTH]; 32] = rng.gen();
        Self { data: seeds }
    }

    pub fn pubkey<H: Hash>(&self) -> WinternitzPubkey {
        let result: [[u8; HASH_LENGTH]; 32] = self.data.map(|seed| {
            let mut hashed_value: [u8; HASH_LENGTH] = H::hash(&seed);
            for _ in 0..255 {
                hashed_value = H::hash(&hashed_value);
            }
            hashed_value[..HASH_LENGTH]
                .try_into()
                .expect("Invalid length")
        });
        WinternitzPubkey::from(result)
    }

    pub fn sign<H: Hash>(&self, message: &[u8]) -> WinternitzSignature {
        let digest = H::hash(message);

        let mut signature = MaybeUninit::<[[u8; HASH_LENGTH]; 32]>::uninit();
        let signature_ptr = signature.as_mut_ptr() as *mut [u8; HASH_LENGTH];

        digest
            .iter()
            .zip(self.data.iter())
            .enumerate()
            .for_each(|(i, (n, seed))| {
                let mut hashed_value: [u8; HASH_LENGTH] = H::hash(seed);
                for _ in 0..(!n as usize) {
                    hashed_value = H::hash(&hashed_value)
                }
                unsafe {
                    signature_ptr.add(i).write(hashed_value);
                }
            });

        WinternitzSignature::from(unsafe { signature.assume_init() })
    }
}
