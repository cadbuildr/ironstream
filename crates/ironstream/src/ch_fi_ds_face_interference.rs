// FILE: ch_fi_ds_face_interference.rs
// occt: ChFiDS_FaceInterference

/// Interference between a fillet surface and a face.
/// Stores a 2D curve on the face, a 2D curve on the fillet surface,
/// parameter bounds, and transition information.
#[derive(Debug, Clone)]
pub struct ChFiDsFaceInterference {
    /// Index of the line in the fillet stripe
    line_index: i32,
    /// First parameter on the curves
    first_param: f64,
    /// Last parameter on the curves
    last_param: f64,
}

impl ChFiDsFaceInterference {
    /// Create a new FaceInterference
    pub fn new() -> Self {
        Self {
            line_index: -1,
            first_param: 0.0,
            last_param: 0.0,
        }
    }

    /// Set line index
    pub fn set_line_index(&mut self, i: i32) {
        self.line_index = i;
    }

    /// Get line index
    pub fn line_index(&self) -> i32 {
        self.line_index
    }

    /// Set first parameter
    pub fn set_first_parameter(&mut self, u: f64) {
        self.first_param = u;
    }

    /// Set last parameter
    pub fn set_last_parameter(&mut self, u: f64) {
        self.last_param = u;
    }

    /// Get first parameter
    pub fn first_parameter(&self) -> f64 {
        self.first_param
    }

    /// Get last parameter
    pub fn last_parameter(&self) -> f64 {
        self.last_param
    }

    /// Set parameter (first if is_first is true, last otherwise)
    pub fn set_parameter(&mut self, u: f64, is_first: bool) {
        if is_first {
            self.first_param = u;
        } else {
            self.last_param = u;
        }
    }

    /// Get parameter (first if is_first is true, last otherwise)
    pub fn parameter(&self, is_first: bool) -> f64 {
        if is_first {
            self.first_param
        } else {
            self.last_param
        }
    }
}

impl Default for ChFiDsFaceInterference {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let fi = ChFiDsFaceInterference::new();
        assert_eq!(fi.line_index(), -1);
        assert_eq!(fi.first_parameter(), 0.0);
        assert_eq!(fi.last_parameter(), 0.0);
    }

    #[test]
    fn test_set_line_index() {
        let mut fi = ChFiDsFaceInterference::new();
        fi.set_line_index(5);
        assert_eq!(fi.line_index(), 5);
    }

    #[test]
    fn test_set_parameters() {
        let mut fi = ChFiDsFaceInterference::new();
        fi.set_first_parameter(1.5);
        fi.set_last_parameter(3.5);
        assert_eq!(fi.first_parameter(), 1.5);
        assert_eq!(fi.last_parameter(), 3.5);
    }

    #[test]
    fn test_set_parameter_method() {
        let mut fi = ChFiDsFaceInterference::new();
        fi.set_parameter(2.0, true);
        fi.set_parameter(4.0, false);
        assert_eq!(fi.parameter(true), 2.0);
        assert_eq!(fi.parameter(false), 4.0);
    }

    #[test]
    fn test_default() {
        let fi = ChFiDsFaceInterference::default();
        assert_eq!(fi.line_index(), -1);
    }
}
