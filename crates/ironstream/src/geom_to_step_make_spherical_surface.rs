// FILE: geom_to_step_make_spherical_surface.rs
// occt: GeomToStep_MakeSphericalSurface

#[derive(Clone, Debug)]
pub struct StepGeom_SphericalSurface {
    pub center: (f64, f64, f64),
    pub radius: f64,
}

pub struct GeomToStep_MakeSphericalSurface {
    done: bool,
    result: Option<StepGeom_SphericalSurface>,
}

impl GeomToStep_MakeSphericalSurface {
    pub fn new() -> Self {
        GeomToStep_MakeSphericalSurface {
            done: false,
            result: None,
        }
    }

    pub fn from_center_and_radius(cx: f64, cy: f64, cz: f64, radius: f64) -> Self {
        let mut conv = Self::new();
        if radius > 1e-10 {
            conv.result = Some(StepGeom_SphericalSurface {
                center: (cx, cy, cz),
                radius,
            });
            conv.done = true;
        }
        conv
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn value(&self) -> Option<&StepGeom_SphericalSurface> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakeSphericalSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_center_and_radius() {
        let conv = GeomToStep_MakeSphericalSurface::from_center_and_radius(0.0, 0.0, 0.0, 1.0);
        assert!(conv.is_done());
    }

    #[test]
    fn test_invalid_radius() {
        let conv = GeomToStep_MakeSphericalSurface::from_center_and_radius(0.0, 0.0, 0.0, 0.0);
        assert!(!conv.is_done());
    }
}
