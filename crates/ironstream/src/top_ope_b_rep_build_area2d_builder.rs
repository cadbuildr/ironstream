// FILE: top_ope_b_rep_build_area2d_builder.rs
// occt: TopOpeBRepBuild_Area2dBuilder

#[derive(Debug, Clone)]
pub struct TopOpeBRepBuildArea2dBuilder {
    areas: Vec<Vec<i32>>,
    current_area_index: usize,
    current_loop_index: usize,
}

impl TopOpeBRepBuildArea2dBuilder {
    pub fn new() -> Self {
        TopOpeBRepBuildArea2dBuilder {
            areas: Vec::new(),
            current_area_index: 0,
            current_loop_index: 0,
        }
    }

    pub fn add_area(&mut self, loops: Vec<i32>) {
        self.areas.push(loops);
    }

    pub fn init_area_builder(&mut self, _force_class: bool) {
        self.current_area_index = 0;
        self.current_loop_index = 0;
    }

    pub fn more_area(&self) -> bool {
        self.current_area_index < self.areas.len()
    }

    pub fn next_area(&mut self) {
        self.current_area_index += 1;
        self.current_loop_index = 0;
    }

    pub fn init_loop(&mut self) {
        self.current_loop_index = 0;
    }

    pub fn more_loop(&self) -> bool {
        if self.current_area_index < self.areas.len() {
            self.current_loop_index < self.areas[self.current_area_index].len()
        } else {
            false
        }
    }

    pub fn next_loop(&mut self) {
        self.current_loop_index += 1;
    }

    pub fn num_areas(&self) -> usize {
        self.areas.len()
    }
}

impl Default for TopOpeBRepBuildArea2dBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area2d_builder_new() {
        let a2b = TopOpeBRepBuildArea2dBuilder::new();
        assert_eq!(a2b.num_areas(), 0);
    }

    #[test]
    fn test_area2d_builder_add_area() {
        let mut a2b = TopOpeBRepBuildArea2dBuilder::new();
        a2b.add_area(vec![1, 2]);
        a2b.add_area(vec![3, 4, 5]);
        assert_eq!(a2b.num_areas(), 2);
    }

    #[test]
    fn test_area2d_builder_iteration() {
        let mut a2b = TopOpeBRepBuildArea2dBuilder::new();
        a2b.add_area(vec![1, 2]);
        a2b.add_area(vec![3, 4]);

        a2b.init_area_builder(false);
        assert!(a2b.more_area());

        a2b.init_loop();
        assert!(a2b.more_loop());

        a2b.next_loop();
        assert!(a2b.more_loop());

        a2b.next_loop();
        assert!(!a2b.more_loop());

        a2b.next_area();
        assert!(a2b.more_area());
    }

    #[test]
    fn test_area2d_builder_default() {
        let a2b = TopOpeBRepBuildArea2dBuilder::default();
        assert_eq!(a2b.num_areas(), 0);
    }
}
