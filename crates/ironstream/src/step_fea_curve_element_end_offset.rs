// FILE: step_fea_curve_element_end_offset.rs
// occt: StepFEA_CurveElementEndOffset

/// Representation of STEP entity CurveElementEndOffset.
#[derive(Clone)]
pub struct CurveElementEndOffset {
    offset_x: f64,
    offset_y: f64,
    offset_z: f64,
}

impl CurveElementEndOffset {
    pub fn new() -> Self {
        Self {
            offset_x: 0.0,
            offset_y: 0.0,
            offset_z: 0.0,
        }
    }

    pub fn init(&mut self, x: f64, y: f64, z: f64) {
        self.offset_x = x;
        self.offset_y = y;
        self.offset_z = z;
    }

    pub fn offset_x(&self) -> f64 {
        self.offset_x
    }

    pub fn set_offset_x(&mut self, x: f64) {
        self.offset_x = x;
    }

    pub fn offset_y(&self) -> f64 {
        self.offset_y
    }

    pub fn set_offset_y(&mut self, y: f64) {
        self.offset_y = y;
    }

    pub fn offset_z(&self) -> f64 {
        self.offset_z
    }

    pub fn set_offset_z(&mut self, z: f64) {
        self.offset_z = z;
    }
}

impl Default for CurveElementEndOffset {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let offset = CurveElementEndOffset::new();
        assert_eq!(offset.offset_x(), 0.0);
        assert_eq!(offset.offset_y(), 0.0);
        assert_eq!(offset.offset_z(), 0.0);
    }

    #[test]
    fn test_init() {
        let mut offset = CurveElementEndOffset::new();
        offset.init(1.0, 2.0, 3.0);

        assert_eq!(offset.offset_x(), 1.0);
        assert_eq!(offset.offset_y(), 2.0);
        assert_eq!(offset.offset_z(), 3.0);
    }

    #[test]
    fn test_setters() {
        let mut offset = CurveElementEndOffset::new();
        offset.set_offset_x(0.5);
        offset.set_offset_y(1.5);
        offset.set_offset_z(2.5);

        assert_eq!(offset.offset_x(), 0.5);
        assert_eq!(offset.offset_y(), 1.5);
        assert_eq!(offset.offset_z(), 2.5);
    }
}
