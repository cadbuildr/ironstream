// FILE: geom_to_step_make_curve.rs
// occt: GeomToStep_MakeCurve

#[derive(Clone, Debug)]
pub struct StepGeom_Curve {
    pub curve_type: String,
}

pub struct GeomToStep_MakeCurve {
    done: bool,
    result: Option<StepGeom_Curve>,
}

impl GeomToStep_MakeCurve {
    pub fn new() -> Self {
        GeomToStep_MakeCurve {
            done: false,
            result: None,
        }
    }

    pub fn from_type(curve_type: &str) -> Self {
        let mut conv = Self::new();
        if !curve_type.is_empty() {
            conv.result = Some(StepGeom_Curve {
                curve_type: curve_type.to_string(),
            });
            conv.done = true;
        }
        conv
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn value(&self) -> Option<&StepGeom_Curve> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakeCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_type() {
        let conv = GeomToStep_MakeCurve::from_type("Line");
        assert!(conv.is_done());
    }
}
