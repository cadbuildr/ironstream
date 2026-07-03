// FILE: top_ope_b_rep_build_face_area_builder.rs
// occt: TopOpeBRepBuild_FaceAreaBuilder

#[derive(Debug, Clone)]
pub struct TopOpeBRepBuildFaceAreaBuilder {
    areas: Vec<Vec<i32>>,
    current_area_index: usize,
    current_loop_index: usize,
}

impl TopOpeBRepBuildFaceAreaBuilder {
    pub fn new() -> Self {
        TopOpeBRepBuildFaceAreaBuilder {
            areas: Vec::new(),
            current_area_index: 0,
            current_loop_index: 0,
        }
    }

    pub fn add_area(&mut self, loops: Vec<i32>) {
        self.areas.push(loops);
    }

    pub fn init_face_area_builder(&mut self, _force_class: bool) {
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

impl Default for TopOpeBRepBuildFaceAreaBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_face_area_builder_new() {
        let fab = TopOpeBRepBuildFaceAreaBuilder::new();
        assert_eq!(fab.num_areas(), 0);
    }

    #[test]
    fn test_face_area_builder_add_area() {
        let mut fab = TopOpeBRepBuildFaceAreaBuilder::new();
        fab.add_area(vec![1, 2, 3]);
        fab.add_area(vec![4, 5]);
        assert_eq!(fab.num_areas(), 2);
    }

    #[test]
    fn test_face_area_builder_iteration() {
        let mut fab = TopOpeBRepBuildFaceAreaBuilder::new();
        fab.add_area(vec![1, 2]);
        fab.add_area(vec![3, 4, 5]);

        fab.init_face_area_builder(false);
        assert!(fab.more_area());

        fab.init_loop();
        assert!(fab.more_loop());

        fab.next_loop();
        assert!(fab.more_loop());

        fab.next_loop();
        assert!(!fab.more_loop());

        fab.next_area();
        assert!(fab.more_area());
    }

    #[test]
    fn test_face_area_builder_default() {
        let fab = TopOpeBRepBuildFaceAreaBuilder::default();
        assert_eq!(fab.num_areas(), 0);
    }
}
