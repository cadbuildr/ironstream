// FILE: top_ope_b_rep_build_solid_area_builder.rs
// occt: TopOpeBRepBuild_SolidAreaBuilder

#[derive(Debug, Clone)]
pub struct TopOpeBRepBuildSolidAreaBuilder {
    areas: Vec<Vec<i32>>,
    current_area_index: usize,
}

impl TopOpeBRepBuildSolidAreaBuilder {
    pub fn new() -> Self {
        TopOpeBRepBuildSolidAreaBuilder {
            areas: Vec::new(),
            current_area_index: 0,
        }
    }

    pub fn add_area(&mut self, shells: Vec<i32>) {
        self.areas.push(shells);
    }

    pub fn more_area(&self) -> bool {
        self.current_area_index < self.areas.len()
    }

    pub fn next_area(&mut self) {
        self.current_area_index += 1;
    }

    pub fn num_areas(&self) -> usize {
        self.areas.len()
    }
}

impl Default for TopOpeBRepBuildSolidAreaBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solid_area_builder_new() {
        let sab = TopOpeBRepBuildSolidAreaBuilder::new();
        assert_eq!(sab.num_areas(), 0);
    }

    #[test]
    fn test_solid_area_builder_add_area() {
        let mut sab = TopOpeBRepBuildSolidAreaBuilder::new();
        sab.add_area(vec![1, 2]);
        assert_eq!(sab.num_areas(), 1);
    }

    #[test]
    fn test_solid_area_builder_iteration() {
        let mut sab = TopOpeBRepBuildSolidAreaBuilder::new();
        sab.add_area(vec![1]);
        sab.add_area(vec![2]);

        assert!(sab.more_area());
        sab.next_area();
        assert!(sab.more_area());
        sab.next_area();
        assert!(!sab.more_area());
    }
}
