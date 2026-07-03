// FILE: top_ope_b_rep_ds_transition.rs
// occt: TopOpeBRepDS_Transition

/// State enumeration: IN, OUT, ON, UNKNOWN
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum State {
    /// Inside
    In = 0,
    /// Outside
    Out = 1,
    /// On boundary
    On = 2,
    /// Unknown state
    Unknown = 3,
}

impl State {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(State::In),
            1 => Some(State::Out),
            2 => Some(State::On),
            3 => Some(State::Unknown),
            _ => None,
        }
    }

    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

/// Shape type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ShapeEnum {
    Vertex = 0,
    Edge = 1,
    Wire = 2,
    Face = 3,
    Shell = 4,
    Solid = 5,
    CompSolid = 6,
    Compound = 7,
    Shape = 8,
}

impl ShapeEnum {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(ShapeEnum::Vertex),
            1 => Some(ShapeEnum::Edge),
            2 => Some(ShapeEnum::Wire),
            3 => Some(ShapeEnum::Face),
            4 => Some(ShapeEnum::Shell),
            5 => Some(ShapeEnum::Solid),
            6 => Some(ShapeEnum::CompSolid),
            7 => Some(ShapeEnum::Compound),
            8 => Some(ShapeEnum::Shape),
            _ => None,
        }
    }

    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

/// Orientation enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Orientation {
    Forward = 0,
    Reversed = 1,
    Internal = 2,
    External = 3,
}

impl Orientation {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Orientation::Forward),
            1 => Some(Orientation::Reversed),
            2 => Some(Orientation::Internal),
            3 => Some(Orientation::External),
            _ => None,
        }
    }

    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

/// A transition: state before and after an interference
#[derive(Debug, Clone)]
pub struct Transition {
    state_before: State,
    state_after: State,
    shape_before: ShapeEnum,
    shape_after: ShapeEnum,
    index: i32,
    index_before: i32,
    index_after: i32,
}

impl Transition {
    /// Create a new default Transition
    pub fn new() -> Self {
        Transition {
            state_before: State::Unknown,
            state_after: State::Unknown,
            shape_before: ShapeEnum::Face,
            shape_after: ShapeEnum::Face,
            index: 0,
            index_before: 0,
            index_after: 0,
        }
    }

    /// Create a Transition with states and shapes
    pub fn with_states(
        state_before: State,
        state_after: State,
        shape_before: ShapeEnum,
        shape_after: ShapeEnum,
    ) -> Self {
        Transition {
            state_before,
            state_after,
            shape_before,
            shape_after,
            index: 0,
            index_before: 0,
            index_after: 0,
        }
    }

    /// Create Transition from orientation
    pub fn from_orientation(o: Orientation) -> Self {
        let (state_before, state_after) = match o {
            Orientation::Forward => (State::Out, State::In),
            Orientation::Reversed => (State::In, State::Out),
            Orientation::Internal => (State::In, State::In),
            Orientation::External => (State::Out, State::Out),
        };

        Transition {
            state_before,
            state_after,
            shape_before: ShapeEnum::Face,
            shape_after: ShapeEnum::Face,
            index: 0,
            index_before: 0,
            index_after: 0,
        }
    }

    /// Set state before
    pub fn set_state_before(&mut self, s: State) {
        self.state_before = s;
    }

    /// Set state after
    pub fn set_state_after(&mut self, s: State) {
        self.state_after = s;
    }

    /// Set shape before
    pub fn set_shape_before(&mut self, se: ShapeEnum) {
        self.shape_before = se;
    }

    /// Set shape after
    pub fn set_shape_after(&mut self, se: ShapeEnum) {
        self.shape_after = se;
    }

    /// Get state before
    pub fn state_before(&self) -> State {
        self.state_before
    }

    /// Get state after
    pub fn state_after(&self) -> State {
        self.state_after
    }

    /// Get shape before
    pub fn shape_before(&self) -> ShapeEnum {
        self.shape_before
    }

    /// Get shape after
    pub fn shape_after(&self) -> ShapeEnum {
        self.shape_after
    }

