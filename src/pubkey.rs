use solana_nostd_keccak::{hash, hashv};

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

    pub fn merklize(&self) -> [u8;32] {
        hashv(&[
            &hashv(&[
                &hashv(&[
                    &hashv(&[
                        &hashv(&[&self.0[0], &self.0[1]]), 
                        &hashv(&[&self.0[2], &self.0[3]]), 
                    ]),
                    &hashv(&[
                        &hashv(&[&self.0[4], &self.0[5]]), 
                        &hashv(&[&self.0[6], &self.0[7]]),
                    ]),
                ]),
                &hashv(&[
                    &hashv(&[
                        &hashv(&[&self.0[8], &self.0[9]]), 
                        &hashv(&[&self.0[10], &self.0[11]]), 
                    ]),
                    &hashv(&[
                        &hashv(&[&self.0[12], &self.0[13]]), 
                        &hashv(&[&self.0[14], &self.0[15]]),
                    ]),
                ]),
            ]),
            &hashv(&[
                &hashv(&[
                    &hashv(&[
                        &hashv(&[&self.0[16], &self.0[17]]), 
                        &hashv(&[&self.0[18], &self.0[19]]), 
                    ]),
                    &hashv(&[
                        &hashv(&[&self.0[20], &self.0[21]]), 
                        &hashv(&[&self.0[22], &self.0[23]]),
                    ]),
                ]),
                &hashv(&[
                    &hashv(&[
                        &hashv(&[&self.0[24], &self.0[25]]), 
                        &hashv(&[&self.0[26], &self.0[27]]), 
                    ]),
                    &hashv(&[
                        &hashv(&[&self.0[28], &self.0[29]]), 
                        &hashv(&[&self.0[30], &self.0[31]])    
                    ]), 
                ])  
            ]) 
        ])
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