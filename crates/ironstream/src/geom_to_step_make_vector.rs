// FILE: geom_to_step_make_vector.rs
// occt: GeomToStep_MakeVector

#[derive(Clone, Debug)]
pub struct StepGeom_Vector {
    pub components: (f64, f64, f64),
    pub magnitude: f64,
}

pub struct GeomToStep_MakeVector {
    done: bool,
    result: Option<StepGeom_Vector>,
}

impl GeomToStep_MakeVector {
    pub fn new() -> Self {
        GeomToStep_MakeVector {
            done: false,
            result: None,
        }
    }

    pub fn from_components(x: f64, y: f64, z: f64) -> Self {
        let mut conv = Self::new();
        let magnitude = (x * x + y * y + z * z).sqrt();
        if magnitude > 1e-10 {
            conv.result = Some(StepGeom_Vector {
                components: (x, y, z),
                magnitude,
            });
            conv.done = true;
        }
        conv
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn value(&self) -> Option<&StepGeom_Vector> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakeVector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_components() {
        let conv = GeomToStep_MakeVector::from_components(3.0, 4.0, 0.0);
        assert!(conv.is_done());
        let vec = conv.value().unwrap();
        assert!((vec.magnitude - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_zero_vector() {
        let conv = GeomToStep_MakeVector::from_components(0.0, 0.0, 0.0);
        assert!(!conv.is_done());
    }
}
