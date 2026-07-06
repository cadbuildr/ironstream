// FILE: bop_tools_list_of_connexity_block.rs
// occt: BOPTools_ListOfConnexityBlock

use std::collections::VecDeque;

/// A block of connected shapes in Boolean operations.
/// Mirrors BOPTools_ConnexityBlock from OCCT.
#[derive(Debug, Clone)]
pub struct ConnexityBlock {
    shapes: VecDeque<usize>,   // Shape IDs instead of TopoDS_Shape
    loops: VecDeque<usize>,    // Loop shape IDs
    is_regular: bool,          // Regular/irregular flag
}

impl ConnexityBlock {
    /// Creates a new connexity block.
    pub fn new() -> Self {
        ConnexityBlock {
            shapes: VecDeque::new(),
            loops: VecDeque::new(),
            is_regular: true,
        }
    }

    /// Returns the shapes list.
    pub fn shapes(&self) -> Vec<usize> {
        self.shapes.iter().copied().collect()
    }

    /// Returns a mutable reference to the shapes list.
    pub fn change_shapes(&mut self) -> &mut VecDeque<usize> {
        &mut self.shapes
    }

    /// Sets the regular flag.
    pub fn set_regular(&mut self, flag: bool) {
        self.is_regular = flag;
    }

    /// Checks if the block is regular.
    pub fn is_regular(&self) -> bool {
        self.is_regular
    }

    /// Returns the loops list.
    pub fn loops(&self) -> Vec<usize> {
        self.loops.iter().copied().collect()
    }

    /// Returns a mutable reference to the loops list.
    pub fn change_loops(&mut self) -> &mut VecDeque<usize> {
        &mut self.loops
    }

    /// Adds a shape to the shapes list.
    pub fn add_shape(&mut self, shape_id: usize) {
        self.shapes.push_back(shape_id);
    }

    /// Adds a loop to the loops list.
    pub fn add_loop(&mut self, loop_id: usize) {
        self.loops.push_back(loop_id);
    }

    /// Returns the number of shapes.
    pub fn nb_shapes(&self) -> usize {
        self.shapes.len()
    }

    /// Returns the number of loops.
    pub fn nb_loops(&self) -> usize {
        self.loops.len()
    }

    /// Clears all shapes and loops.
    pub fn clear(&mut self) {
        self.shapes.clear();
        self.loops.clear();
        self.is_regular = true;
    }
}

impl Default for ConnexityBlock {
    fn default() -> Self {
        Self::new()
    }
}

/// Deprecated type alias: list of connexity blocks.
/// This is a newtype wrapping VecDeque<ConnexityBlock> to match OCCT's NCollection_List semantics.
pub struct BoptoolsListOfConnexityBlock {
    data: VecDeque<ConnexityBlock>,
}

impl BoptoolsListOfConnexityBlock {
    /// Creates an empty list.
    pub fn new() -> Self {
        BoptoolsListOfConnexityBlock {
            data: VecDeque::new(),
        }
    }

    /// Appends a connexity block to the list.
    pub fn push(&mut self, block: ConnexityBlock) {
        self.data.push_back(block);
    }

    /// Prepends a connexity block to the list.
    pub fn push_front(&mut self, block: ConnexityBlock) {
        self.data.push_front(block);
    }

    /// Returns the number of blocks.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the list is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Accesses a block by index.
    pub fn get(&self, index: usize) -> Option<&ConnexityBlock> {
        self.data.get(index)
    }

    /// Mutably accesses a block by index.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut ConnexityBlock> {
        self.data.get_mut(index)
    }

    /// Returns an iterator over the blocks.
    pub fn iter(&self) -> impl Iterator<Item = &ConnexityBlock> {
        self.data.iter()
    }

    /// Returns a mutable iterator over the blocks.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut ConnexityBlock> {
        self.data.iter_mut()
    }

    /// Clears all blocks.
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Removes and returns the first block, if any.
    pub fn pop_front(&mut self) -> Option<ConnexityBlock> {
        self.data.pop_front()
    }

    /// Removes and returns the last block, if any.
    pub fn pop_back(&mut self) -> Option<ConnexityBlock> {
        self.data.pop_back()
    }
}

impl Default for BoptoolsListOfConnexityBlock {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation() {
        let block = ConnexityBlock::new();
        assert!(block.is_regular());
        assert_eq!(block.nb_shapes(), 0);
        assert_eq!(block.nb_loops(), 0);
    }

