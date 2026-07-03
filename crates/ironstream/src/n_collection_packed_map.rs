// FILE: n_collection_packed_map.rs
// occt: NCollection_PackedMap

/// Packed map using bit flags for compact storage.
pub struct PackedMap {
    bits: Vec<u64>,
}

impl PackedMap {
    pub fn new(capacity: usize) -> Self {
        let num_words = (capacity + 63) / 64;
        PackedMap {
            bits: vec![0; num_words],
        }
    }

    pub fn set(&mut self, idx: usize) {
        let word_idx = idx / 64;
        let bit_idx = idx % 64;
        if word_idx < self.bits.len() {
            self.bits[word_idx] |= 1u64 << bit_idx;
        }
    }

    pub fn clear(&mut self, idx: usize) {
        let word_idx = idx / 64;
        let bit_idx = idx % 64;
        if word_idx < self.bits.len() {
            self.bits[word_idx] &= !(1u64 << bit_idx);
        }
    }

    pub fn test(&self, idx: usize) -> bool {
        let word_idx = idx / 64;
        let bit_idx = idx % 64;
        if word_idx < self.bits.len() {
            (self.bits[word_idx] & (1u64 << bit_idx)) != 0
        } else {
            false
        }
    }

    pub fn count(&self) -> usize {
        self.bits.iter().map(|w| w.count_ones() as usize).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packed_map() {
        let mut map = PackedMap::new(128);
        assert!(!map.test(0));
        map.set(0);
        assert!(map.test(0));
        map.clear(0);
        assert!(!map.test(0));
    }

    #[test]
    fn test_packed_map_count() {
        let mut map = PackedMap::new(128);
        map.set(0);
        map.set(5);
        map.set(64);
        assert_eq!(map.count(), 3);
    }
}
