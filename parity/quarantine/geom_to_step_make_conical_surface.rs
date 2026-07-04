// FILE: geom_to_step_make_conical_surface.rs
// occt: GeomToStep_MakeConicalSurface

#[derive(Clone, Debug)]
pub struct StepGeom_ConicalSurface {
    pub apex: (f64, f64, f64),
    pub semi_angle: f64,
}

pub struct GeomToStep_MakeConicalSurface {
    done: bool,
    result: Option<StepGeom_ConicalSurface>,
}

impl GeomToStep_MakeConicalSurface {
    pub fn new() -> Self {
        GeomToStep_MakeConicalSurface {
            done: false,
            result: None,
        }
    }

    pub fn from_apex_and_angle(apex_x: f64, apex_y: f64, apex_z: f64, angle: f64) -> Self {
        let mut conv = Self::new();
        if angle > 0.0 && angle < std::f64::consts::PI / 2.0 {
            conv.result = Some(StepGeom_ConicalSurface {
                apex: (apex_x, apex_y, apex_z),
                semi_angle: angle,
            });
            conv.done = true;
        }
        conv
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn value(&self) -> Option<&StepGeom_ConicalSurface> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakeConicalSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_apex_and_angle() {
        let conv = GeomToStep_MakeConicalSurface::from_apex_and_angle(0.0, 0.0, 0.0, 0.5);
        assert!(conv.is_done());
    }

    #[test]
    fn test_invalid_angle() {
        let conv = GeomToStep_MakeConicalSurface::from_apex_and_angle(0.0, 0.0, 0.0, 0.0);
        assert!(!conv.is_done());
    }
}
