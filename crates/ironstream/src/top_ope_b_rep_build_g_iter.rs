// FILE: top_ope_b_rep_build_g_iter.rs
// occt: TopOpeBRepBuild_GIter

#[derive(Debug, Clone)]
pub struct TopOpeBRepBuildGIter {
    ii: i32,
    state1: u32,
    state2: u32,
}

impl TopOpeBRepBuildGIter {
    pub fn new() -> Self {
        TopOpeBRepBuildGIter {
            ii: 0,
            state1: 0,
            state2: 0,
        }
    }

    pub fn init(&mut self) {
        self.ii = 0;
        self.state1 = 0;
        self.state2 = 0;
    }

    pub fn more(&self) -> bool {
        self.ii < 9
    }

    pub fn next(&mut self) {
        self.ii += 1;
    }

    pub fn current(&self) -> (u32, u32) {
        (self.state1, self.state2)
    }

    pub fn set_current(&mut self, s1: u32, s2: u32) {
        self.state1 = s1;
        self.state2 = s2;
    }

    pub fn dump(&self) {
        println!("GIter: ii={}, state1={}, state2={}", self.ii, self.state1, self.state2);
    }
}

impl Default for TopOpeBRepBuildGIter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_g_iter_new() {
        let gi = TopOpeBRepBuildGIter::new();
        assert_eq!(gi.ii, 0);
    }

    #[test]
    fn test_g_iter_init() {
        let mut gi = TopOpeBRepBuildGIter::new();
        gi.set_current(1, 2);
        gi.init();
        assert_eq!(gi.ii, 0);
        assert_eq!(gi.current(), (0, 0));
    }

    #[test]
    fn test_g_iter_iteration() {
        let mut gi = TopOpeBRepBuildGIter::new();
        for _ in 0..9 {
            assert!(gi.more());
            gi.next();
        }
        assert!(!gi.more());
    }

    #[test]
    fn test_g_iter_current() {
        let mut gi = TopOpeBRepBuildGIter::new();
        gi.set_current(3, 4);
        assert_eq!(gi.current(), (3, 4));
    }

    #[test]
    fn test_g_iter_dump() {
        let mut gi = TopOpeBRepBuildGIter::new();
        gi.set_current(1, 2);
        gi.dump();
    }
}
