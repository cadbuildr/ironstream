// FILE: top_ope_b_rep_build_area_builder.rs
// occt: TopOpeBRepBuild_AreaBuilder

#[derive(Debug, Clone)]
pub struct TopOpeBRepBuildAreaBuilder {
    areas: Vec<Vec<i32>>,
    current_area_index: usize,
    current_loop_index: usize,
    unknown_raise: bool,
}

impl TopOpeBRepBuildAreaBuilder {
    pub fn new() -> Self {
        TopOpeBRepBuildAreaBuilder {
            areas: Vec::new(),
            current_area_index: 0,
            current_loop_index: 0,
            unknown_raise: false,
        }
    }

    pub fn add_area(&mut self, loops: Vec<i32>) {
        self.areas.push(loops);
    }

    pub fn init_area(&mut self) -> i32 {
        self.current_area_index = 0;
        self.areas.len() as i32
    }

    pub fn more_area(&self) -> bool {
        self.current_area_index < self.areas.len()
    }

    pub fn next_area(&mut self) {
        self.current_area_index += 1;
        self.current_loop_index = 0;
    }

    pub fn init_loop(&mut self) -> i32 {
        self.current_loop_index = 0;
        if self.current_area_index < self.areas.len() {
            self.areas[self.current_area_index].len() as i32
        } else {
            0
        }
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

    pub fn current_loop_id(&self) -> Option<i32> {
        if self.current_area_index < self.areas.len()
            && self.current_loop_index < self.areas[self.current_area_index].len()
        {
            Some(self.areas[self.current_area_index][self.current_loop_index])
        } else {
            None
        }
    }

    pub fn num_areas(&self) -> usize {
        self.areas.len()
    }

    pub fn set_unknown_raise(&mut self, raise: bool) {
        self.unknown_raise = raise;
    }
}

impl Default for TopOpeBRepBuildAreaBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_builder_new() {
        let ab = TopOpeBRepBuildAreaBuilder::new();
        assert_eq!(ab.num_areas(), 0);
    }

    #[test]
    fn test_area_builder_add_area() {
        let mut ab = TopOpeBRepBuildAreaBuilder::new();
        ab.add_area(vec![1, 2, 3]);
        ab.add_area(vec![4, 5]);
        assert_eq!(ab.num_areas(), 2);
    }

    #[test]
    fn test_area_builder_iteration() {
        let mut ab = TopOpeBRepBuildAreaBuilder::new();
        ab.add_area(vec![1, 2]);
        ab.add_area(vec![3, 4, 5]);

        ab.init_area();
        assert!(ab.more_area());

        ab.init_loop();
        assert!(ab.more_loop());
        assert_eq!(ab.current_loop_id(), Some(1));

        ab.next_loop();
        assert!(ab.more_loop());
        assert_eq!(ab.current_loop_id(), Some(2));

        ab.next_loop();
        assert!(!ab.more_loop());

        ab.next_area();
        assert!(ab.more_area());

        ab.init_loop();
        assert!(ab.more_loop());
        assert_eq!(ab.current_loop_id(), Some(3));
    }

    #[test]
    fn test_area_builder_unknown_raise() {
        let mut ab = TopOpeBRepBuildAreaBuilder::new();
        ab.set_unknown_raise(true);
        assert!(ab.unknown_raise);
    }
}
