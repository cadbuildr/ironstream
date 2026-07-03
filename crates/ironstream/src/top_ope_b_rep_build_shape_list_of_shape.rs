// FILE: top_ope_b_rep_build_shape_list_of_shape.rs
// occt: TopOpeBRepBuild_ShapeListOfShape

#[derive(Debug, Clone)]
pub struct TopOpeBRepBuildShapeListOfShape {
    shape_id: i32,
    shapes: Vec<i32>,
}

impl TopOpeBRepBuildShapeListOfShape {
    pub fn new(shape_id: i32) -> Self {
        TopOpeBRepBuildShapeListOfShape {
            shape_id,
            shapes: Vec::new(),
        }
    }

    pub fn shape_id(&self) -> i32 {
        self.shape_id
    }

    pub fn add_shape(&mut self, shape_id: i32) {
        self.shapes.push(shape_id);
    }

    pub fn shapes(&self) -> &[i32] {
        &self.shapes
    }

    pub fn num_shapes(&self) -> usize {
        self.shapes.len()
    }
}

impl Default for TopOpeBRepBuildShapeListOfShape {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_list_of_shape_new() {
        let slos = TopOpeBRepBuildShapeListOfShape::new(5);
        assert_eq!(slos.shape_id(), 5);
        assert_eq!(slos.num_shapes(), 0);
    }

    #[test]
    fn test_shape_list_of_shape_add() {
        let mut slos = TopOpeBRepBuildShapeListOfShape::new(1);
        slos.add_shape(10);
        slos.add_shape(20);
        assert_eq!(slos.num_shapes(), 2);
    }

    #[test]
    fn test_shape_list_of_shape_shapes() {
        let mut slos = TopOpeBRepBuildShapeListOfShape::new(1);
        slos.add_shape(5);
        slos.add_shape(15);
        assert_eq!(slos.shapes().len(), 2);
    }
}
