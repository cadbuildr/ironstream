// FILE: hlr_algo_edges_block.rs
// occt: HLRAlgo_EdgesBlock

#[derive(Clone, Copy, Debug)]
pub enum TopAbsOrientation {
    Forward = 0,
    Reversed = 1,
    Internal = 2,
    External = 3,
}

/// A block of edges for a wire with flags and orientation.
pub struct HlrAlgoEdgesBlock {
    edges: Vec<i32>,
    flags: Vec<i32>,
}

impl HlrAlgoEdgesBlock {
    const EMASK_ORIENT: i32 = 15;
    const EMASK_OUTLINE: i32 = 16;
    const EMASK_INTERNAL: i32 = 32;
    const EMASK_DOUBLE: i32 = 64;
    const EMASK_ISOLINE: i32 = 128;

    /// Create a block for a given number of edges.
    pub fn new(nb_edges: usize) -> Self {
        HlrAlgoEdgesBlock {
            edges: vec![0; nb_edges],
            flags: vec![0; nb_edges],
        }
    }

    /// Get number of edges in this block.
    pub fn nb_edges(&self) -> usize {
        self.edges.len()
    }

    /// Set the edge index at position i.
    pub fn set_edge(&mut self, i: usize, ei: i32) {
        if i < self.edges.len() {
            self.edges[i] = ei;
        }
    }

    /// Get the edge index at position i.
    pub fn edge(&self, i: usize) -> Option<i32> {
        self.edges.get(i).copied()
    }

    /// Set orientation at position i.
    pub fn set_orientation(&mut self, i: usize, or: TopAbsOrientation) {
        if i < self.flags.len() {
            self.flags[i] = (self.flags[i] & !Self::EMASK_ORIENT) | ((or as i32) & Self::EMASK_ORIENT);
        }
    }

    /// Get orientation at position i.
    pub fn orientation(&self, i: usize) -> Option<TopAbsOrientation> {
        self.flags.get(i).map(|f| {
            match (f & Self::EMASK_ORIENT) as u8 {
                0 => TopAbsOrientation::Forward,
                1 => TopAbsOrientation::Reversed,
                2 => TopAbsOrientation::Internal,
                _ => TopAbsOrientation::External,
            }
        })
    }

    /// Check if edge at i is on outline.
    pub fn outline(&self, i: usize) -> bool {
        self.flags.get(i).map(|f| (f & Self::EMASK_OUTLINE) != 0).unwrap_or(false)
    }

    /// Set outline flag.
    pub fn set_outline(&mut self, i: usize, b: bool) {
        if i < self.flags.len() {
            if b {
                self.flags[i] |= Self::EMASK_OUTLINE;
            } else {
                self.flags[i] &= !Self::EMASK_OUTLINE;
            }
        }
    }

    /// Check if edge is internal.
    pub fn internal(&self, i: usize) -> bool {
        self.flags.get(i).map(|f| (f & Self::EMASK_INTERNAL) != 0).unwrap_or(false)
    }

    /// Set internal flag.
    pub fn set_internal(&mut self, i: usize, b: bool) {
        if i < self.flags.len() {
            if b {
                self.flags[i] |= Self::EMASK_INTERNAL;
            } else {
                self.flags[i] &= !Self::EMASK_INTERNAL;
            }
        }
    }

    /// Check if edge is double.
    pub fn double(&self, i: usize) -> bool {
        self.flags.get(i).map(|f| (f & Self::EMASK_DOUBLE) != 0).unwrap_or(false)
    }

    /// Set double flag.
    pub fn set_double(&mut self, i: usize, b: bool) {
        if i < self.flags.len() {
            if b {
                self.flags[i] |= Self::EMASK_DOUBLE;
            } else {
                self.flags[i] &= !Self::EMASK_DOUBLE;
            }
        }
    }

    /// Check if edge is isoline.
    pub fn isoline(&self, i: usize) -> bool {
        self.flags.get(i).map(|f| (f & Self::EMASK_ISOLINE) != 0).unwrap_or(false)
    }

    /// Set isoline flag.
    pub fn set_isoline(&mut self, i: usize, b: bool) {
        if i < self.flags.len() {
            if b {
                self.flags[i] |= Self::EMASK_ISOLINE;
            } else {
                self.flags[i] &= !Self::EMASK_ISOLINE;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let block = HlrAlgoEdgesBlock::new(5);
        assert_eq!(block.nb_edges(), 5);
    }

    #[test]
    fn test_set_get_edge() {
        let mut block = HlrAlgoEdgesBlock::new(3);
        block.set_edge(0, 10);
        block.set_edge(1, 20);
        assert_eq!(block.edge(0), Some(10));
        assert_eq!(block.edge(1), Some(20));
    }

    #[test]
    fn test_orientation() {
        let mut block = HlrAlgoEdgesBlock::new(2);
        block.set_orientation(0, TopAbsOrientation::Forward);
        block.set_orientation(1, TopAbsOrientation::Reversed);
        matches!(block.orientation(0), Some(TopAbsOrientation::Forward));
        matches!(block.orientation(1), Some(TopAbsOrientation::Reversed));
    }

    #[test]
    fn test_flags() {
        let mut block = HlrAlgoEdgesBlock::new(1);
        assert!(!block.outline(0));
        block.set_outline(0, true);
        assert!(block.outline(0));

        assert!(!block.internal(0));
        block.set_internal(0, true);
        assert!(block.internal(0));
    }

    #[test]
    fn test_bounds() {
        let block = HlrAlgoEdgesBlock::new(5);
        assert_eq!(block.edge(10), None);
        assert_eq!(block.outline(10), false);
    }
}
