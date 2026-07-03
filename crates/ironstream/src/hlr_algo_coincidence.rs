// FILE: hlr_algo_coincidence.rs
// occt: HLRAlgo_Coincidence

#[derive(Clone, Copy)]
pub enum TopAbsState {
    In = 0,
    Out = 1,
    On = 2,
    Unknown = 3,
}

/// Information on the "hiding" edge at an intersection point.
/// Stores 2D data (parameter, tangent/curvature) and 3D data (state before/after).
#[derive(Clone, Copy)]
pub struct HlrAlgoCoincidence {
    fe: i32,              // Edge index (2D)
    param: f64,           // Parameter on edge (2D)
    st_bef: TopAbsState,  // State before intersection (3D)
    st_aft: TopAbsState,  // State after intersection (3D)
}

impl HlrAlgoCoincidence {
    /// Create a default coincidence.
    pub fn new() -> Self {
        HlrAlgoCoincidence {
            fe: 0,
            param: 0.0,
            st_bef: TopAbsState::In,
            st_aft: TopAbsState::In,
        }
    }

    /// Set 2D data (edge index and parameter).
    pub fn set_2d(&mut self, fe: i32, param: f64) {
        self.fe = fe;
        self.param = param;
    }

    /// Set 3D state data (state before and after intersection).
    pub fn set_state_3d(&mut self, st_bef: TopAbsState, st_aft: TopAbsState) {
        self.st_bef = st_bef;
        self.st_aft = st_aft;
    }

    /// Get 2D data.
    pub fn value_2d(&self) -> (i32, f64) {
        (self.fe, self.param)
    }

    /// Get 3D state data.
    pub fn state_3d(&self) -> (TopAbsState, TopAbsState) {
        (self.st_bef, self.st_aft)
    }

    /// Get edge index.
    pub fn fe(&self) -> i32 {
        self.fe
    }

    /// Get parameter.
    pub fn param(&self) -> f64 {
        self.param
    }

    /// Get state before.
    pub fn st_bef(&self) -> TopAbsState {
        self.st_bef
    }

    /// Get state after.
    pub fn st_aft(&self) -> TopAbsState {
        self.st_aft
    }
}

impl Default for HlrAlgoCoincidence {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let c = HlrAlgoCoincidence::new();
        assert_eq!(c.fe(), 0);
        assert_eq!(c.param(), 0.0);
        matches!(c.st_bef(), TopAbsState::In);
        matches!(c.st_aft(), TopAbsState::In);
    }

    #[test]
    fn test_set_2d() {
        let mut c = HlrAlgoCoincidence::new();
        c.set_2d(5, 0.75);
        let (fe, param) = c.value_2d();
        assert_eq!(fe, 5);
        assert_eq!(param, 0.75);
    }

    #[test]
    fn test_set_state_3d() {
        let mut c = HlrAlgoCoincidence::new();
        c.set_state_3d(TopAbsState::Out, TopAbsState::In);
        let (bef, aft) = c.state_3d();
        matches!(bef, TopAbsState::Out);
        matches!(aft, TopAbsState::In);
    }

    #[test]
    fn test_default() {
        let c = HlrAlgoCoincidence::default();
        assert_eq!(c.fe(), 0);
    }
}
