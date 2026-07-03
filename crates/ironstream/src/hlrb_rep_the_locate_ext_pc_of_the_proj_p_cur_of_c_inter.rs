// FILE: hlrb_rep_the_locate_ext_pc_of_the_proj_p_cur_of_c_inter.rs
// occt: HLRBRep_TheLocateExtPCOfTheProjPCurOfCInter

#[derive(Clone, Debug)]
pub struct HLRBRepTheLocateExtPC {
    parameter: f64,
    found: bool,
}

impl HLRBRepTheLocateExtPC {
    pub fn new() -> Self {
        HLRBRepTheLocateExtPC {
            parameter: 0.0,
            found: false,
        }
    }

    pub fn parameter(&self) -> f64 {
        self.parameter
    }

    pub fn is_found(&self) -> bool {
        self.found
    }

    pub fn locate(&mut self, _param: f64) {
        self.found = true;
    }
}

impl Default for HLRBRepTheLocateExtPC {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let loc = HLRBRepTheLocateExtPC::new();
        assert!(!loc.is_found());
    }

    #[test]
    fn test_locate() {
        let mut loc = HLRBRepTheLocateExtPC::new();
        loc.locate(0.5);
        assert!(loc.is_found());
    }
}
