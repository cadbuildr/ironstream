// FILE: top_ope_b_rep_build_area3d_builder.rs
// occt: TopOpeBRepBuild_Area3dBuilder

#[derive(Debug, Clone)]
pub struct TopOpeBRepBuildArea3dBuilder {
    areas: Vec<Vec<i32>>,
    current_area_index: usize,
    current_loop_index: usize,
}

impl TopOpeBRepBuildArea3dBuilder {
    pub fn new() -> Self {
        TopOpeBRepBuildArea3dBuilder {
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

impl Default for TopOpeBRepBuildArea3dBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area3d_builder_new() {
        let a3b = TopOpeBRepBuildArea3dBuilder::new();
        assert_eq!(a3b.num_areas(), 0);
    }

    #[test]
    fn test_area3d_builder_add_area() {
        let mut a3b = TopOpeBRepBuildArea3dBuilder::new();
        a3b.add_area(vec![1, 2, 3]);
        assert_eq!(a3b.num_areas(), 1);
    }

    #[test]
    fn test_area3d_builder_iteration() {
        let mut a3b = TopOpeBRepBuildArea3dBuilder::new();
        a3b.add_area(vec![10, 20]);

        a3b.init_area_builder(false);
        assert!(a3b.more_area());

        a3b.init_loop();
        assert!(a3b.more_loop());

        a3b.next_loop();
        assert!(a3b.more_loop());

        a3b.next_loop();
        assert!(!a3b.more_loop());
    }

    #[test]
    fn test_area3d_builder_default() {
        let a3b = TopOpeBRepBuildArea3dBuilder::default();
        assert_eq!(a3b.num_areas(), 0);
    }
}
