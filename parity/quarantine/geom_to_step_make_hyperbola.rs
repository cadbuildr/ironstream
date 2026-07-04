// FILE: geom_to_step_make_hyperbola.rs
// occt: GeomToStep_MakeHyperbola

#[derive(Clone, Debug)]
pub struct StepGeom_Hyperbola {
    pub center: (f64, f64, f64),
    pub real_axis: f64,
    pub imag_axis: f64,
}

pub struct GeomToStep_MakeHyperbola {
    done: bool,
    result: Option<StepGeom_Hyperbola>,
}

impl GeomToStep_MakeHyperbola {
    pub fn new() -> Self {
        GeomToStep_MakeHyperbola {
            done: false,
            result: None,
        }
    }

    pub fn from_center_and_axes(cx: f64, cy: f64, cz: f64, real: f64, imag: f64) -> Self {
        let mut conv = Self::new();
        if real > 1e-10 && imag > 1e-10 {
            conv.result = Some(StepGeom_Hyperbola {
                center: (cx, cy, cz),
                real_axis: real,
                imag_axis: imag,
            });
            conv.done = true;
        }
        conv
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn value(&self) -> Option<&StepGeom_Hyperbola> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakeHyperbola {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_center_and_axes() {
        let conv = GeomToStep_MakeHyperbola::from_center_and_axes(0.0, 0.0, 0.0, 2.0, 1.0);
        assert!(conv.is_done());
    }
}
