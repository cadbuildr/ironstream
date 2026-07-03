// FILE: hlr_algo_wires_block.rs
// occt: HLRAlgo_WiresBlock

/// A block of wires (set of edge blocks).
pub struct HlrAlgoWiresBlock {
    wires: Vec<Option<i32>>, // Wire indices
    min_max_min: Vec<[i32; 8]>,
    min_max_max: Vec<[i32; 8]>,
}

impl HlrAlgoWiresBlock {
    /// Create a block of wires.
    pub fn new(nb_wires: usize) -> Self {
        HlrAlgoWiresBlock {
            wires: vec![None; nb_wires],
            min_max_min: vec![[0; 8]; 1],
            min_max_max: vec![[0; 8]; 1],
        }
    }

    /// Get number of wires.
    pub fn nb_wires(&self) -> usize {
        self.wires.len()
    }

    /// Set wire at index.
    pub fn set_wire(&mut self, i: usize, w: i32) {
        if i < self.wires.len() {
            self.wires[i] = Some(w);
        }
    }

    /// Get wire at index.
    pub fn wire(&self, i: usize) -> Option<i32> {
        self.wires.get(i).and_then(|w| *w)
    }

    /// Update min/max indices.
    pub fn update_min_max(&mut self, mins: [i32; 8], maxs: [i32; 8]) {
        if !self.min_max_min.is_empty() {
            self.min_max_min[0] = mins;
            self.min_max_max[0] = maxs;
        }
    }

    /// Get min indices.
    pub fn min(&self) -> Option<[i32; 8]> {
        self.min_max_min.get(0).copied()
    }

    /// Get max indices.
    pub fn max(&self) -> Option<[i32; 8]> {
        self.min_max_max.get(0).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let block = HlrAlgoWiresBlock::new(5);
        assert_eq!(block.nb_wires(), 5);
    }

    #[test]
    fn test_set_get_wire() {
        let mut block = HlrAlgoWiresBlock::new(3);
        block.set_wire(0, 10);
        block.set_wire(1, 20);
        assert_eq!(block.wire(0), Some(10));
        assert_eq!(block.wire(1), Some(20));
        assert_eq!(block.wire(2), None);
    }

    #[test]
    fn test_update_min_max() {
        let mut block = HlrAlgoWiresBlock::new(3);
        let mins = [1, 2, 3, 4, 5, 6, 7, 8];
        let maxs = [9, 10, 11, 12, 13, 14, 15, 16];
        block.update_min_max(mins, maxs);
        assert_eq!(block.min(), Some(mins));
        assert_eq!(block.max(), Some(maxs));
    }

    #[test]
    fn test_bounds() {
        let block = HlrAlgoWiresBlock::new(5);
        assert_eq!(block.wire(10), None);
    }
}
