// FILE: hlrb_rep_the_interference_of_inter_c_surf.rs
// occt: HLRBRep_TheInterferenceOfInterCSurf

#[derive(Clone, Debug)]
pub struct HLRBRepTheInterferenceOfInterCSurf {
    param_on_curve: f64,
    param_on_surface_u: f64,
    param_on_surface_v: f64,
}

impl HLRBRepTheInterferenceOfInterCSurf {
    pub fn new() -> Self {
        HLRBRepTheInterferenceOfInterCSurf {
            param_on_curve: 0.0,
            param_on_surface_u: 0.0,
            param_on_surface_v: 0.0,
        }
    }

    pub fn param_on_curve(&self) -> f64 {
        self.param_on_curve
    }

    pub fn param_on_surface_u(&self) -> f64 {
        self.param_on_surface_u
    }

    pub fn param_on_surface_v(&self) -> f64 {
        self.param_on_surface_v
    }
}

impl Default for HLRBRepTheInterferenceOfInterCSurf {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let i = HLRBRepTheInterferenceOfInterCSurf::new();
        assert_eq!(i.param_on_curve(), 0.0);
    }
}
