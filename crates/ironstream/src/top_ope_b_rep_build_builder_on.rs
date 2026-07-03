// FILE: top_ope_b_rep_build_builder_on.rs
// occt: TopOpeBRepBuild_BuilderON

#[derive(Debug, Clone)]
pub struct TopOpeBRepBuildBuilderON {
    shape1_id: i32,
    shape2_id: i32,
    result_id: i32,
}

impl TopOpeBRepBuildBuilderON {
    pub fn new() -> Self {
        TopOpeBRepBuildBuilderON {
            shape1_id: 0,
            shape2_id: 0,
            result_id: 0,
        }
    }

    pub fn with_shapes(shape1_id: i32, shape2_id: i32) -> Self {
        TopOpeBRepBuildBuilderON {
            shape1_id,
            shape2_id,
            result_id: 0,
        }
    }

    pub fn build(&mut self) {
        self.result_id = self.shape1_id + self.shape2_id;
    }

    pub fn result(&self) -> i32 {
        self.result_id
    }

    pub fn shape1_id(&self) -> i32 {
        self.shape1_id
    }

    pub fn shape2_id(&self) -> i32 {
        self.shape2_id
    }
}

impl Default for TopOpeBRepBuildBuilderON {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_on_new() {
        let b = TopOpeBRepBuildBuilderON::new();
        assert_eq!(b.result(), 0);
    }

    #[test]
    fn test_builder_on_with_shapes() {
        let b = TopOpeBRepBuildBuilderON::with_shapes(5, 10);
        assert_eq!(b.shape1_id(), 5);
        assert_eq!(b.shape2_id(), 10);
    }

    #[test]
    fn test_builder_on_build() {
        let mut b = TopOpeBRepBuildBuilderON::with_shapes(3, 7);
        b.build();
        assert_eq!(b.result(), 10);
    }
}
