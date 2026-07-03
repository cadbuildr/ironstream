// FILE: bop_tools_connexity_block.rs
// occt: BOPTools_ConnexityBlock

/// A block of connected shapes.
#[derive(Clone, Debug)]
pub struct BopToolsConnexityBlock {
    shapes: Vec<Vec<u8>>,
    loops: Vec<Vec<u8>>,
    is_regular: bool,
}

impl BopToolsConnexityBlock {
    /// Create an empty connectivity block.
    pub fn new() -> Self {
        BopToolsConnexityBlock {
            shapes: Vec::new(),
            loops: Vec::new(),
            is_regular: true,
        }
    }

    /// Get the shapes list.
    pub fn shapes(&self) -> &[Vec<u8>] {
        &self.shapes
    }

    /// Get mutable access to shapes.
    pub fn shapes_mut(&mut self) -> &mut Vec<Vec<u8>> {
        &mut self.shapes
    }

    /// Set the regular flag.
    pub fn set_regular(&mut self, flag: bool) {
        self.is_regular = flag;
    }

    /// Check if the block is regular.
    pub fn is_regular(&self) -> bool {
        self.is_regular
    }

    /// Get the loops list.
    pub fn loops(&self) -> &[Vec<u8>] {
        &self.loops
    }

    /// Get mutable access to loops.
    pub fn loops_mut(&mut self) -> &mut Vec<Vec<u8>> {
        &mut self.loops
    }
}

impl Default for BopToolsConnexityBlock {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let block = BopToolsConnexityBlock::new();
        assert_eq!(block.shapes().len(), 0);
        assert_eq!(block.loops().len(), 0);
        assert!(block.is_regular());
    }

    #[test]
    fn test_set_regular() {
        let mut block = BopToolsConnexityBlock::new();
        assert!(block.is_regular());
        block.set_regular(false);
        assert!(!block.is_regular());
    }

    #[test]
    fn test_shapes_mut() {
        let mut block = BopToolsConnexityBlock::new();
        block.shapes_mut().push(vec![1, 2, 3]);
        assert_eq!(block.shapes().len(), 1);
    }

    #[test]
    fn test_loops_mut() {
        let mut block = BopToolsConnexityBlock::new();
        block.loops_mut().push(vec![10, 20]);
        assert_eq!(block.loops().len(), 1);
    }
}
