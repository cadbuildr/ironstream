// FILE: top_ope_b_rep_build_loop.rs
// occt: TopOpeBRepBuild_Loop

#[derive(Debug, Clone)]
pub struct TopOpeBRepBuildLoop {
    is_shape: bool,
    shape_id: i32,
    block_lower: i32,
    block_upper: i32,
}

impl TopOpeBRepBuildLoop {
    pub fn new_shape(shape_id: i32) -> Self {
        TopOpeBRepBuildLoop {
            is_shape: true,
            shape_id,
            block_lower: 0,
            block_upper: 0,
        }
    }

    pub fn new_block(lower: i32, upper: i32) -> Self {
        TopOpeBRepBuildLoop {
            is_shape: false,
            shape_id: 0,
            block_lower: lower,
            block_upper: upper,
        }
    }

    pub fn is_shape(&self) -> bool {
        self.is_shape
    }

    pub fn shape_id(&self) -> i32 {
        self.shape_id
    }

    pub fn block_range(&self) -> (i32, i32) {
        (self.block_lower, self.block_upper)
    }

    pub fn dump(&self) {
        if self.is_shape {
            println!("Loop: Shape({})", self.shape_id);
        } else {
            println!("Loop: Block({}, {})", self.block_lower, self.block_upper);
        }
    }
}

impl Default for TopOpeBRepBuildLoop {
    fn default() -> Self {
        Self::new_shape(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_new_shape() {
        let loop_obj = TopOpeBRepBuildLoop::new_shape(5);
        assert!(loop_obj.is_shape());
        assert_eq!(loop_obj.shape_id(), 5);
    }

    #[test]
    fn test_loop_new_block() {
        let loop_obj = TopOpeBRepBuildLoop::new_block(1, 10);
        assert!(!loop_obj.is_shape());
        assert_eq!(loop_obj.block_range(), (1, 10));
    }

    #[test]
    fn test_loop_dump_shape() {
        let loop_obj = TopOpeBRepBuildLoop::new_shape(3);
        loop_obj.dump();
    }

    #[test]
    fn test_loop_dump_block() {
        let loop_obj = TopOpeBRepBuildLoop::new_block(2, 8);
        loop_obj.dump();
    }

    #[test]
    fn test_loop_default() {
        let loop_obj = TopOpeBRepBuildLoop::default();
        assert!(loop_obj.is_shape());
        assert_eq!(loop_obj.shape_id(), 0);
    }
}
