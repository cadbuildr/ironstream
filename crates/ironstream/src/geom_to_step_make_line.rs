// FILE: geom_to_step_make_line.rs
// occt: GeomToStep_MakeLine

#[derive(Clone, Debug)]
pub struct StepGeom_Line {
    pub point: (f64, f64, f64),
    pub direction: (f64, f64, f64),
}

pub struct GeomToStep_MakeLine {
    done: bool,
    result: Option<StepGeom_Line>,
}

impl GeomToStep_MakeLine {
    pub fn new() -> Self {
        GeomToStep_MakeLine {
            done: false,
            result: None,
        }
    }

    pub fn from_point_and_direction(
        px: f64, py: f64, pz: f64,
        dx: f64, dy: f64, dz: f64,
    ) -> Self {
        let mut conv = Self::new();
        let norm = (dx * dx + dy * dy + dz * dz).sqrt();
        if norm > 1e-10 {
            conv.result = Some(StepGeom_Line {
                point: (px, py, pz),
                direction: (dx / norm, dy / norm, dz / norm),
            });
            conv.done = true;
        }
        conv
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn value(&self) -> Option<&StepGeom_Line> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakeLine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_point_and_direction() {
        let conv = GeomToStep_MakeLine::from_point_and_direction(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        assert!(conv.is_done());
    }

    #[test]
    fn test_zero_direction() {
        let conv = GeomToStep_MakeLine::from_point_and_direction(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert!(!conv.is_done());
    }
}
