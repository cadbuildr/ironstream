// FILE: bopds_couple_of_pave_blocks.rs
// occt: BOPDS_CoupleOfPaveBlocks

/// Stores a pair of pave blocks
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BodsCoupleOfPaveBlocks {
    pb1: i32,
    pb2: i32,
}

impl BodsCoupleOfPaveBlocks {
    pub fn new() -> Self {
        BodsCoupleOfPaveBlocks { pb1: -1, pb2: -1 }
    }

    pub fn with_blocks(pb1: i32, pb2: i32) -> Self {
        BodsCoupleOfPaveBlocks { pb1, pb2 }
    }

    pub fn pb1(&self) -> i32 {
        self.pb1
    }

    pub fn pb2(&self) -> i32 {
        self.pb2
    }

    pub fn set_blocks(&mut self, pb1: i32, pb2: i32) {
        self.pb1 = pb1;
        self.pb2 = pb2;
    }
}

impl Default for BodsCoupleOfPaveBlocks {
    fn default() -> Self {
        BodsCoupleOfPaveBlocks::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let couple = BodsCoupleOfPaveBlocks::new();
        assert_eq!(couple.pb1(), -1);
        assert_eq!(couple.pb2(), -1);
    }

    #[test]
    fn test_with_blocks() {
        let couple = BodsCoupleOfPaveBlocks::with_blocks(1, 2);
        assert_eq!(couple.pb1(), 1);
        assert_eq!(couple.pb2(), 2);
    }

    #[test]
    fn test_set_blocks() {
        let mut couple = BodsCoupleOfPaveBlocks::new();
        couple.set_blocks(3, 4);
        assert_eq!(couple.pb1(), 3);
        assert_eq!(couple.pb2(), 4);
    }
}
