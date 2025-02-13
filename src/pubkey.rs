use solana_nostd_sha256::hashv;

use crate::HASH_LENGTH;

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
                            .map(|byte| format!("{:02x}", byte))
                            .collect::<String>()
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
    pub fn merklize(&self) -> [u8; HASH_LENGTH] {
        let left = self.merklize_first_half();
        let right = self.merklize_second_half();
        Self::hash_node_pair(&left, &right)
    }

    #[inline]
    pub fn merklize_first_half(&self) -> [u8; HASH_LENGTH] {
        Self::hash_node_pair(&&self.merklize_quarter(0), &&self.merklize_quarter(8))
    }

    #[inline]
    pub fn merklize_second_half(&self) -> [u8; HASH_LENGTH] {
        Self::hash_node_pair(&&self.merklize_quarter(16), &&self.merklize_quarter(24))
    }

    #[inline]
    fn hash_leaf_pair(&self, i: usize) -> [u8; HASH_LENGTH] {
        hashv(&[&self.data[i], &self.data[i + 1]])
    }

    #[inline]
    fn hash_node_pair(left: &[u8; HASH_LENGTH], right: &[u8; HASH_LENGTH]) -> [u8; HASH_LENGTH] {
        hashv(&[left, right])
    }

    #[inline]
    fn merklize_eighth(&self, start_idx: usize) -> [u8; HASH_LENGTH] {
        let h1 = self.hash_leaf_pair(start_idx);
        let h2 = self.hash_leaf_pair(start_idx + 2);
        Self::hash_node_pair(&h1, &h2)
    }

    #[inline]
    fn merklize_quarter(&self, start_idx: usize) -> [u8; HASH_LENGTH] {
        let left = self.merklize_eighth(start_idx);
        let right = self.merklize_eighth(start_idx + 4);
        Self::hash_node_pair(&left, &right)
    }
}

impl From<[[u8; HASH_LENGTH]; 32]> for WinternitzPubkey {
    fn from(value: [[u8; HASH_LENGTH]; 32]) -> Self {
        Self { data: value }
    }
}
