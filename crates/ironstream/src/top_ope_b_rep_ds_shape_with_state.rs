// FILE: top_ope_b_rep_ds_shape_with_state.rs
// occt: TopOpeBRepDS_ShapeWithState

/// A shape with state information
#[derive(Debug, Clone)]
pub struct ShapeWithState {
    /// Shape identifier
    shape_id: usize,
    /// State information (IN=0, OUT=1, ON=2, UNKNOWN=3)
    state: u8,
}

impl ShapeWithState {
    /// Create new ShapeWithState
    pub fn new(shape_id: usize, state: u8) -> Self {
        ShapeWithState { shape_id, state }
    }

    /// Get shape id
    pub fn shape_id(&self) -> usize {
        self.shape_id
    }

    /// Set shape id
    pub fn set_shape_id(&mut self, id: usize) {
        self.shape_id = id;
    }

    /// Get state
    pub fn state(&self) -> u8 {
        self.state
    }

    /// Set state
    pub fn set_state(&mut self, state: u8) {
        self.state = state;
    }

    /// Check if state is IN (0)
    pub fn is_in(&self) -> bool {
        self.state == 0
    }

    /// Check if state is OUT (1)
    pub fn is_out(&self) -> bool {
        self.state == 1
    }

    /// Check if state is ON (2)
    pub fn is_on(&self) -> bool {
        self.state == 2
    }

    /// Check if state is UNKNOWN (3)
    pub fn is_unknown(&self) -> bool {
        self.state == 3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_with_state_new() {
        let s = ShapeWithState::new(5, 0);
        assert_eq!(s.shape_id(), 5);
        assert_eq!(s.state(), 0);
    }

    #[test]
    fn test_shape_with_state_setters() {
        let mut s = ShapeWithState::new(5, 1);
        s.set_shape_id(10);
        s.set_state(2);
        assert_eq!(s.shape_id(), 10);
        assert_eq!(s.state(), 2);
    }

    #[test]
    fn test_shape_with_state_checks() {
        let s_in = ShapeWithState::new(1, 0);
        assert!(s_in.is_in());
        assert!(!s_in.is_out());

        let s_out = ShapeWithState::new(1, 1);
        assert!(s_out.is_out());
        assert!(!s_out.is_in());

        let s_on = ShapeWithState::new(1, 2);
        assert!(s_on.is_on());

        let s_unknown = ShapeWithState::new(1, 3);
        assert!(s_unknown.is_unknown());
    }
}
