// FILE: geom_to_step_make_bounded_curve.rs
// occt: GeomToStep_MakeBoundedCurve

#[derive(Clone, Debug)]
pub struct StepGeom_BoundedCurve {
    pub curve_type: String,
}

pub struct GeomToStep_MakeBoundedCurve {
    done: bool,
    result: Option<StepGeom_BoundedCurve>,
}

impl GeomToStep_MakeBoundedCurve {
    pub fn new() -> Self {
        GeomToStep_MakeBoundedCurve {
            done: false,
            result: None,
        }
    }

    pub fn from_curve_type(curve_type: &str) -> Self {
        let mut conv = Self::new();
        if !curve_type.is_empty() {
            conv.result = Some(StepGeom_BoundedCurve {
                curve_type: curve_type.to_string(),
            });
            conv.done = true;
        }
        conv
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn value(&self) -> Option<&StepGeom_BoundedCurve> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakeBoundedCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let conv = GeomToStep_MakeBoundedCurve::new();
        assert!(!conv.is_done());
    }

    #[test]
    fn test_from_curve_type() {
        let conv = GeomToStep_MakeBoundedCurve::from_curve_type("BSpline");
        assert!(conv.is_done());
        assert_eq!(conv.value().unwrap().curve_type, "BSpline");
    }
}
