// FILE: top_ope_b_rep_build_shape_set.rs
// occt: TopOpeBRepBuild_ShapeSet

#[derive(Debug, Clone)]
pub struct TopOpeBRepBuildShapeSet {
    shapes: Vec<i32>,
    start_shapes: Vec<i32>,
    sub_shape_type: u32,
    current_shape_index: usize,
    check_shape: bool,
}

impl TopOpeBRepBuildShapeSet {
    pub fn new(sub_shape_type: u32) -> Self {
        TopOpeBRepBuildShapeSet {
            shapes: Vec::new(),
            start_shapes: Vec::new(),
            sub_shape_type,
            current_shape_index: 0,
            check_shape: true,
        }
    }

    pub fn add_shape(&mut self, shape_id: i32) {
        self.shapes.push(shape_id);
    }

    pub fn add_start_element(&mut self, shape_id: i32) {
        self.start_shapes.push(shape_id);
    }

    pub fn add_element(&mut self, shape_id: i32) {
        self.shapes.push(shape_id);
    }

    pub fn start_elements(&self) -> &[i32] {
        &self.start_shapes
    }

    pub fn init_shapes(&mut self) {
        self.current_shape_index = 0;
    }

    pub fn more_shapes(&self) -> bool {
        self.current_shape_index < self.shapes.len()
    }

    pub fn next_shape(&mut self) {
        self.current_shape_index += 1;
    }

    pub fn shape(&self) -> Option<i32> {
        if self.current_shape_index < self.shapes.len() {
            Some(self.shapes[self.current_shape_index])
        } else {
            None
        }
    }

    pub fn check_shape(&self) -> bool {
        self.check_shape
    }

    pub fn set_check_shape(&mut self, check: bool) {
        self.check_shape = check;
    }

    pub fn num_shapes(&self) -> usize {
        self.shapes.len()
    }
}

impl Default for TopOpeBRepBuildShapeSet {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_set_new() {
        let ss = TopOpeBRepBuildShapeSet::new(1);
        assert_eq!(ss.num_shapes(), 0);
        assert!(ss.check_shape());
    }

    #[test]
    fn test_shape_set_add_shape() {
        let mut ss = TopOpeBRepBuildShapeSet::new(1);
        ss.add_shape(10);
        ss.add_shape(20);
        assert_eq!(ss.num_shapes(), 2);
    }

    #[test]
    fn test_shape_set_add_start_element() {
        let mut ss = TopOpeBRepBuildShapeSet::new(1);
        ss.add_start_element(5);
        ss.add_start_element(15);
        assert_eq!(ss.start_elements().len(), 2);
    }

    #[test]
    fn test_shape_set_iteration() {
        let mut ss = TopOpeBRepBuildShapeSet::new(1);
        ss.add_shape(100);
        ss.add_shape(200);

        ss.init_shapes();
        assert!(ss.more_shapes());
        assert_eq!(ss.shape(), Some(100));

        ss.next_shape();
        assert!(ss.more_shapes());
        assert_eq!(ss.shape(), Some(200));

        ss.next_shape();
        assert!(!ss.more_shapes());
    }

    #[test]
    fn test_shape_set_check() {
        let mut ss = TopOpeBRepBuildShapeSet::new(1);
        assert!(ss.check_shape());
        ss.set_check_shape(false);
        assert!(!ss.check_shape());
    }
}
