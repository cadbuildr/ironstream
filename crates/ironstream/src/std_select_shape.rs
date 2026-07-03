// FILE: std_select_shape.rs
// occt: StdSelect_Shape

//! A shape object for selection purposes.

#[derive(Clone, Debug)]
pub struct Shape {
    pub shape_id: u32,
    pub shape_type: ShapeType,
    pub is_selectable: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShapeType {
    Vertex,
    Edge,
    Wire,
    Face,
    Shell,
    Solid,
    CompSolid,
    Compound,
}

impl Shape {
    pub fn new(shape_id: u32, shape_type: ShapeType) -> Self {
        Shape {
            shape_id,
            shape_type,
            is_selectable: true,
        }
    }

    pub fn with_selectability(mut self, selectable: bool) -> Self {
        self.is_selectable = selectable;
        self
    }

    pub fn set_selectable(&mut self, selectable: bool) {
        self.is_selectable = selectable;
    }

    pub fn is_selectable(&self) -> bool {
        self.is_selectable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape_creation() {
        let shape = Shape::new(1, ShapeType::Face);
        assert_eq!(shape.shape_id, 1);
        assert_eq!(shape.shape_type, ShapeType::Face);
        assert!(shape.is_selectable);
    }

    #[test]
    fn shape_builder() {
        let shape = Shape::new(1, ShapeType::Edge)
            .with_selectability(false);

        assert!(!shape.is_selectable());
    }

    #[test]
    fn shape_mutate() {
        let mut shape = Shape::new(1, ShapeType::Vertex);
        shape.set_selectable(false);
        assert!(!shape.is_selectable);
    }
}