    /// Set before (state and shape)
    pub fn set_before(&mut self, s: State, shape: ShapeEnum) {
        self.state_before = s;
        self.shape_before = shape;
    }

    /// Set after (state and shape)
    pub fn set_after(&mut self, s: State, shape: ShapeEnum) {
        self.state_after = s;
        self.shape_after = shape;
    }

    /// Set index
    pub fn set_index(&mut self, i: i32) {
        self.index = i;
    }

    /// Set index before
    pub fn set_index_before(&mut self, i: i32) {
        self.index_before = i;
    }

    /// Set index after
    pub fn set_index_after(&mut self, i: i32) {
        self.index_after = i;
    }

    /// Get index
    pub fn index(&self) -> i32 {
        self.index
    }

    /// Get index before
    pub fn index_before(&self) -> i32 {
        self.index_before
    }

    /// Get index after
    pub fn index_after(&self) -> i32 {
        self.index_after
    }
}

impl Default for Transition {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_enum() {
        assert_eq!(State::In.as_u8(), 0);
        assert_eq!(State::Out.as_u8(), 1);
        assert_eq!(State::On.as_u8(), 2);
        assert_eq!(State::Unknown.as_u8(), 3);
    }

    #[test]
    fn test_state_from_u8() {
        assert_eq!(State::from_u8(0), Some(State::In));
        assert_eq!(State::from_u8(1), Some(State::Out));
        assert_eq!(State::from_u8(2), Some(State::On));
        assert_eq!(State::from_u8(3), Some(State::Unknown));
        assert_eq!(State::from_u8(99), None);
    }

    #[test]
    fn test_transition_new() {
        let t = Transition::new();
        assert_eq!(t.state_before(), State::Unknown);
        assert_eq!(t.state_after(), State::Unknown);
        assert_eq!(t.shape_before(), ShapeEnum::Face);
        assert_eq!(t.shape_after(), ShapeEnum::Face);
    }

    #[test]
    fn test_transition_with_states() {
        let t = Transition::with_states(State::Out, State::In, ShapeEnum::Edge, ShapeEnum::Face);
        assert_eq!(t.state_before(), State::Out);
        assert_eq!(t.state_after(), State::In);
        assert_eq!(t.shape_before(), ShapeEnum::Edge);
        assert_eq!(t.shape_after(), ShapeEnum::Face);
    }

    #[test]
    fn test_transition_from_orientation() {
        let t = Transition::from_orientation(Orientation::Forward);
        assert_eq!(t.state_before(), State::Out);
        assert_eq!(t.state_after(), State::In);

        let t = Transition::from_orientation(Orientation::Reversed);
        assert_eq!(t.state_before(), State::In);
        assert_eq!(t.state_after(), State::Out);

        let t = Transition::from_orientation(Orientation::Internal);
        assert_eq!(t.state_before(), State::In);
        assert_eq!(t.state_after(), State::In);

        let t = Transition::from_orientation(Orientation::External);
        assert_eq!(t.state_before(), State::Out);
        assert_eq!(t.state_after(), State::Out);
    }

    #[test]
    fn test_transition_indices() {
        let mut t = Transition::new();
        t.set_index(10);
        t.set_index_before(5);
        t.set_index_after(15);
        assert_eq!(t.index(), 10);
        assert_eq!(t.index_before(), 5);
        assert_eq!(t.index_after(), 15);
    }

    #[test]
    fn test_shape_enum() {
        assert_eq!(ShapeEnum::Face.as_u8(), 3);
        assert_eq!(ShapeEnum::Edge.as_u8(), 1);
        assert_eq!(ShapeEnum::from_u8(3), Some(ShapeEnum::Face));
    }

    #[test]
    fn test_orientation_enum() {
        assert_eq!(Orientation::Forward.as_u8(), 0);
        assert_eq!(Orientation::Reversed.as_u8(), 1);
        assert_eq!(Orientation::from_u8(0), Some(Orientation::Forward));
    }
}
