// FILE: bop_tools_set.rs
// occt: BOPTools_Set

/// A set of shapes for boolean operations.
#[derive(Clone, Debug)]
pub struct BopToolsSet {
    shapes: Vec<Vec<u8>>,
    nb_shapes: usize,
    sum: usize,
}

impl BopToolsSet {
    /// Create an empty set.
    pub fn new() -> Self {
        BopToolsSet {
            shapes: Vec::new(),
            nb_shapes: 0,
            sum: 0,
        }
    }

    /// Add a shape to the set.
    pub fn add(&mut self, shape_data: Vec<u8>, _shape_type: u8) {
        let contrib = shape_data.len();
        self.shapes.push(shape_data);
        self.nb_shapes += 1;
        self.sum = self.sum.wrapping_add(contrib as usize);
    }

    /// Get the number of shapes.
    pub fn nb_shapes(&self) -> usize {
        self.nb_shapes
    }

    /// Get the shapes.
    pub fn shapes(&self) -> &[Vec<u8>] {
        &self.shapes
    }

    /// Get the hash sum.
    pub fn get_sum(&self) -> usize {
        self.sum
    }

    /// Check if two sets are equal.
    pub fn is_equal(&self, other: &BopToolsSet) -> bool {
        self.nb_shapes == other.nb_shapes && self.sum == other.sum
    }

    /// Clear the set.
    pub fn clear(&mut self) {
        self.shapes.clear();
        self.nb_shapes = 0;
        self.sum = 0;
    }
}

impl Default for BopToolsSet {
    fn default() -> Self {
        Self::new()
    }
}

impl std::cmp::PartialEq for BopToolsSet {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other)
    }
}

impl std::hash::Hash for BopToolsSet {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.sum.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let set = BopToolsSet::new();
        assert_eq!(set.nb_shapes(), 0);
        assert_eq!(set.get_sum(), 0);
    }

    #[test]
    fn test_add() {
        let mut set = BopToolsSet::new();
        set.add(vec![1, 2, 3], 0);
        assert_eq!(set.nb_shapes(), 1);
        assert!(set.get_sum() > 0);
    }

    #[test]
    fn test_is_equal() {
        let mut set1 = BopToolsSet::new();
        let mut set2 = BopToolsSet::new();

        set1.add(vec![1, 2], 0);
        set2.add(vec![1, 2], 0);

        assert_eq!(set1, set2);
    }

    #[test]
    fn test_clear() {
        let mut set = BopToolsSet::new();
        set.add(vec![1, 2, 3], 0);
        set.clear();
        assert_eq!(set.nb_shapes(), 0);
        assert_eq!(set.get_sum(), 0);
    }
}
