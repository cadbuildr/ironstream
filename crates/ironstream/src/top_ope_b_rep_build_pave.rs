// FILE: top_ope_b_rep_build_pave.rs
// occt: TopOpeBRepBuild_Pave

#[derive(Debug, Clone)]
pub struct TopOpeBRepBuildPave {
    vertex_id: i32,
    parameter: f64,
    is_bound: bool,
    has_same_domain: bool,
    same_domain_id: i32,
    interference_type: u32,
}

impl TopOpeBRepBuildPave {
    pub fn new(vertex_id: i32, parameter: f64, is_bound: bool) -> Self {
        TopOpeBRepBuildPave {
            vertex_id,
            parameter,
            is_bound,
            has_same_domain: false,
            same_domain_id: 0,
            interference_type: 0,
        }
    }

    pub fn vertex_id(&self) -> i32 {
        self.vertex_id
    }

    pub fn parameter(&self) -> f64 {
        self.parameter
    }

    pub fn set_parameter(&mut self, param: f64) {
        self.parameter = param;
    }

    pub fn is_bound(&self) -> bool {
        self.is_bound
    }

    pub fn has_same_domain(&self) -> bool {
        self.has_same_domain
    }

    pub fn set_has_same_domain(&mut self, has: bool) {
        self.has_same_domain = has;
    }

    pub fn same_domain_id(&self) -> i32 {
        self.same_domain_id
    }

    pub fn set_same_domain(&mut self, sd_id: i32) {
        self.same_domain_id = sd_id;
        self.has_same_domain = true;
    }

    pub fn interference_type(&self) -> u32 {
        self.interference_type
    }

    pub fn set_interference_type(&mut self, itype: u32) {
        self.interference_type = itype;
    }

    pub fn is_shape(&self) -> bool {
        true
    }

    pub fn dump(&self) {
        println!("Pave: vertex_id={}, param={}, bound={}", self.vertex_id, self.parameter, self.is_bound);
    }
}

impl Default for TopOpeBRepBuildPave {
    fn default() -> Self {
        Self::new(0, 0.0, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pave_new() {
        let pave = TopOpeBRepBuildPave::new(5, 3.5, true);
        assert_eq!(pave.vertex_id(), 5);
        assert_eq!(pave.parameter(), 3.5);
        assert!(pave.is_bound());
    }

    #[test]
    fn test_pave_set_parameter() {
        let mut pave = TopOpeBRepBuildPave::new(1, 1.0, false);
        pave.set_parameter(2.5);
        assert_eq!(pave.parameter(), 2.5);
    }

    #[test]
    fn test_pave_same_domain() {
        let mut pave = TopOpeBRepBuildPave::new(1, 0.5, false);
        assert!(!pave.has_same_domain());
        pave.set_same_domain(10);
        assert!(pave.has_same_domain());
        assert_eq!(pave.same_domain_id(), 10);
    }

    #[test]
    fn test_pave_interference_type() {
        let mut pave = TopOpeBRepBuildPave::new(1, 0.5, false);
        pave.set_interference_type(2);
        assert_eq!(pave.interference_type(), 2);
    }

    #[test]
    fn test_pave_dump() {
        let pave = TopOpeBRepBuildPave::new(1, 0.5, true);
        pave.dump();
    }
}
