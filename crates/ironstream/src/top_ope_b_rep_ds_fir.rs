// FILE: top_ope_b_rep_ds_fir.rs
// occt: TopOpeBRepDS_FIR

/// Face Interference Record
/// Stores face interference information for analysis
#[derive(Debug, Clone)]
pub struct FIR {
    /// Face index
    face_idx: usize,
    /// Interference index
    interference_idx: usize,
    /// U parameter
    param_u: f64,
    /// V parameter
    param_v: f64,
}

impl FIR {
    /// Create new FIR
    pub fn new(face_idx: usize, interference_idx: usize, param_u: f64, param_v: f64) -> Self {
        FIR {
            face_idx,
            interference_idx,
            param_u,
            param_v,
        }
    }

    /// Get face index
    pub fn face_idx(&self) -> usize {
        self.face_idx
    }

    /// Get interference index
    pub fn interference_idx(&self) -> usize {
        self.interference_idx
    }

    /// Get U parameter
    pub fn param_u(&self) -> f64 {
        self.param_u
    }

    /// Set U parameter
    pub fn set_param_u(&mut self, u: f64) {
        self.param_u = u;
    }

    /// Get V parameter
    pub fn param_v(&self) -> f64 {
        self.param_v
    }

    /// Set V parameter
    pub fn set_param_v(&mut self, v: f64) {
        self.param_v = v;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fir_new() {
        let fir = FIR::new(5, 10, 1.0, 2.0);
        assert_eq!(fir.face_idx(), 5);
        assert_eq!(fir.interference_idx(), 10);
        assert_eq!(fir.param_u(), 1.0);
        assert_eq!(fir.param_v(), 2.0);
    }

    #[test]
    fn test_fir_set_params() {
        let mut fir = FIR::new(1, 2, 0.5, 0.6);
        fir.set_param_u(1.5);
        fir.set_param_v(1.6);
        assert_eq!(fir.param_u(), 1.5);
        assert_eq!(fir.param_v(), 1.6);
    }
}
