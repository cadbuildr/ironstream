// FILE: top_ope_b_rep_build_builder1.rs
// occt: TopOpeBRepBuild_Builder1

#[derive(Debug, Clone)]
pub struct TopOpeBRepBuildBuilder1 {
    shapes: Vec<i32>,
    result: Vec<i32>,
}

impl TopOpeBRepBuildBuilder1 {
    pub fn new() -> Self {
        TopOpeBRepBuildBuilder1 {
            shapes: Vec::new(),
            result: Vec::new(),
        }
    }

    pub fn add_shape(&mut self, shape_id: i32) {
        self.shapes.push(shape_id);
    }

    pub fn build(&mut self) {
        self.result = self.shapes.clone();
    }

    pub fn result(&self) -> &[i32] {
        &self.result
    }

    pub fn num_result_shapes(&self) -> usize {
        self.result.len()
    }
}

impl Default for TopOpeBRepBuildBuilder1 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder1_new() {
        let b = TopOpeBRepBuildBuilder1::new();
        assert_eq!(b.num_result_shapes(), 0);
    }

    #[test]
    fn test_builder1_add_shape() {
        let mut b = TopOpeBRepBuildBuilder1::new();
        b.add_shape(1);
        b.add_shape(2);
        b.build();
        assert_eq!(b.num_result_shapes(), 2);
    }

    #[test]
    fn test_builder1_result() {
        let mut b = TopOpeBRepBuildBuilder1::new();
        b.add_shape(10);
        b.build();
        assert_eq!(b.result().len(), 1);
    }
}