    #[test]
    fn test_block_add_shapes() {
        let mut block = ConnexityBlock::new();
        block.add_shape(1);
        block.add_shape(2);
        block.add_shape(3);
        assert_eq!(block.nb_shapes(), 3);
    }

    #[test]
    fn test_block_add_loops() {
        let mut block = ConnexityBlock::new();
        block.add_loop(10);
        block.add_loop(11);
        assert_eq!(block.nb_loops(), 2);
    }

    #[test]
    fn test_block_regular_flag() {
        let mut block = ConnexityBlock::new();
        assert!(block.is_regular());
        block.set_regular(false);
        assert!(!block.is_regular());
        block.set_regular(true);
        assert!(block.is_regular());
    }

    #[test]
    fn test_block_clear() {
        let mut block = ConnexityBlock::new();
        block.add_shape(1);
        block.add_loop(2);
        assert_eq!(block.nb_shapes(), 1);
        assert_eq!(block.nb_loops(), 1);

        block.clear();
        assert_eq!(block.nb_shapes(), 0);
        assert_eq!(block.nb_loops(), 0);
        assert!(block.is_regular());
    }

    #[test]
    fn test_block_change_shapes() {
        let mut block = ConnexityBlock::new();
        block.change_shapes().push_back(42);
        assert_eq!(block.nb_shapes(), 1);
    }

    #[test]
    fn test_block_change_loops() {
        let mut block = ConnexityBlock::new();
        block.change_loops().push_back(99);
        assert_eq!(block.nb_loops(), 1);
    }

    #[test]
    fn test_list_creation() {
        let list = BoptoolsListOfConnexityBlock::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_list_push() {
        let mut list = BoptoolsListOfConnexityBlock::new();
        let block = ConnexityBlock::new();
        list.push(block);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_list_push_front() {
        let mut list = BoptoolsListOfConnexityBlock::new();
        let block1 = ConnexityBlock::new();
        let block2 = ConnexityBlock::new();
        list.push(block1);
        list.push_front(block2);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_list_multiple_blocks() {
        let mut list = BoptoolsListOfConnexityBlock::new();
        for i in 0..5 {
            let mut block = ConnexityBlock::new();
            block.add_shape(i as usize);
            list.push(block);
        }
        assert_eq!(list.len(), 5);
    }

    #[test]
    fn test_list_get() {
        let mut list = BoptoolsListOfConnexityBlock::new();
        let mut block = ConnexityBlock::new();
        block.add_shape(42);
        list.push(block);

        let retrieved = list.get(0).unwrap();
        assert_eq!(retrieved.nb_shapes(), 1);
    }

    #[test]
    fn test_list_get_mut() {
        let mut list = BoptoolsListOfConnexityBlock::new();
        let block = ConnexityBlock::new();
        list.push(block);

        if let Some(b) = list.get_mut(0) {
            b.add_shape(99);
        }
        assert_eq!(list.get(0).unwrap().nb_shapes(), 1);
    }

    #[test]
    fn test_list_clear() {
        let mut list = BoptoolsListOfConnexityBlock::new();
        list.push(ConnexityBlock::new());
        list.push(ConnexityBlock::new());
        assert_eq!(list.len(), 2);

        list.clear();
        assert!(list.is_empty());
    }

    #[test]
    fn test_list_pop_front() {
        let mut list = BoptoolsListOfConnexityBlock::new();
        list.push(ConnexityBlock::new());
        list.push(ConnexityBlock::new());

        let popped = list.pop_front();
        assert!(popped.is_some());
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_list_pop_back() {
        let mut list = BoptoolsListOfConnexityBlock::new();
        list.push(ConnexityBlock::new());
        list.push(ConnexityBlock::new());

        let popped = list.pop_back();
        assert!(popped.is_some());
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_list_iterator() {
        let mut list = BoptoolsListOfConnexityBlock::new();
        for i in 0..3 {
            let mut block = ConnexityBlock::new();
            block.add_shape(i);
            list.push(block);
        }

        let count = list.iter().count();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_list_iter_mut() {
        let mut list = BoptoolsListOfConnexityBlock::new();
        list.push(ConnexityBlock::new());
        list.push(ConnexityBlock::new());

        for block in list.iter_mut() {
            block.set_regular(false);
        }

        for block in list.iter() {
            assert!(!block.is_regular());
        }
    }
}
