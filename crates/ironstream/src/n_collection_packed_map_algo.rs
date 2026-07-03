// FILE: n_collection_packed_map_algo.rs
// occt: NCollection_PackedMapAlgo

/// Algorithms for packed (bit-vector) map operations.
pub mod packed_map_algo {
    /// Check if all bits from other are set in this map.
    pub fn contains(this: &[u8], other: &[u8]) -> bool {
        if this.len() < other.len() {
            return false;
        }
        for i in 0..other.len() {
            if (this[i] & other[i]) != other[i] {
                return false;
            }
        }
        true
    }

    /// Union of two packed maps (bitwise OR).
    pub fn union(this: &mut [u8], other: &[u8]) {
        for i in 0..other.len().min(this.len()) {
            this[i] |= other[i];
        }
    }

    /// Intersection of two packed maps (bitwise AND).
    pub fn intersection(this: &mut [u8], other: &[u8]) {
        for i in 0..this.len() {
            if i < other.len() {
                this[i] &= other[i];
            } else {
                this[i] = 0;
            }
        }
    }

    /// Count set bits.
    pub fn count_bits(data: &[u8]) -> usize {
        data.iter().map(|b| b.count_ones() as usize).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::packed_map_algo::*;

    #[test]
    fn test_contains() {
        let a = [0b1111, 0b0011];
        let b = [0b0011, 0b0001];
        assert!(contains(&a, &b));
    }

    #[test]
    fn test_union() {
        let mut a = [0b1100, 0b0000];
        let b = [0b0011, 0b1111];
        union(&mut a, &b);
        assert_eq!(a[0], 0b1111);
        assert_eq!(a[1], 0b1111);
    }

    #[test]
    fn test_intersection() {
        let mut a = [0b1111, 0b1111];
        let b = [0b0011, 0b0011];
        intersection(&mut a, &b);
        assert_eq!(a[0], 0b0011);
        assert_eq!(a[1], 0b0011);
    }

    #[test]
    fn test_count_bits() {
        let data = [0b1111, 0b0011];
        assert_eq!(count_bits(&data), 6);
    }
}
