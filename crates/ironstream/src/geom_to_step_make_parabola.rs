// FILE: geom_to_step_make_parabola.rs
// occt: GeomToStep_MakeParabola

#[derive(Clone, Debug)]
pub struct StepGeom_Parabola {
    pub vertex: (f64, f64, f64),
    pub focal_length: f64,
}

pub struct GeomToStep_MakeParabola {
    done: bool,
    result: Option<StepGeom_Parabola>,
}

impl GeomToStep_MakeParabola {
    pub fn new() -> Self {
        GeomToStep_MakeParabola {
            done: false,
            result: None,
        }
    }

    pub fn from_vertex_and_focal_length(vx: f64, vy: f64, vz: f64, focal: f64) -> Self {
        let mut conv = Self::new();
        if focal > 1e-10 {
            conv.result = Some(StepGeom_Parabola {
                vertex: (vx, vy, vz),
                focal_length: focal,
            });
            conv.done = true;
        }
        conv
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn value(&self) -> Option<&StepGeom_Parabola> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakeParabola {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_vertex_and_focal_length() {
        let conv = GeomToStep_MakeParabola::from_vertex_and_focal_length(0.0, 0.0, 0.0, 1.0);
        assert!(conv.is_done());
    }
}
