// FILE: geom_to_step_make_ellipse.rs
// occt: GeomToStep_MakeEllipse

#[derive(Clone, Debug)]
pub struct StepGeom_Ellipse {
    pub center: (f64, f64, f64),
    pub major_radius: f64,
    pub minor_radius: f64,
}

pub struct GeomToStep_MakeEllipse {
    done: bool,
    result: Option<StepGeom_Ellipse>,
}

impl GeomToStep_MakeEllipse {
    pub fn new() -> Self {
        GeomToStep_MakeEllipse {
            done: false,
            result: None,
        }
    }

    pub fn from_center_and_radii(cx: f64, cy: f64, cz: f64, major: f64, minor: f64) -> Self {
        let mut conv = Self::new();
        if major > minor && minor > 1e-10 {
            conv.result = Some(StepGeom_Ellipse {
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

    pub fn value(&self) -> Option<&StepGeom_Ellipse> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakeEllipse {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_center_and_radii() {
        let conv = GeomToStep_MakeEllipse::from_center_and_radii(0.0, 0.0, 0.0, 2.0, 1.0);
        assert!(conv.is_done());
    }

    #[test]
    fn test_invalid_radii() {
        let conv = GeomToStep_MakeEllipse::from_center_and_radii(0.0, 0.0, 0.0, 1.0, 2.0);
        assert!(!conv.is_done());
    }
}
