// FILE: top_ope_b_rep_build_g_topo.rs
// occt: TopOpeBRepBuild_GTopo

#[derive(Debug, Clone)]
pub struct TopOpeBRepBuildGTopo {
    cases: [[bool; 3]; 3],
    type1: u32,
    type2: u32,
    config1: u32,
    config2: u32,
    reverse_force: bool,
    reverse_value: bool,
}

impl TopOpeBRepBuildGTopo {
    pub fn new() -> Self {
        TopOpeBRepBuildGTopo {
            cases: [[false; 3]; 3],
            type1: 0,
            type2: 0,
            config1: 0,
            config2: 0,
            reverse_force: false,
            reverse_value: false,
        }
    }

    pub fn with_cases(
        ii: bool, in_: bool, io: bool,
        ni: bool, nn: bool, no_: bool,
        oi: bool, on: bool, oo: bool,
        type1: u32, type2: u32,
        config1: u32, config2: u32,
    ) -> Self {
        let mut gt = Self::new();
        gt.cases[0][0] = ii;
        gt.cases[0][1] = in_;
        gt.cases[0][2] = io;
        gt.cases[1][0] = ni;
        gt.cases[1][1] = nn;
        gt.cases[1][2] = no_;
        gt.cases[2][0] = oi;
        gt.cases[2][1] = on;
        gt.cases[2][2] = oo;
        gt.type1 = type1;
        gt.type2 = type2;
        gt.config1 = config1;
        gt.config2 = config2;
        gt
    }

    pub fn reset(&mut self) {
        self.cases = [[false; 3]; 3];
        self.type1 = 0;
        self.type2 = 0;
    }

    pub fn type_values(&self) -> (u32, u32) {
        (self.type1, self.type2)
    }

    pub fn change_type(&mut self, t1: u32, t2: u32) {
        self.type1 = t1;
        self.type2 = t2;
    }

    pub fn config1(&self) -> u32 {
        self.config1
    }

    pub fn config2(&self) -> u32 {
        self.config2
    }

    pub fn change_config(&mut self, c1: u32, c2: u32) {
        self.config1 = c1;
        self.config2 = c2;
    }

    pub fn value(&self, i1: usize, i2: usize) -> bool {
        if i1 < 3 && i2 < 3 {
            self.cases[i1][i2]
        } else {
            false
        }
    }

    pub fn change_value(&mut self, i1: usize, i2: usize, b: bool) {
        if i1 < 3 && i2 < 3 {
            self.cases[i1][i2] = b;
        }
    }

    pub fn set_reverse(&mut self, rev: bool) {
        self.reverse_force = rev;
    }

    pub fn reverse(&self) -> bool {
        self.reverse_value
    }

    pub fn copy_permuted(&self) -> Self {
        let mut cp = self.clone();
        cp.type1 = self.type2;
        cp.type2 = self.type1;
        cp
    }

    pub fn dump(&self) {
        println!("GTopo: type1={}, type2={}, config1={}, config2={}",
            self.type1, self.type2, self.config1, self.config2);
    }
}

impl Default for TopOpeBRepBuildGTopo {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_g_topo_new() {
        let gt = TopOpeBRepBuildGTopo::new();
        assert_eq!(gt.type1, 0);
        assert_eq!(gt.type2, 0);
    }

    #[test]
    fn test_g_topo_with_cases() {
        let gt = TopOpeBRepBuildGTopo::with_cases(
            true, false, true,
            false, true, false,
            true, false, true,
            1, 2, 0, 1
        );
        assert_eq!(gt.type1, 1);
        assert_eq!(gt.type2, 2);
        assert!(gt.value(0, 0));
        assert!(!gt.value(0, 1));
    }

    #[test]
    fn test_g_topo_reset() {
        let mut gt = TopOpeBRepBuildGTopo::with_cases(
            true, true, true,
            true, true, true,
            true, true, true,
            1, 2, 0, 1
        );
        gt.reset();
        assert_eq!(gt.type1, 0);
        assert!(!gt.value(0, 0));
    }

    #[test]
    fn test_g_topo_config() {
        let mut gt = TopOpeBRepBuildGTopo::new();
        gt.change_config(3, 4);
        assert_eq!(gt.config1(), 3);
        assert_eq!(gt.config2(), 4);
    }

    #[test]
    fn test_g_topo_reverse() {
        let mut gt = TopOpeBRepBuildGTopo::new();
        gt.set_reverse(true);
        assert!(gt.reverse_force);
    }

    #[test]
    fn test_g_topo_copy_permuted() {
        let gt = TopOpeBRepBuildGTopo::with_cases(
            false, false, false,
            false, false, false,
            false, false, false,
            5, 7, 0, 1
        );
        let cp = gt.copy_permuted();
        assert_eq!(cp.type1, 7);
        assert_eq!(cp.type2, 5);
    }

    #[test]
    fn test_g_topo_dump() {
        let gt = TopOpeBRepBuildGTopo::new();
        gt.dump();
    }
}
