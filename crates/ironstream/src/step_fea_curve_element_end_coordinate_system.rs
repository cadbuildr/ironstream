// FILE: step_fea_curve_element_end_coordinate_system.rs
// occt: StepFEA_CurveElementEndCoordinateSystem

/// Representation of STEP entity CurveElementEndCoordinateSystem.
#[derive(Clone)]
pub struct CurveElementEndCoordinateSystem {
    axis_1: Option<String>,
    axis_2: Option<String>,
    axis_3: Option<String>,
}

impl CurveElementEndCoordinateSystem {
    pub fn new() -> Self {
        Self {
            axis_1: None,
            axis_2: None,
            axis_3: None,
        }
    }

    pub fn init(
        &mut self,
        axis_1: Option<String>,
        axis_2: Option<String>,
        axis_3: Option<String>,
    ) {
        self.axis_1 = axis_1;
        self.axis_2 = axis_2;
        self.axis_3 = axis_3;
    }

    pub fn axis_1(&self) -> Option<&str> {
        self.axis_1.as_deref()
    }

    pub fn set_axis_1(&mut self, a: Option<String>) {
        self.axis_1 = a;
    }

    pub fn axis_2(&self) -> Option<&str> {
        self.axis_2.as_deref()
    }

    pub fn set_axis_2(&mut self, a: Option<String>) {
        self.axis_2 = a;
    }

    pub fn axis_3(&self) -> Option<&str> {
        self.axis_3.as_deref()
    }

    pub fn set_axis_3(&mut self, a: Option<String>) {
        self.axis_3 = a;
    }
}

impl Default for CurveElementEndCoordinateSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let cs = CurveElementEndCoordinateSystem::new();
        assert!(cs.axis_1().is_none());
    }

    #[test]
    fn test_init() {
        let mut cs = CurveElementEndCoordinateSystem::new();
        cs.init(
            Some("X".to_string()),
            Some("Y".to_string()),
            Some("Z".to_string()),
        );

        assert_eq!(cs.axis_1(), Some("X"));
        assert_eq!(cs.axis_2(), Some("Y"));
        assert_eq!(cs.axis_3(), Some("Z"));
    }
}
