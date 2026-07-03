// FILE: ch_fi_ds_regul.rs
// occt: ChFiDS_Regul

/// Storage of a curve and its 2 faces or surfaces of support.
#[derive(Debug, Clone)]
pub struct ChFiDsRegul {
    /// Index of the curve
    curve_index: i32,
    /// Index of first support (face or surface)
    s1_index: i32,
    /// Index of second support (face or surface)
    s2_index: i32,
    /// Whether s1 is a face (true) or surface (false)
    s1_is_face: bool,
    /// Whether s2 is a face (true) or surface (false)
    s2_is_face: bool,
}

impl ChFiDsRegul {
    /// Create a new Regul with default values
    pub fn new() -> Self {
        Self {
            curve_index: -1,
            s1_index: -1,
            s2_index: -1,
            s1_is_face: true,
            s2_is_face: true,
        }
    }

    /// Set the curve index
    pub fn set_curve(&mut self, ic: i32) {
        self.curve_index = ic;
    }

    /// Set the first support (face or surface)
    pub fn set_s1(&mut self, is1: i32, is_face: bool) {
        self.s1_index = is1;
        self.s1_is_face = is_face;
    }

    /// Set the second support (face or surface)
    pub fn set_s2(&mut self, is2: i32, is_face: bool) {
        self.s2_index = is2;
        self.s2_is_face = is_face;
    }

    /// Check if first support is a face
    pub fn is_surface1(&self) -> bool {
        !self.s1_is_face
    }

    /// Check if second support is a face
    pub fn is_surface2(&self) -> bool {
        !self.s2_is_face
    }

    /// Get the curve index
    pub fn curve(&self) -> i32 {
        self.curve_index
    }

    /// Get the first support index
    pub fn s1(&self) -> i32 {
        self.s1_index
    }

    /// Get the second support index
    pub fn s2(&self) -> i32 {
        self.s2_index
    }
}

impl Default for ChFiDsRegul {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let regul = ChFiDsRegul::new();
        assert_eq!(regul.curve(), -1);
        assert_eq!(regul.s1(), -1);
        assert_eq!(regul.s2(), -1);
        assert!(!regul.is_surface1());
        assert!(!regul.is_surface2());
    }

    #[test]
    fn test_set_curve() {
        let mut regul = ChFiDsRegul::new();
        regul.set_curve(42);
        assert_eq!(regul.curve(), 42);
    }

    #[test]
    fn test_set_s1() {
        let mut regul = ChFiDsRegul::new();
        regul.set_s1(5, true);
        assert_eq!(regul.s1(), 5);
        assert!(!regul.is_surface1());

        regul.set_s1(6, false);
        assert_eq!(regul.s1(), 6);
        assert!(regul.is_surface1());
    }

    #[test]
    fn test_set_s2() {
        let mut regul = ChFiDsRegul::new();
        regul.set_s2(10, true);
        assert_eq!(regul.s2(), 10);
        assert!(!regul.is_surface2());

        regul.set_s2(11, false);
        assert_eq!(regul.s2(), 11);
        assert!(regul.is_surface2());
    }

    #[test]
    fn test_default() {
        let regul = ChFiDsRegul::default();
        assert_eq!(regul.curve(), -1);
    }
}
