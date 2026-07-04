// FILE: geom_to_step_make_axis2_placement2d.rs
// occt: GeomToStep_MakeAxis2Placement2d

/// Converts geometric axis2 placement to STEP Axis2Placement2d
#[derive(Clone, Debug)]
pub struct StepGeom_Axis2Placement2d {
    pub location: (f64, f64),
    pub ref_direction: (f64, f64),
}

impl Default for StepGeom_Axis2Placement2d {
    fn default() -> Self {
        StepGeom_Axis2Placement2d {
            location: (0.0, 0.0),
            ref_direction: (1.0, 0.0),
        }
    }
}

pub struct GeomToStep_MakeAxis2Placement2d {
    done: bool,
    result: Option<StepGeom_Axis2Placement2d>,
}

impl GeomToStep_MakeAxis2Placement2d {
    pub fn new() -> Self {
        GeomToStep_MakeAxis2Placement2d {
            done: false,
            result: None,
        }
    }

    pub fn from_location_and_direction(lx: f64, ly: f64, refx: f64, refy: f64) -> Self {
        let norm = (refx * refx + refy * refy).sqrt();
        let mut conv = Self::new();
        if norm > 1e-10 {
            conv.result = Some(StepGeom_Axis2Placement2d {
                location: (lx, ly),
                ref_direction: (refx / norm, refy / norm),
            });
            conv.done = true;
        }
        conv
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn value(&self) -> Option<&StepGeom_Axis2Placement2d> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakeAxis2Placement2d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let conv = GeomToStep_MakeAxis2Placement2d::new();
        assert!(!conv.is_done());
    }

    #[test]
    fn test_from_location_and_direction() {
        let conv = GeomToStep_MakeAxis2Placement2d::from_location_and_direction(1.0, 2.0, 1.0, 0.0);
        assert!(conv.is_done());
        let result = conv.value().unwrap();
        assert_eq!(result.location, (1.0, 2.0));
    }

    #[test]
    fn test_zero_direction() {
        let conv = GeomToStep_MakeAxis2Placement2d::from_location_and_direction(0.0, 0.0, 0.0, 0.0);
        assert!(!conv.is_done());
    }
}
