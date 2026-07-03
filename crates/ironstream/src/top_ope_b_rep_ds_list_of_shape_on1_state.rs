// FILE: top_ope_b_rep_ds_list_of_shape_on1_state.rs
// occt: TopOpeBRepDS_ListOfShapeOn1State

/// A shape with state information
#[derive(Debug, Clone)]
pub struct ShapeOn1State {
    /// Shape identifier
    shape_id: usize,
    /// State value
    state: u8,
}

impl ShapeOn1State {
    /// Create new ShapeOn1State
    pub fn new(shape_id: usize, state: u8) -> Self {
        ShapeOn1State { shape_id, state }
    }

    /// Get shape id
    pub fn shape_id(&self) -> usize {
        self.shape_id
    }

    /// Get state
    pub fn state(&self) -> u8 {
        self.state
    }

    /// Set state
    pub fn set_state(&mut self, state: u8) {
        self.state = state;
    }
}

/// List of shapes with state information
#[derive(Debug, Clone)]
pub struct ListOfShapeOn1State {
    /// List of shapes with state
    items: Vec<ShapeOn1State>,
}

impl ListOfShapeOn1State {
    /// Create new list
    pub fn new() -> Self {
        ListOfShapeOn1State {
            items: Vec::new(),
        }
    }

    /// Add a shape with state
    pub fn append(&mut self, shape_id: usize, state: u8) {
        self.items.push(ShapeOn1State::new(shape_id, state));
    }

    /// Get items
    pub fn items(&self) -> &[ShapeOn1State] {
        &self.items
    }

    /// Get mutable items
    pub fn items_mut(&mut self) -> &mut Vec<ShapeOn1State> {
        &mut self.items
    }

    /// Clear list
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Get length
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl Default for ListOfShapeOn1State {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_on1_state_new() {
        let s = ShapeOn1State::new(5, 2);
        assert_eq!(s.shape_id(), 5);
        assert_eq!(s.state(), 2);
    }

    #[test]
    fn test_shape_on1_state_set_state() {
        let mut s = ShapeOn1State::new(5, 2);
        s.set_state(3);
        assert_eq!(s.state(), 3);
    }

    #[test]
    fn test_list_of_shape_on1_state_new() {
        let l = ListOfShapeOn1State::new();
        assert!(l.is_empty());
        assert_eq!(l.len(), 0);
    }

    #[test]
    fn test_list_append() {
        let mut l = ListOfShapeOn1State::new();
        l.append(1, 0);
        l.append(2, 1);
        l.append(3, 2);
        assert_eq!(l.len(), 3);
        assert_eq!(l.items()[0].shape_id(), 1);
        assert_eq!(l.items()[1].state(), 1);
    }

    #[test]
    fn test_list_clear() {
        let mut l = ListOfShapeOn1State::new();
        l.append(1, 0);
        l.append(2, 1);
        assert_eq!(l.len(), 2);
        l.clear();
        assert!(l.is_empty());
    }
}
