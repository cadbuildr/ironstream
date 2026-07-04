// FILE: geom_to_step_make_direction.rs
// occt: GeomToStep_MakeDirection

#[derive(Clone, Debug)]
pub struct StepGeom_Direction {
    pub components: (f64, f64, f64),
}

pub struct GeomToStep_MakeDirection {
    done: bool,
    result: Option<StepGeom_Direction>,
}

impl GeomToStep_MakeDirection {
    pub fn new() -> Self {
        GeomToStep_MakeDirection {
            done: false,
            result: None,
        }
    }

    pub fn from_components(x: f64, y: f64, z: f64) -> Self {
        let mut conv = Self::new();
        let norm = (x * x + y * y + z * z).sqrt();
        if norm > 1e-10 {
            conv.result = Some(StepGeom_Direction {
                components: (x / norm, y / norm, z / norm),
            });
            conv.done = true;
        }
        conv
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn value(&self) -> Option<&StepGeom_Direction> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakeDirection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_components() {
        let conv = GeomToStep_MakeDirection::from_components(1.0, 0.0, 0.0);
        assert!(conv.is_done());
        let dir = conv.value().unwrap();
        assert!((dir.components.0 - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_zero_direction() {
        let conv = GeomToStep_MakeDirection::from_components(0.0, 0.0, 0.0);
        assert!(!conv.is_done());
    }
}
