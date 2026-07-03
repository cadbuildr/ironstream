// FILE: hlrb_rep_the_proj_p_cur_of_c_inter.rs
// occt: HLRBRep_TheProjPCurOfCInter

#[derive(Clone, Debug)]
pub struct HLRBRepTheProjPCur {
    u_parameter: f64,
    v_parameter: f64,
}

impl HLRBRepTheProjPCur {
    pub fn new() -> Self {
        HLRBRepTheProjPCur {
            u_parameter: 0.0,
            v_parameter: 0.0,
        }
    }

    pub fn u_parameter(&self) -> f64 {
        self.u_parameter
    }

    pub fn v_parameter(&self) -> f64 {
        self.v_parameter
    }

    pub fn set_parameters(&mut self, u: f64, v: f64) {
        self.u_parameter = u;
        self.v_parameter = v;
    }
}

impl Default for HLRBRepTheProjPCur {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let proj = HLRBRepTheProjPCur::new();
        assert_eq!(proj.u_parameter(), 0.0);
        assert_eq!(proj.v_parameter(), 0.0);
    }

    #[test]
    fn test_set_parameters() {
        let mut proj = HLRBRepTheProjPCur::new();
        proj.set_parameters(1.5, 2.5);
        assert_eq!(proj.u_parameter(), 1.5);
        assert_eq!(proj.v_parameter(), 2.5);
    }
}
