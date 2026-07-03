// FILE: sweep_num_shape.rs
// occt: Sweep_NumShape

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TopAbsShapeEnum {
    Compound,
    Compsolid,
    Solid,
    Shell,
    Face,
    Wire,
    Edge,
    Vertex,
    Shape,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TopAbsOrientation {
    Forward,
    Reversed,
    Internal,
    External,
}

/// Gives a simple indexed representation of a Directing Edge topology.
#[derive(Clone, Copy, Debug)]
pub struct SweepNumShape {
    my_type: TopAbsShapeEnum,
    my_index: i32,
    my_closed: bool,
    my_beg_inf: bool,
    my_end_inf: bool,
}

impl SweepNumShape {
    /// Creates a dummy indexed edge.
    pub fn new() -> Self {
        SweepNumShape {
            my_type: TopAbsShapeEnum::Edge,
            my_index: 0,
            my_closed: false,
            my_beg_inf: false,
            my_end_inf: false,
        }
    }

    /// Creates a new simple indexed edge.
    ///
    /// For an Edge: Index is the number of vertices (0, 1 or 2),
    /// Type is TopAbs_EDGE, Closed is true if it is a closed edge,
    /// BegInf is true if the Edge is infinite at the beginning,
    /// EndInf is true if the edge is infinite at the end.
    ///
    /// For a Vertex: Index is the index of the vertex in the edge (1 or 2),
    /// Type is TopAbsVERTEX, all the other fields have no meaning.
    pub fn with_params(
        index: i32,
        shape_type: TopAbsShapeEnum,
        closed: bool,
        beg_inf: bool,
        end_inf: bool,
    ) -> Self {
        SweepNumShape {
            my_type: shape_type,
            my_index: index,
            my_closed: closed,
            my_beg_inf: beg_inf,
            my_end_inf: end_inf,
        }
    }

    /// Reinitialize a simple indexed edge.
    pub fn init(
        &mut self,
        index: i32,
        shape_type: TopAbsShapeEnum,
        closed: bool,
        beg_inf: bool,
        end_inf: bool,
    ) {
        self.my_type = shape_type;
        self.my_index = index;
        self.my_closed = closed;
        self.my_beg_inf = beg_inf;
        self.my_end_inf = end_inf;
    }

    /// Returns the index.
    pub fn index(&self) -> i32 {
        self.my_index
    }

    /// Returns the shape type.
    pub fn shape_type(&self) -> TopAbsShapeEnum {
        self.my_type
    }

    /// Returns true if the edge is closed.
    pub fn closed(&self) -> bool {
        self.my_closed
    }

    /// Returns true if the edge is infinite at the beginning.
    pub fn beg_infinite(&self) -> bool {
        self.my_beg_inf
    }

    /// Returns true if the edge is infinite at the end.
    pub fn end_infinite(&self) -> bool {
        self.my_end_inf
    }

    /// Returns the orientation.
    pub fn orientation(&self) -> TopAbsOrientation {
        TopAbsOrientation::Forward
    }
}

impl Default for SweepNumShape {
    fn default() -> Self {
        SweepNumShape::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sweep_num_shape_default() {
        let shape = SweepNumShape::new();
        assert_eq!(shape.index(), 0);
        assert_eq!(shape.shape_type(), TopAbsShapeEnum::Edge);
        assert_eq!(shape.closed(), false);
        assert_eq!(shape.beg_infinite(), false);
        assert_eq!(shape.end_infinite(), false);
    }

    #[test]
    fn test_sweep_num_shape_with_params() {
        let shape = SweepNumShape::with_params(
            2,
            TopAbsShapeEnum::Edge,
            true,
            false,
            true,
        );
        assert_eq!(shape.index(), 2);
        assert_eq!(shape.shape_type(), TopAbsShapeEnum::Edge);
        assert_eq!(shape.closed(), true);
        assert_eq!(shape.beg_infinite(), false);
        assert_eq!(shape.end_infinite(), true);
    }

    #[test]
    fn test_sweep_num_shape_vertex() {
        let shape = SweepNumShape::with_params(
            1,
            TopAbsShapeEnum::Vertex,
            false,
            false,
            false,
        );
        assert_eq!(shape.index(), 1);
        assert_eq!(shape.shape_type(), TopAbsShapeEnum::Vertex);
    }

    #[test]
    fn test_sweep_num_shape_init() {
        let mut shape = SweepNumShape::new();
        shape.init(3, TopAbsShapeEnum::Edge, true, true, false);

        assert_eq!(shape.index(), 3);
        assert_eq!(shape.shape_type(), TopAbsShapeEnum::Edge);
        assert_eq!(shape.closed(), true);
        assert_eq!(shape.beg_infinite(), true);
        assert_eq!(shape.end_infinite(), false);
    }

    #[test]
    fn test_sweep_num_shape_orientation() {
        let shape = SweepNumShape::new();
        assert_eq!(shape.orientation(), TopAbsOrientation::Forward);
    }

    #[test]
    fn test_sweep_num_shape_default_trait() {
        let shape = SweepNumShape::default();
        assert_eq!(shape.index(), 0);
        assert_eq!(shape.closed(), false);
    }

    #[test]
    fn test_sweep_num_shape_copy() {
        let shape1 = SweepNumShape::with_params(2, TopAbsShapeEnum::Edge, true, false, true);
        let shape2 = shape1;

        assert_eq!(shape1.index(), shape2.index());
        assert_eq!(shape1.closed(), shape2.closed());
    }
}
