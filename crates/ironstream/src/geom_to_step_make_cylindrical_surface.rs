// FILE: geom_to_step_make_cylindrical_surface.rs
// occt: GeomToStep_MakeCylindricalSurface

#[derive(Clone, Debug)]
pub struct StepGeom_CylindricalSurface {
    pub axis: (f64, f64, f64),
    pub radius: f64,
}

pub struct GeomToStep_MakeCylindricalSurface {
    done: bool,
    result: Option<StepGeom_CylindricalSurface>,
}

impl GeomToStep_MakeCylindricalSurface {
    pub fn new() -> Self {
        GeomToStep_MakeCylindricalSurface {
            done: false,
            result: None,
        }
    }

    pub fn from_axis_and_radius(ax: f64, ay: f64, az: f64, radius: f64) -> Self {
        let mut conv = Self::new();
        let norm = (ax * ax + ay * ay + az * az).sqrt();
        if norm > 1e-10 && radius > 1e-10 {
            conv.result = Some(StepGeom_CylindricalSurface {
                axis: (ax / norm, ay / norm, az / norm),
                radius,
            });
            conv.done = true;
        }
        conv
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn value(&self) -> Option<&StepGeom_CylindricalSurface> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakeCylindricalSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_axis_and_radius() {
        let conv = GeomToStep_MakeCylindricalSurface::from_axis_and_radius(0.0, 0.0, 1.0, 5.0);
        assert!(conv.is_done());
    }

    #[test]
    fn test_invalid_radius() {
        let conv = GeomToStep_MakeCylindricalSurface::from_axis_and_radius(0.0, 0.0, 1.0, 0.0);
        assert!(!conv.is_done());
    }
}
