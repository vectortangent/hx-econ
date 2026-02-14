//! Bitset utilities for tracking dirty sets

/// Simple bitset for tracking dirty markets/edges/regions
pub struct BitSet {
    bits: Vec<u64>,
}

impl BitSet {
    /// Create a new bitset with capacity for `size` bits
    pub fn new(size: usize) -> Self {
        let words = (size + 63) / 64;
        Self {
            bits: vec![0; words],
        }
    }

    /// Set a bit
    #[inline]
    pub fn set(&mut self, index: usize) {
        let word = index / 64;
        let bit = index % 64;
        if word < self.bits.len() {
            self.bits[word] |= 1u64 << bit;
        }
    }

    /// Clear a bit
    #[inline]
    pub fn clear(&mut self, index: usize) {
        let word = index / 64;
        let bit = index % 64;
        if word < self.bits.len() {
            self.bits[word] &= !(1u64 << bit);
        }
    }

    /// Test if a bit is set
    #[inline]
    pub fn test(&self, index: usize) -> bool {
        let word = index / 64;
        let bit = index % 64;
        if word < self.bits.len() {
            (self.bits[word] & (1u64 << bit)) != 0
        } else {
            false
        }
    }

    /// Clear all bits
    pub fn clear_all(&mut self) {
        self.bits.fill(0);
    }

    /// Iterate over set bits
    pub fn iter_set(&self) -> impl Iterator<Item = usize> + '_ {
        self.bits
            .iter()
            .enumerate()
            .flat_map(|(word_idx, &word)| {
                (0..64).filter_map(move |bit| {
                    if (word & (1u64 << bit)) != 0 {
                        Some(word_idx * 64 + bit)
                    } else {
                        None
                    }
                })
            })
    }

    /// Count set bits
    pub fn count(&self) -> usize {
        self.bits.iter().map(|w| w.count_ones() as usize).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitset_basic() {
        let mut bs = BitSet::new(100);
        assert!(!bs.test(0));
        assert!(!bs.test(50));

        bs.set(0);
        bs.set(50);
        bs.set(99);

        assert!(bs.test(0));
        assert!(bs.test(50));
        assert!(bs.test(99));
        assert!(!bs.test(1));

        bs.clear(50);
        assert!(!bs.test(50));

        assert_eq!(bs.count(), 2);
    }

    #[test]
    fn test_bitset_iter() {
        let mut bs = BitSet::new(200);
        bs.set(5);
        bs.set(100);
        bs.set(150);

        let set_bits: Vec<_> = bs.iter_set().collect();
        assert_eq!(set_bits, vec![5, 100, 150]);
    }
}
