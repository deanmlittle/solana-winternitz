use crate::{hash::Hash, HASH_LENGTH};
use core::fmt::Write;

#[repr(C)]
#[derive(PartialEq)]
pub struct WinternitzPubkey {
    data: [[u8; HASH_LENGTH]; 32],
}

impl core::fmt::Debug for WinternitzPubkey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("WinternitzPubkey")
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

impl WinternitzPubkey {
    pub fn hashes(&self) -> [[u8; HASH_LENGTH]; 32] {
        self.data
    }

    // Your public interface remains the same
    pub fn merklize<H: Hash>(&self) -> [u8; HASH_LENGTH] {
        let left = self.merklize_first_half::<H>();
        let right = self.merklize_second_half::<H>();
        Self::hash_node_pair::<H>(&left, &right)
    }

    #[inline]
    pub fn merklize_first_half<H: Hash>(&self) -> [u8; HASH_LENGTH] {
        Self::hash_node_pair::<H>(
            &self.merklize_quarter::<H>(0),
            &self.merklize_quarter::<H>(8),
        )
    }

    #[inline]
    pub fn merklize_second_half<H: Hash>(&self) -> [u8; HASH_LENGTH] {
        Self::hash_node_pair::<H>(
            &self.merklize_quarter::<H>(16),
            &self.merklize_quarter::<H>(24),
        )
    }

    #[inline]
    fn hash_leaf_pair<H: Hash>(&self, i: usize) -> [u8; HASH_LENGTH] {
        H::hashv(&[&self.data[i], &self.data[i + 1]])
    }

    #[inline]
    fn hash_node_pair<H: Hash>(
        left: &[u8; HASH_LENGTH],
        right: &[u8; HASH_LENGTH],
    ) -> [u8; HASH_LENGTH] {
        H::hashv(&[left, right])
    }

    #[inline]
    fn merklize_eighth<H: Hash>(&self, start_idx: usize) -> [u8; HASH_LENGTH] {
        let h1 = self.hash_leaf_pair::<H>(start_idx);
        let h2 = self.hash_leaf_pair::<H>(start_idx + 2);
        Self::hash_node_pair::<H>(&h1, &h2)
    }

    #[inline]
    fn merklize_quarter<H: Hash>(&self, start_idx: usize) -> [u8; HASH_LENGTH] {
        let left = self.merklize_eighth::<H>(start_idx);
        let right = self.merklize_eighth::<H>(start_idx + 4);
        Self::hash_node_pair::<H>(&left, &right)
    }
}

impl From<[[u8; HASH_LENGTH]; 32]> for WinternitzPubkey {
    fn from(value: [[u8; HASH_LENGTH]; 32]) -> Self {
        Self { data: value }
    }
}
