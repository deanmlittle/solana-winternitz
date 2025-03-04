use crate::{address::WinternitzAddress, hash::WinternitzHash, winternitz_debug};

#[repr(C)]
#[derive(PartialEq)]
pub struct WinternitzPubkey(pub [[u8; 32]; 32]);

impl From<[[u8; 32]; 32]> for WinternitzPubkey {
    fn from(value: [[u8; 32]; 32]) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

impl WinternitzPubkey {
    #[inline(always)]
    pub fn merklize<H: WinternitzHash>(&self) -> [u8; 32] {
        H::hash_pair(
            &H::hash_pair(
                &H::hash_pair(
                    &H::hash_pair(
                        &H::hash_pair(&self.0[0], &self.0[1]),
                        &H::hash_pair(&self.0[2], &self.0[3]),
                    ),
                    &H::hash_pair(
                        &H::hash_pair(&self.0[4], &self.0[5]),
                        &H::hash_pair(&self.0[6], &self.0[7]),
                    ),
                ),
                &H::hash_pair(
                    &H::hash_pair(
                        &H::hash_pair(&self.0[8], &self.0[9]),
                        &H::hash_pair(&self.0[10], &self.0[11]),
                    ),
                    &H::hash_pair(
                        &H::hash_pair(&self.0[12], &self.0[13]),
                        &H::hash_pair(&self.0[14], &self.0[15]),
                    ),
                ),
            ),
            &H::hash_pair(
                &H::hash_pair(
                    &H::hash_pair(
                        &H::hash_pair(&self.0[16], &self.0[17]),
                        &H::hash_pair(&self.0[18], &self.0[19]),
                    ),
                    &H::hash_pair(
                        &H::hash_pair(&self.0[20], &self.0[21]),
                        &H::hash_pair(&self.0[22], &self.0[23]),
                    ),
                ),
                &H::hash_pair(
                    &H::hash_pair(
                        &H::hash_pair(&self.0[24], &self.0[25]),
                        &H::hash_pair(&self.0[26], &self.0[27]),
                    ),
                    &H::hash_pair(
                        &H::hash_pair(&self.0[28], &self.0[29]),
                        &H::hash_pair(&self.0[30], &self.0[31]),
                    ),
                ),
            ),
        )
    }

    #[inline(always)]
    pub fn pairing_hash<H: WinternitzHash>(&self) -> [u8; 32] {
        H::hash_pair(
            &H::hash_pair(&self.0[28], &self.0[29]),
            &H::hash_pair(&self.0[30], &self.0[31]),
        )
    }

    #[inline(always)]
    pub fn address<H: WinternitzHash>(&self) -> WinternitzAddress {
        H::hash(&self.merklize::<H>()).into()
    }
}

winternitz_debug!(WinternitzPubkey, "WinternitzPubkey");
