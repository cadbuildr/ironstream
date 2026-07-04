// FILE: geom_to_step_make_conic.rs
// occt: GeomToStep_MakeConic

#[derive(Clone, Debug)]
pub struct StepGeom_Conic {
    pub conic_type: String,
}

pub struct GeomToStep_MakeConic {
    done: bool,
    result: Option<StepGeom_Conic>,
}

impl GeomToStep_MakeConic {
    pub fn new() -> Self {
        GeomToStep_MakeConic {
            done: false,
            result: None,
        }
    }

    pub fn from_type(conic_type: &str) -> Self {
        let mut conv = Self::new();
        if !conic_type.is_empty() {
            conv.result = Some(StepGeom_Conic {
                conic_type: conic_type.to_string(),
            });
            conv.done = true;
        }
        conv
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn value(&self) -> Option<&StepGeom_Conic> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakeConic {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_type() {
        let conv = GeomToStep_MakeConic::from_type("Circle");
        assert!(conv.is_done());
    }
}
