// FILE: bopds_common_block.rs
// occt: BOPDS_CommonBlock

/// Stores information about common block in Boolean operations
#[derive(Clone, Debug)]
pub struct BodsCommonBlock {
    pave_blocks: Vec<i32>,
    faces: Vec<i32>,
}

impl BodsCommonBlock {
    pub fn new() -> Self {
        BodsCommonBlock {
            pave_blocks: Vec::new(),
            faces: Vec::new(),
        }
    }

    pub fn add_pave_block(&mut self, pb: i32) {
        self.pave_blocks.push(pb);
    }

    pub fn pave_blocks(&self) -> &[i32] {
        &self.pave_blocks
    }

    pub fn add_face(&mut self, face: i32) {
        self.faces.push(face);
    }

    pub fn faces(&self) -> &[i32] {
        &self.faces
    }

    pub fn clear(&mut self) {
        self.pave_blocks.clear();
        self.faces.clear();
    }
}

impl Default for BodsCommonBlock {
    fn default() -> Self {
        BodsCommonBlock::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cb = BodsCommonBlock::new();
        assert_eq!(cb.pave_blocks().len(), 0);
        assert_eq!(cb.faces().len(), 0);
    }

    #[test]
    fn test_add_pave_block() {
        let mut cb = BodsCommonBlock::new();
        cb.add_pave_block(1);
        cb.add_pave_block(2);
        assert_eq!(cb.pave_blocks().len(), 2);
    }

    #[test]
    fn test_add_face() {
        let mut cb = BodsCommonBlock::new();
        cb.add_face(10);
        cb.add_face(20);
        assert_eq!(cb.faces().len(), 2);
    }

    #[test]
    fn test_clear() {
        let mut cb = BodsCommonBlock::new();
        cb.add_pave_block(1);
        cb.add_face(10);
        cb.clear();
        assert_eq!(cb.pave_blocks().len(), 0);
        assert_eq!(cb.faces().len(), 0);
    }
}
