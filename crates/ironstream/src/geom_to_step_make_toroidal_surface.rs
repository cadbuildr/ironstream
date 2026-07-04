// FILE: geom_to_step_make_toroidal_surface.rs
// occt: GeomToStep_MakeToroidalSurface

#[derive(Clone, Debug)]
pub struct StepGeom_ToroidalSurface {
    pub center: (f64, f64, f64),
    pub major_radius: f64,
    pub minor_radius: f64,
}

pub struct GeomToStep_MakeToroidalSurface {
    done: bool,
    result: Option<StepGeom_ToroidalSurface>,
}

impl GeomToStep_MakeToroidalSurface {
    pub fn new() -> Self {
        GeomToStep_MakeToroidalSurface {
            done: false,
            result: None,
        }
    }

    pub fn from_center_and_radii(cx: f64, cy: f64, cz: f64, major: f64, minor: f64) -> Self {
        let mut conv = Self::new();
        if major > minor && minor > 1e-10 {
            conv.result = Some(StepGeom_ToroidalSurface {
                center: (cx, cy, cz),
                major_radius: major,
                minor_radius: minor,
            });
            conv.done = true;
        }
        conv
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn value(&self) -> Option<&StepGeom_ToroidalSurface> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakeToroidalSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_center_and_radii() {
        let conv = GeomToStep_MakeToroidalSurface::from_center_and_radii(0.0, 0.0, 0.0, 3.0, 1.0);
        assert!(conv.is_done());
    }
}
