// FILE: geom_to_step_make_plane.rs
// occt: GeomToStep_MakePlane

#[derive(Clone, Debug)]
pub struct StepGeom_Plane {
    pub point: (f64, f64, f64),
    pub normal: (f64, f64, f64),
}

pub struct GeomToStep_MakePlane {
    done: bool,
    result: Option<StepGeom_Plane>,
}

impl GeomToStep_MakePlane {
    pub fn new() -> Self {
        GeomToStep_MakePlane {
            done: false,
            result: None,
        }
    }

    pub fn from_point_and_normal(
        px: f64, py: f64, pz: f64,
        nx: f64, ny: f64, nz: f64,
    ) -> Self {
        let mut conv = Self::new();
        let norm = (nx * nx + ny * ny + nz * nz).sqrt();
        if norm > 1e-10 {
            conv.result = Some(StepGeom_Plane {
                point: (px, py, pz),
                normal: (nx / norm, ny / norm, nz / norm),
            });
            conv.done = true;
        }
        conv
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn value(&self) -> Option<&StepGeom_Plane> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakePlane {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_point_and_normal() {
        let conv = GeomToStep_MakePlane::from_point_and_normal(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        assert!(conv.is_done());
    }

    #[test]
    fn test_zero_normal() {
        let conv = GeomToStep_MakePlane::from_point_and_normal(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert!(!conv.is_done());
    }
}
