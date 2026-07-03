// FILE: int_tools_root.rs
// occt: IntTools_Root

/// Represents a root of a 1D function (for edge/surface intersection).
#[derive(Clone, Copy, Debug)]
pub struct IntToolsRoot {
    root: f64,
    root_type: i32,
    state_before: TopAbsState,
    state_after: TopAbsState,
    layer_height: f64,
    t1: f64,
    t2: f64,
    f1: f64,
    f2: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TopAbsState {
    In = 0,
    Out = 1,
    On = 2,
    Unknown = 3,
}

impl IntToolsRoot {
    /// Create an empty root.
    pub fn new() -> Self {
        IntToolsRoot {
            root: 0.0,
            root_type: 0,
            state_before: TopAbsState::Unknown,
            state_after: TopAbsState::Unknown,
            layer_height: 0.0,
            t1: 0.0,
            t2: 0.0,
            f1: 0.0,
            f2: 0.0,
        }
    }

    /// Create a root with value and type.
    pub fn with_root_type(root: f64, root_type: i32) -> Self {
        IntToolsRoot {
            root,
            root_type,
            ..Self::new()
        }
    }

    /// Set the root value.
    pub fn set_root(&mut self, root: f64) {
        self.root = root;
    }

    /// Set the root type.
    pub fn set_type(&mut self, root_type: i32) {
        self.root_type = root_type;
    }

    /// Set the state before the root.
    pub fn set_state_before(&mut self, state: TopAbsState) {
        self.state_before = state;
    }

    /// Set the state after the root.
    pub fn set_state_after(&mut self, state: TopAbsState) {
        self.state_after = state;
    }

    /// Set the layer height (not used in Edge/Edge).
    pub fn set_layer_height(&mut self, height: f64) {
        self.layer_height = height;
    }

    /// Set the interval [t1, t2] and function values f1, f2.
    pub fn set_interval(&mut self, t1: f64, t2: f64, f1: f64, f2: f64) {
        self.t1 = t1;
        self.t2 = t2;
        self.f1 = f1;
        self.f2 = f2;
    }

    /// Get the root value.
    pub fn root(&self) -> f64 {
        self.root
    }

    /// Get the root type.
    pub fn root_type(&self) -> i32 {
        self.root_type
    }

    /// Get the state before.
    pub fn state_before(&self) -> TopAbsState {
        self.state_before
    }

    /// Get the state after.
    pub fn state_after(&self) -> TopAbsState {
        self.state_after
    }

    /// Get the layer height.
    pub fn layer_height(&self) -> f64 {
        self.layer_height
    }

    /// Get the interval and function values.
    pub fn interval(&self) -> (f64, f64, f64, f64) {
        (self.t1, self.t2, self.f1, self.f2)
    }

    /// Check if the root is valid.
    pub fn is_valid(&self) -> bool {
        match (self.state_before, self.state_after) {
            (TopAbsState::Out, TopAbsState::In) => true,
            (TopAbsState::Out, TopAbsState::On) => true,
            (TopAbsState::On, TopAbsState::Out) => true,
            (TopAbsState::In, TopAbsState::Out) => true,
            _ => false,
        }
    }
}

impl Default for IntToolsRoot {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let root = IntToolsRoot::new();
        assert_eq!(root.root(), 0.0);
        assert_eq!(root.root_type(), 0);
    }

    #[test]
    fn test_with_root_type() {
        let root = IntToolsRoot::with_root_type(1.5, 2);
        assert_eq!(root.root(), 1.5);
        assert_eq!(root.root_type(), 2);
    }

    #[test]
    fn test_set_states() {
        let mut root = IntToolsRoot::new();
        root.set_state_before(TopAbsState::Out);
        root.set_state_after(TopAbsState::In);
        assert_eq!(root.state_before(), TopAbsState::Out);
        assert_eq!(root.state_after(), TopAbsState::In);
    }

    #[test]
    fn test_is_valid() {
        let mut root = IntToolsRoot::new();
        root.set_state_before(TopAbsState::Out);
        root.set_state_after(TopAbsState::In);
        assert!(root.is_valid());

        let mut root2 = IntToolsRoot::new();
        root2.set_state_before(TopAbsState::In);
        root2.set_state_after(TopAbsState::In);
        assert!(!root2.is_valid());
    }

    #[test]
    fn test_interval() {
        let mut root = IntToolsRoot::new();
        root.set_interval(1.0, 2.0, 0.5, 1.5);
        let (t1, t2, f1, f2) = root.interval();
        assert_eq!(t1, 1.0);
        assert_eq!(t2, 2.0);
        assert_eq!(f1, 0.5);
        assert_eq!(f2, 1.5);
    }
}
