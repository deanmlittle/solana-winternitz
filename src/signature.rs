use crate::{address::WinternitzAddress, hash::WinternitzHash, pubkey::WinternitzPubkey, winternitz_debug};

#[repr(C)]
#[derive(PartialEq)]
pub struct WinternitzSignature(pub [[u8; 32]; 32]);

impl From<[[u8; 32]; 32]> for WinternitzSignature {
    fn from(value: [[u8; 32]; 32]) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

#[repr(C)]
#[derive(PartialEq)]
pub struct WinternitzPrimeSignature(pub [[u8; 32]; 28]);

impl WinternitzPrimeSignature {
    pub fn recover_address<H: WinternitzHash>(&self, message: &[u8], pairing_hash: &[u8;32]) -> WinternitzAddress {
        let v = H::hashd(message);

        let mut h = self.0;

        for (i, v) in v[..28].iter().enumerate() {
            for _ in 0..*v {
                h[i] = H::hash(&h[i]);
            }
        }

        H::hash(
            &H::hash_pair(
                &H::hash_pair(
                    &H::hash_pair(
                        &H::hash_pair(
                            &H::hash_pair(&h[0], &h[1]),
                            &H::hash_pair(&h[2], &h[3]),
                        ),
                        &H::hash_pair(
                            &H::hash_pair(&h[4], &h[5]),
                            &H::hash_pair(&h[6], &h[7]),
                        ),
                    ),
                    &H::hash_pair(
                        &H::hash_pair(
                            &H::hash_pair(&h[8], &h[9]),
                            &H::hash_pair(&h[10], &h[11]),
                        ),
                        &H::hash_pair(
                            &H::hash_pair(&h[12], &h[13]),
                            &H::hash_pair(&h[14], &h[15]),
                        ),
                    ),
                ),
                &H::hash_pair(
                    &H::hash_pair(
                        &H::hash_pair(
                            &H::hash_pair(&h[16], &h[17]),
                            &H::hash_pair(&h[18], &h[19]),
                        ),
                        &H::hash_pair(
                            &H::hash_pair(&h[20], &h[21]),
                            &H::hash_pair(&h[22], &h[23]),
                        ),
                    ),
                    &H::hash_pair(
                        &H::hash_pair(
                            &H::hash_pair(&h[24], &h[25]),
                            &H::hash_pair(&h[26], &h[27]),
                        ),
                        pairing_hash
                    ),
                ),
            )
        ).into()
    }
}

#[repr(C)]
#[derive(PartialEq)]
pub struct WinternitzExecuteSignature(pub [[u8; 32]; 4]);

impl WinternitzExecuteSignature {
    pub fn recover_pairing_hash<H: WinternitzHash>(&self, message: &[u8]) -> [u8; 32] {
        let v = H::hashd(message);

        let mut h = self.0;

        for (i, v) in v[28..].iter().enumerate() {
            for _ in 0..*v {
                h[i] = H::hash(&h[i]);
            }
        }

        H::hash_pair(
            &H::hash_pair(&h[0], &h[1]),
            &H::hash_pair(&h[2], &h[3]),
        )
    }
}

impl WinternitzSignature {
    #[inline(always)]
    pub fn recover_pubkey<H: WinternitzHash>(&self, message: &[u8]) -> WinternitzPubkey {
        let v = H::hashd(message);
        let mut pubkey: WinternitzPubkey = self.0.into();
        for (i, v) in v.iter().enumerate() {
            for _ in 0..*v {
                pubkey.0[i] = H::hash(&pubkey.0[i]);
            }
        }
        pubkey
    }

    pub fn split<H: WinternitzHash>(
        &self,
        message: &[u8],
    ) -> ([u8;32], WinternitzPrimeSignature, WinternitzExecuteSignature) {
        let pairing_hash = self.recover_pubkey::<H>(message).pairing_hash::<H>();

        let prime = WinternitzPrimeSignature([
            self.0[0],
            self.0[1],
            self.0[2],
            self.0[3],
            self.0[4],
            self.0[5],
            self.0[6],
            self.0[7],
            self.0[8],
            self.0[9],
            self.0[10],
            self.0[11],
            self.0[12],
            self.0[13],
            self.0[14],
            self.0[15],
            self.0[16],
            self.0[17],
            self.0[18],
            self.0[19],
            self.0[20],
            self.0[21],
            self.0[22],
            self.0[23],
            self.0[24],
            self.0[25],
            self.0[26],
            self.0[27],
        ]);

        let execute = WinternitzExecuteSignature([self.0[28], self.0[29], self.0[30], self.0[31]]);

        (pairing_hash, prime, execute)
    }
}

winternitz_debug!(WinternitzSignature, "WinternitzSignature");
winternitz_debug!(WinternitzPrimeSignature, "WinternitzPrimeSignature");
winternitz_debug!(WinternitzExecuteSignature, "WinternitzExecuteSignature");
