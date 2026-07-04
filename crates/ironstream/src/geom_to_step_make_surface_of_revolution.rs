// FILE: geom_to_step_make_surface_of_revolution.rs
// occt: GeomToStep_MakeSurfaceOfRevolution

#[derive(Clone, Debug)]
pub struct StepGeom_SurfaceOfRevolution {
    pub axis_location: (f64, f64, f64),
    pub axis_direction: (f64, f64, f64),
}

pub struct GeomToStep_MakeSurfaceOfRevolution {
    done: bool,
    result: Option<StepGeom_SurfaceOfRevolution>,
}

impl GeomToStep_MakeSurfaceOfRevolution {
    pub fn new() -> Self {
        GeomToStep_MakeSurfaceOfRevolution {
            done: false,
            result: None,
        }
    }

    pub fn from_axis(lx: f64, ly: f64, lz: f64, dx: f64, dy: f64, dz: f64) -> Self {
        let mut conv = Self::new();
        let norm = (dx * dx + dy * dy + dz * dz).sqrt();
        if norm > 1e-10 {
            conv.result = Some(StepGeom_SurfaceOfRevolution {
                axis_location: (lx, ly, lz),
                axis_direction: (dx / norm, dy / norm, dz / norm),
            });
            conv.done = true;
        }
        conv
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn value(&self) -> Option<&StepGeom_SurfaceOfRevolution> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakeSurfaceOfRevolution {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_axis() {
        let conv = GeomToStep_MakeSurfaceOfRevolution::from_axis(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        assert!(conv.is_done());
    }
}
