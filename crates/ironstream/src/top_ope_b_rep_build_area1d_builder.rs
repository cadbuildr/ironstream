// FILE: top_ope_b_rep_build_area1d_builder.rs
// occt: TopOpeBRepBuild_Area1dBuilder

#[derive(Debug, Clone)]
pub struct TopOpeBRepBuildArea1dBuilder {
    loops: Vec<i32>,
    current_index: usize,
    force_class: bool,
}

impl TopOpeBRepBuildArea1dBuilder {
    pub fn new() -> Self {
        TopOpeBRepBuildArea1dBuilder {
            loops: Vec::new(),
            current_index: 0,
            force_class: false,
        }
    }

    pub fn with_force_class(force_class: bool) -> Self {
        TopOpeBRepBuildArea1dBuilder {
            loops: Vec::new(),
            current_index: 0,
            force_class,
        }
    }

    pub fn add_loop(&mut self, loop_id: i32) {
        self.loops.push(loop_id);
    }

    pub fn init_area_builder(&mut self, force_class: bool) {
        self.force_class = force_class;
        self.current_index = 0;
    }

    pub fn more_loop(&self) -> bool {
        self.current_index < self.loops.len()
    }

    pub fn next_loop(&mut self) {
        self.current_index += 1;
    }

    pub fn current_loop_id(&self) -> Option<i32> {
        if self.current_index < self.loops.len() {
            Some(self.loops[self.current_index])
        } else {
            None
        }
    }

    pub fn num_loops(&self) -> usize {
        self.loops.len()
    }

    pub fn force_class(&self) -> bool {
        self.force_class
    }
}

impl Default for TopOpeBRepBuildArea1dBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area1d_builder_new() {
        let a1b = TopOpeBRepBuildArea1dBuilder::new();
        assert_eq!(a1b.num_loops(), 0);
        assert!(!a1b.force_class());
    }

    #[test]
    fn test_area1d_builder_with_force_class() {
        let a1b = TopOpeBRepBuildArea1dBuilder::with_force_class(true);
        assert!(a1b.force_class());
    }

    #[test]
    fn test_area1d_builder_add_loop() {
        let mut a1b = TopOpeBRepBuildArea1dBuilder::new();
        a1b.add_loop(1);
        a1b.add_loop(2);
        a1b.add_loop(3);
        assert_eq!(a1b.num_loops(), 3);
    }

    #[test]
    fn test_area1d_builder_iteration() {
        let mut a1b = TopOpeBRepBuildArea1dBuilder::new();
        a1b.add_loop(10);
        a1b.add_loop(20);

        assert!(a1b.more_loop());
        assert_eq!(a1b.current_loop_id(), Some(10));

        a1b.next_loop();
        assert!(a1b.more_loop());
        assert_eq!(a1b.current_loop_id(), Some(20));

        a1b.next_loop();
        assert!(!a1b.more_loop());
    }

    #[test]
    fn test_area1d_builder_init() {
        let mut a1b = TopOpeBRepBuildArea1dBuilder::new();
        a1b.init_area_builder(true);
        assert!(a1b.force_class());
    }
}
