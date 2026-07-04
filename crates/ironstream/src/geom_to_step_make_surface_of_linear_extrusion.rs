// FILE: geom_to_step_make_surface_of_linear_extrusion.rs
// occt: GeomToStep_MakeSurfaceOfLinearExtrusion

#[derive(Clone, Debug)]
pub struct StepGeom_SurfaceOfLinearExtrusion {
    pub extrusion_direction: (f64, f64, f64),
}

pub struct GeomToStep_MakeSurfaceOfLinearExtrusion {
    done: bool,
    result: Option<StepGeom_SurfaceOfLinearExtrusion>,
}

impl GeomToStep_MakeSurfaceOfLinearExtrusion {
    pub fn new() -> Self {
        GeomToStep_MakeSurfaceOfLinearExtrusion {
            done: false,
            result: None,
        }
    }

    pub fn from_direction(dx: f64, dy: f64, dz: f64) -> Self {
        let mut conv = Self::new();
        let norm = (dx * dx + dy * dy + dz * dz).sqrt();
        if norm > 1e-10 {
            conv.result = Some(StepGeom_SurfaceOfLinearExtrusion {
                extrusion_direction: (dx / norm, dy / norm, dz / norm),
            });
            conv.done = true;
        }
        conv
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn value(&self) -> Option<&StepGeom_SurfaceOfLinearExtrusion> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakeSurfaceOfLinearExtrusion {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_direction() {
        let conv = GeomToStep_MakeSurfaceOfLinearExtrusion::from_direction(0.0, 0.0, 1.0);
        assert!(conv.is_done());
    }
}
