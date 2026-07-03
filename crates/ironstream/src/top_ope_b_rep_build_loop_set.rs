// FILE: top_ope_b_rep_build_loop_set.rs
// occt: TopOpeBRepBuild_LoopSet

#[derive(Debug, Clone)]
pub struct TopOpeBRepBuildLoopSet {
    loops: Vec<i32>,
    current_index: usize,
}

impl TopOpeBRepBuildLoopSet {
    pub fn new() -> Self {
        TopOpeBRepBuildLoopSet {
            loops: Vec::new(),
            current_index: 0,
        }
    }

    pub fn add_loop(&mut self, loop_id: i32) {
        self.loops.push(loop_id);
    }

    pub fn init_loop(&mut self) {
        self.current_index = 0;
    }

    pub fn more_loop(&self) -> bool {
        self.current_index < self.loops.len()
    }

    pub fn next_loop(&mut self) {
        self.current_index += 1;
    }

    pub fn loop_id(&self) -> Option<i32> {
        if self.current_index < self.loops.len() {
            Some(self.loops[self.current_index])
        } else {
            None
        }
    }

    pub fn num_loops(&self) -> usize {
        self.loops.len()
    }
}

impl Default for TopOpeBRepBuildLoopSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loop_set_new() {
        let ls = TopOpeBRepBuildLoopSet::new();
        assert_eq!(ls.num_loops(), 0);
    }

    #[test]
    fn test_loop_set_add_loop() {
        let mut ls = TopOpeBRepBuildLoopSet::new();
        ls.add_loop(1);
        ls.add_loop(2);
        ls.add_loop(3);
        assert_eq!(ls.num_loops(), 3);
    }

    #[test]
    fn test_loop_set_iteration() {
        let mut ls = TopOpeBRepBuildLoopSet::new();
        ls.add_loop(10);
        ls.add_loop(20);

        ls.init_loop();
        assert!(ls.more_loop());
        assert_eq!(ls.loop_id(), Some(10));

        ls.next_loop();
        assert!(ls.more_loop());
        assert_eq!(ls.loop_id(), Some(20));

        ls.next_loop();
        assert!(!ls.more_loop());
    }

    #[test]
    fn test_loop_set_default() {
        let ls = TopOpeBRepBuildLoopSet::default();
        assert_eq!(ls.num_loops(), 0);
    }
}
