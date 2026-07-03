// FILE: top_ope_b_rep_build_pave_set.rs
// occt: TopOpeBRepBuild_PaveSet

#[derive(Debug, Clone)]
pub struct TopOpeBRepBuildPaveSet {
    edge_id: i32,
    paves: Vec<(i32, f64)>,
    current_index: usize,
    has_equal_parameters: bool,
    equal_parameters: f64,
    closed: bool,
}

impl TopOpeBRepBuildPaveSet {
    pub fn new(edge_id: i32) -> Self {
        TopOpeBRepBuildPaveSet {
            edge_id,
            paves: Vec::new(),
            current_index: 0,
            has_equal_parameters: false,
            equal_parameters: 0.0,
            closed: false,
        }
    }

    pub fn append(&mut self, vertex_id: i32, parameter: f64) {
        self.paves.push((vertex_id, parameter));
    }

    pub fn edge_id(&self) -> i32 {
        self.edge_id
    }

    pub fn init_loop(&mut self) {
        self.current_index = 0;
    }

    pub fn more_loop(&self) -> bool {
        self.current_index < self.paves.len()
    }

    pub fn next_loop(&mut self) {
        self.current_index += 1;
    }

    pub fn current_pave(&self) -> Option<(i32, f64)> {
        if self.current_index < self.paves.len() {
            Some(self.paves[self.current_index])
        } else {
            None
        }
    }

    pub fn has_equal_parameters(&self) -> bool {
        self.has_equal_parameters
    }

    pub fn equal_parameters(&self) -> f64 {
        self.equal_parameters
    }

    pub fn closed_vertices(&self) -> bool {
        self.closed
    }

    pub fn num_paves(&self) -> usize {
        self.paves.len()
    }
}

impl Default for TopOpeBRepBuildPaveSet {
    fn default() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pave_set_new() {
        let ps = TopOpeBRepBuildPaveSet::new(5);
        assert_eq!(ps.edge_id(), 5);
        assert_eq!(ps.num_paves(), 0);
    }

    #[test]
    fn test_pave_set_append() {
        let mut ps = TopOpeBRepBuildPaveSet::new(1);
        ps.append(10, 0.5);
        ps.append(20, 0.8);
        assert_eq!(ps.num_paves(), 2);
    }

    #[test]
    fn test_pave_set_iteration() {
        let mut ps = TopOpeBRepBuildPaveSet::new(1);
        ps.append(10, 0.5);
        ps.append(20, 0.8);

        ps.init_loop();
        assert!(ps.more_loop());
        assert_eq!(ps.current_pave(), Some((10, 0.5)));

        ps.next_loop();
        assert!(ps.more_loop());
        assert_eq!(ps.current_pave(), Some((20, 0.8)));

        ps.next_loop();
        assert!(!ps.more_loop());
    }

    #[test]
    fn test_pave_set_equal_parameters() {
        let ps = TopOpeBRepBuildPaveSet::new(1);
        assert!(!ps.has_equal_parameters());
        assert_eq!(ps.equal_parameters(), 0.0);
    }

    #[test]
    fn test_pave_set_closed() {
        let ps = TopOpeBRepBuildPaveSet::new(1);
        assert!(!ps.closed_vertices());
    }
}
