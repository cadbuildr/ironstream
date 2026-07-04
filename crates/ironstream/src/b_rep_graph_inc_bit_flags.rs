// FILE: b_rep_graph_inc_bit_flags.rs
// occt: BRepGraphInc_BitFlags

//! Contiguous bit-vector for per-entity boolean flags.
//! Stores one bit per entity index in flat array of 64-bit blocks.

const BITS_PER_BLOCK: usize = 64;

/// Bit-vector for efficient flag storage
pub struct BRepGraphIncBitFlags {
    blocks: Vec<u64>,
    bit_count: usize,
}

impl BRepGraphIncBitFlags {
    /// Creates an empty bit-vector
    pub fn new() -> Self {
        BRepGraphIncBitFlags {
            blocks: Vec::new(),
            bit_count: 0,
        }
    }

    /// Resizes the bit-vector to hold at least count bits
    pub fn resize(&mut self, count: usize) {
        let block_count = (count + BITS_PER_BLOCK - 1) / BITS_PER_BLOCK;
        self.blocks.resize(block_count, 0);
        self.bit_count = count;
        self.mask_tail_bits();
    }

    /// Sets the bit at index to true
    pub fn set(&mut self, index: usize) {
        if index < self.bit_count {
            let block = index / BITS_PER_BLOCK;
            let bit = index % BITS_PER_BLOCK;
            if block < self.blocks.len() {
                self.blocks[block] |= 1u64 << bit;
            }
        }
    }

    /// Clears the bit at index to false
    pub fn clear(&mut self, index: usize) {
        if index < self.bit_count {
            let block = index / BITS_PER_BLOCK;
            let bit = index % BITS_PER_BLOCK;
            if block < self.blocks.len() {
                self.blocks[block] &= !(1u64 << bit);
            }
        }
    }

    /// Returns the value of the bit at index
    pub fn test(&self, index: usize) -> bool {
        if index < self.bit_count {
            let block = index / BITS_PER_BLOCK;
            let bit = index % BITS_PER_BLOCK;
            if block < self.blocks.len() {
                return (self.blocks[block] & (1u64 << bit)) != 0;
            }
        }
        false
    }

    /// Clears all bits
    pub fn clear_all(&mut self) {
        for block in &mut self.blocks {
            *block = 0;
        }
    }

    /// Sets all bits
    pub fn set_all(&mut self) {
        for block in &mut self.blocks {
            *block = u64::MAX;
        }
        self.mask_tail_bits();
    }

    /// Returns the number of bits
    pub fn count(&self) -> usize {
        self.bit_count
    }

    /// Masks tail bits beyond bit_count in the last block
    fn mask_tail_bits(&mut self) {
        if !self.blocks.is_empty() && self.bit_count % BITS_PER_BLOCK != 0 {
            let last_block = self.blocks.len() - 1;
            let tail_bits = self.bit_count % BITS_PER_BLOCK;
            let mask = (1u64 << tail_bits) - 1;
            self.blocks[last_block] &= mask;
        }
    }
}

impl Default for BRepGraphIncBitFlags {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_flags_new() {
        let flags = BRepGraphIncBitFlags::new();
        assert_eq!(flags.count(), 0);
    }

    #[test]
    fn test_bit_flags_resize() {
        let mut flags = BRepGraphIncBitFlags::new();
        flags.resize(100);
        assert_eq!(flags.count(), 100);
        for i in 0..100 {
            assert!(!flags.test(i));
        }
    }

    #[test]
    fn test_bit_flags_set_clear_test() {
        let mut flags = BRepGraphIncBitFlags::new();
        flags.resize(64);

        flags.set(0);
        assert!(flags.test(0));

        flags.set(63);
        assert!(flags.test(63));

        flags.clear(0);
        assert!(!flags.test(0));
        assert!(flags.test(63));
    }

    #[test]
    fn test_bit_flags_multiple_blocks() {
        let mut flags = BRepGraphIncBitFlags::new();
        flags.resize(200);

        flags.set(50);
        flags.set(70);
        flags.set(150);

        assert!(flags.test(50));
        assert!(flags.test(70));
        assert!(flags.test(150));
        assert!(!flags.test(49));
        assert!(!flags.test(151));
    }

    #[test]
    fn test_bit_flags_set_all() {
        let mut flags = BRepGraphIncBitFlags::new();
        flags.resize(128);
        flags.set_all();

        for i in 0..128 {
            assert!(flags.test(i));
        }
    }

    #[test]
    fn test_bit_flags_clear_all() {
        let mut flags = BRepGraphIncBitFlags::new();
        flags.resize(128);
        flags.set_all();
        flags.clear_all();

        for i in 0..128 {
            assert!(!flags.test(i));
        }
    }

    #[test]
    fn test_bit_flags_out_of_bounds() {
        let mut flags = BRepGraphIncBitFlags::new();
        flags.resize(10);
        flags.set(20); // Out of bounds, should be safe
        assert!(!flags.test(20)); // Should not panic, returns false
    }
}
