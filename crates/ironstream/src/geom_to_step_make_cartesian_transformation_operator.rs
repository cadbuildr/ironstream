// FILE: geom_to_step_make_cartesian_transformation_operator.rs
// occt: GeomToStep_MakeCartesianTransformationOperator

#[derive(Clone, Debug)]
pub struct StepGeom_CartesianTransformationOperator {
    pub origin: (f64, f64, f64),
    pub scale: f64,
}

pub struct GeomToStep_MakeCartesianTransformationOperator {
    done: bool,
    result: Option<StepGeom_CartesianTransformationOperator>,
}

impl GeomToStep_MakeCartesianTransformationOperator {
    pub fn new() -> Self {
        GeomToStep_MakeCartesianTransformationOperator {
            done: false,
            result: None,
        }
    }

    pub fn from_origin_and_scale(ox: f64, oy: f64, oz: f64, scale: f64) -> Self {
        let mut conv = Self::new();
        if scale > 1e-10 {
            conv.result = Some(StepGeom_CartesianTransformationOperator {
                origin: (ox, oy, oz),
                scale,
            });
            conv.done = true;
        }
        conv
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn value(&self) -> Option<&StepGeom_CartesianTransformationOperator> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakeCartesianTransformationOperator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let conv = GeomToStep_MakeCartesianTransformationOperator::new();
        assert!(!conv.is_done());
    }

    #[test]
    fn test_from_origin_and_scale() {
        let conv = GeomToStep_MakeCartesianTransformationOperator::from_origin_and_scale(1.0, 2.0, 3.0, 2.0);
        assert!(conv.is_done());
    }

    #[test]
    fn test_invalid_scale() {
        let conv = GeomToStep_MakeCartesianTransformationOperator::from_origin_and_scale(0.0, 0.0, 0.0, 0.0);
        assert!(!conv.is_done());
    }
}
