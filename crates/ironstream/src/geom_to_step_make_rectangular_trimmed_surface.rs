// FILE: geom_to_step_make_rectangular_trimmed_surface.rs
// occt: GeomToStep_MakeRectangularTrimmedSurface

#[derive(Clone, Debug)]
pub struct StepGeom_RectangularTrimmedSurface {
    pub u_min: f64,
    pub u_max: f64,
    pub v_min: f64,
    pub v_max: f64,
}

pub struct GeomToStep_MakeRectangularTrimmedSurface {
    done: bool,
    result: Option<StepGeom_RectangularTrimmedSurface>,
}

impl GeomToStep_MakeRectangularTrimmedSurface {
    pub fn new() -> Self {
        GeomToStep_MakeRectangularTrimmedSurface {
            done: false,
            result: None,
        }
    }

    pub fn from_bounds(u_min: f64, u_max: f64, v_min: f64, v_max: f64) -> Self {
        let mut conv = Self::new();
        if u_min < u_max && v_min < v_max {
            conv.result = Some(StepGeom_RectangularTrimmedSurface {
                u_min,
                u_max,
                v_min,
                v_max,
            });
            conv.done = true;
        }
        conv
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn value(&self) -> Option<&StepGeom_RectangularTrimmedSurface> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakeRectangularTrimmedSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_bounds() {
        let conv = GeomToStep_MakeRectangularTrimmedSurface::from_bounds(0.0, 1.0, 0.0, 1.0);
        assert!(conv.is_done());
    }

    #[test]
    fn test_invalid_bounds() {
        let conv = GeomToStep_MakeRectangularTrimmedSurface::from_bounds(1.0, 0.0, 0.0, 1.0);
        assert!(!conv.is_done());
    }
}
