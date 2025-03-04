use core::str::FromStr;

use bip32::{DerivationPath, Seed, XPrv};
use rand::random;

use crate::{
    hash::WinternitzHash, pubkey::WinternitzPubkey, signature::WinternitzSignature,
    winternitz_debug,
};

pub struct WinternitzPrivkey(pub [[u8; 32]; 32]);

impl From<[[u8; 32]; 32]> for WinternitzPrivkey {
    fn from(value: [[u8; 32]; 32]) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

impl WinternitzPrivkey {
    #[inline(always)]
    pub fn generate() -> Self {
        WinternitzPrivkey(random())
    }

    pub fn from_seed(seed: [u8;64], path: &str) -> Result<Self, bip32::Error> {
        let root = XPrv::derive_from_path(&Seed::new(seed), &DerivationPath::from_str(path)?)?;
        Ok(Self([
            root.derive_child(bip32::ChildNumber(0))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(1))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(2))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(3))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(4))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(5))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(6))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(7))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(8))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(9))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(10))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(11))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(12))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(13))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(14))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(15))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(16))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(17))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(18))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(19))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(20))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(21))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(22))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(23))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(24))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(25))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(26))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(27))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(28))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(29))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(30))?.private_key().to_bytes().into(),
            root.derive_child(bip32::ChildNumber(31))?.private_key().to_bytes().into(),
        ]))
    }
        
    #[inline(always)]
    pub fn sign<H: WinternitzHash>(&self, message: &[u8]) -> WinternitzSignature {
        let v: [u8; 32] = H::hashd(message);
        let mut sig: WinternitzSignature = self.0.into();
        for (i, v) in v.iter().enumerate() {
            sig.0[i] = H::hash(&sig.0[i]);
            for _ in 0..!*v {
                sig.0[i] = H::hash(&sig.0[i]);
            }
        }
        sig
    }

    #[inline(always)]
    pub fn pubkey<H: WinternitzHash>(&self) -> WinternitzPubkey {
        let mut pubkey: WinternitzPubkey = self.0.into();
        for i in 0..32 {
            pubkey.0[i] = H::hash(&pubkey.0[i]);
            for _ in 0..255 {
                pubkey.0[i] = H::hash(&pubkey.0[i]);
            }
        }
        pubkey
    }
}

winternitz_debug!(WinternitzPrivkey, "WinternitzPubkey");
