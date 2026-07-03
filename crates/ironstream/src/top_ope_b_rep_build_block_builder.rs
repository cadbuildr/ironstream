// FILE: top_ope_b_rep_build_block_builder.rs
// occt: TopOpeBRepBuild_BlockBuilder

#[derive(Debug, Clone)]
pub struct TopOpeBRepBuildBlockBuilder {
    elements: Vec<i32>,
    validity: Vec<bool>,
    blocks: Vec<Vec<usize>>,
    current_block_index: usize,
}

impl TopOpeBRepBuildBlockBuilder {
    pub fn new() -> Self {
        TopOpeBRepBuildBlockBuilder {
            elements: Vec::new(),
            validity: Vec::new(),
            blocks: Vec::new(),
            current_block_index: 0,
        }
    }

    pub fn add_element(&mut self, element_id: i32) -> usize {
        let idx = self.elements.len();
        self.elements.push(element_id);
        self.validity.push(true);
        idx
    }

    pub fn element(&self, idx: usize) -> Option<i32> {
        self.elements.get(idx).copied()
    }

    pub fn set_valid(&mut self, idx: usize, is_valid: bool) {
        if idx < self.validity.len() {
            self.validity[idx] = is_valid;
        }
    }

    pub fn is_valid(&self, idx: usize) -> bool {
        self.validity.get(idx).copied().unwrap_or(false)
    }

    pub fn make_block(&mut self) {
        self.blocks.clear();
        let mut current_block = Vec::new();
        for i in 0..self.elements.len() {
            if self.is_valid(i) {
                current_block.push(i);
            }
        }
        if !current_block.is_empty() {
            self.blocks.push(current_block);
        }
    }

    pub fn init_block(&mut self) {
        self.current_block_index = 0;
    }

    pub fn more_block(&self) -> bool {
        self.current_block_index < self.blocks.len()
    }

    pub fn next_block(&mut self) {
        self.current_block_index += 1;
    }

    pub fn num_blocks(&self) -> usize {
        self.blocks.len()
    }

    pub fn num_elements(&self) -> usize {
        self.elements.len()
    }
}

impl Default for TopOpeBRepBuildBlockBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_builder_new() {
        let bb = TopOpeBRepBuildBlockBuilder::new();
        assert_eq!(bb.num_elements(), 0);
        assert_eq!(bb.num_blocks(), 0);
    }

    #[test]
    fn test_block_builder_add_element() {
        let mut bb = TopOpeBRepBuildBlockBuilder::new();
        let idx0 = bb.add_element(10);
        let idx1 = bb.add_element(20);
        assert_eq!(idx0, 0);
        assert_eq!(idx1, 1);
        assert_eq!(bb.element(0), Some(10));
        assert_eq!(bb.element(1), Some(20));
    }

    #[test]
    fn test_block_builder_validity() {
        let mut bb = TopOpeBRepBuildBlockBuilder::new();
        let idx = bb.add_element(5);
        assert!(bb.is_valid(idx));

        bb.set_valid(idx, false);
        assert!(!bb.is_valid(idx));
    }

    #[test]
    fn test_block_builder_make_block() {
        let mut bb = TopOpeBRepBuildBlockBuilder::new();
        let idx0 = bb.add_element(1);
        let idx1 = bb.add_element(2);
        bb.add_element(3);
        bb.set_valid(1, false);

        bb.make_block();
        assert_eq!(bb.num_blocks(), 1);
    }

    #[test]
    fn test_block_builder_iteration() {
        let mut bb = TopOpeBRepBuildBlockBuilder::new();
        bb.add_element(1);
        bb.add_element(2);
        bb.make_block();

        bb.init_block();
        assert!(bb.more_block());
        bb.next_block();
        assert!(!bb.more_block());
    }
}
