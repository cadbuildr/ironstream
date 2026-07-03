// FILE: sweep_num_shape_tool.rs
// occt: Sweep_NumShapeTool

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

/// Represents a numbered shape (edge, vertex, etc.)
#[derive(Clone, Copy, Debug)]
pub struct SweepNumShape {
    my_type: TopAbsShapeEnum,
    my_index: i32,
    my_closed: bool,
    my_beg_inf: bool,
    my_end_inf: bool,
}

impl SweepNumShape {
    pub fn new() -> Self {
        SweepNumShape {
            my_type: TopAbsShapeEnum::Edge,
            my_index: 0,
            my_closed: false,
            my_beg_inf: false,
            my_end_inf: false,
        }
    }

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

    pub fn index(&self) -> i32 {
        self.my_index
    }

    pub fn shape_type(&self) -> TopAbsShapeEnum {
        self.my_type
    }

    pub fn closed(&self) -> bool {
        self.my_closed
    }

    pub fn beg_infinite(&self) -> bool {
        self.my_beg_inf
    }

    pub fn end_infinite(&self) -> bool {
        self.my_end_inf
    }
}

impl Default for SweepNumShape {
    fn default() -> Self {
        SweepNumShape::new()
    }
}

/// This class provides the indexation and type analysis services
/// required by the NumShape Directing Shapes of Swept Primitives.
pub struct SweepNumShapeTool {
    my_num_shape: SweepNumShape,
}

impl SweepNumShapeTool {
    /// Create a new NumShapeTool with aShape.
    /// The Tool must prepare an indexation for all the subshapes of this shape.
    pub fn new(a_shape: &SweepNumShape) -> Self {
        SweepNumShapeTool {
            my_num_shape: *a_shape,
        }
    }

    /// Returns the number of subshapes in the shape.
    pub fn nb_shapes(&self) -> i32 {
        self.my_num_shape.index()
    }

    /// Returns the index of aShape.
    pub fn index(&self, _a_shape: &SweepNumShape) -> i32 {
        1
    }

    /// Returns the Shape at index anIndex
    pub fn shape(&self, an_index: i32) -> SweepNumShape {
        if an_index == 1 {
            self.my_num_shape
        } else if an_index == 2 && self.my_num_shape.index() >= 2 {
            SweepNumShape::with_params(
                2,
                TopAbsShapeEnum::Vertex,
                self.my_num_shape.closed(),
                false,
                false,
            )
        } else {
            SweepNumShape::new()
        }
    }

    /// Returns the type of aShape.
    pub fn shape_type(&self, _a_shape: &SweepNumShape) -> TopAbsShapeEnum {
        self.my_num_shape.shape_type()
    }

    /// Returns the orientation of aShape.
    pub fn orientation(&self, _a_shape: &SweepNumShape) -> TopAbsOrientation {
        TopAbsOrientation::Forward
    }

    /// Returns true if there is a First Vertex in the Shape.
    pub fn has_first_vertex(&self) -> bool {
        self.my_num_shape.index() > 0
    }

    /// Returns true if there is a Last Vertex in the Shape.
    pub fn has_last_vertex(&self) -> bool {
        self.my_num_shape.index() > 1 || self.my_num_shape.closed()
    }

    /// Returns the first vertex.
    pub fn first_vertex(&self) -> SweepNumShape {
        SweepNumShape::with_params(
            1,
            TopAbsShapeEnum::Vertex,
            self.my_num_shape.closed(),
            false,
            false,
        )
    }

    /// Returns the last vertex.
    pub fn last_vertex(&self) -> SweepNumShape {
        if self.my_num_shape.index() > 1 {
            SweepNumShape::with_params(
                2,
                TopAbsShapeEnum::Vertex,
                self.my_num_shape.closed(),
                false,
                false,
            )
        } else {
            SweepNumShape::with_params(
                1,
                TopAbsShapeEnum::Vertex,
                self.my_num_shape.closed(),
                false,
                false,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_creation() {
        let shape = SweepNumShape::new();
        let tool = SweepNumShapeTool::new(&shape);
        assert_eq!(tool.nb_shapes(), 0);
    }

    #[test]
    fn test_tool_with_two_vertices() {
        let shape = SweepNumShape::with_params(
            2,
            TopAbsShapeEnum::Edge,
            false,
            false,
            false,
        );
        let tool = SweepNumShapeTool::new(&shape);
        assert_eq!(tool.nb_shapes(), 2);
    }

    #[test]
    fn test_tool_shape_retrieval() {
        let shape = SweepNumShape::with_params(
            2,
            TopAbsShapeEnum::Edge,
            false,
            false,
            false,
        );
        let tool = SweepNumShapeTool::new(&shape);

        let shape_1 = tool.shape(1);
        assert_eq!(shape_1.index(), 2);

        let shape_2 = tool.shape(2);
        assert_eq!(shape_2.shape_type(), TopAbsShapeEnum::Vertex);
    }

    #[test]
    fn test_tool_has_first_vertex() {
        let shape = SweepNumShape::with_params(
            2,
            TopAbsShapeEnum::Edge,
            false,
            false,
            false,
        );
        let tool = SweepNumShapeTool::new(&shape);
        assert_eq!(tool.has_first_vertex(), true);
    }

    #[test]
    fn test_tool_has_last_vertex() {
        let shape = SweepNumShape::with_params(
            2,
            TopAbsShapeEnum::Edge,
            false,
            false,
            false,
        );
        let tool = SweepNumShapeTool::new(&shape);
        assert_eq!(tool.has_last_vertex(), true);
    }

    #[test]
    fn test_tool_first_vertex() {
        let shape = SweepNumShape::with_params(
            2,
            TopAbsShapeEnum::Edge,
            false,
            false,
            false,
        );
        let tool = SweepNumShapeTool::new(&shape);

        let first = tool.first_vertex();
        assert_eq!(first.index(), 1);
        assert_eq!(first.shape_type(), TopAbsShapeEnum::Vertex);
    }

    #[test]
    fn test_tool_last_vertex() {
        let shape = SweepNumShape::with_params(
            2,
            TopAbsShapeEnum::Edge,
            false,
            false,
            false,
        );
        let tool = SweepNumShapeTool::new(&shape);

        let last = tool.last_vertex();
        assert_eq!(last.index(), 2);
        assert_eq!(last.shape_type(), TopAbsShapeEnum::Vertex);
    }

    #[test]
    fn test_tool_closed_edge() {
        let shape = SweepNumShape::with_params(
            0,
            TopAbsShapeEnum::Edge,
            true,
            false,
            false,
        );
        let tool = SweepNumShapeTool::new(&shape);
        assert_eq!(tool.has_last_vertex(), true);
    }

    #[test]
    fn test_tool_orientation() {
        let shape = SweepNumShape::new();
        let tool = SweepNumShapeTool::new(&shape);
        let orientation = tool.orientation(&shape);
        assert_eq!(orientation, TopAbsOrientation::Forward);
    }
}
