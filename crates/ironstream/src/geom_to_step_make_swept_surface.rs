// FILE: geom_to_step_make_swept_surface.rs
// occt: GeomToStep_MakeSweptSurface

#[derive(Clone, Debug)]
pub struct StepGeom_SweptSurface {
    pub swept_type: String,
}

pub struct GeomToStep_MakeSweptSurface {
    done: bool,
    result: Option<StepGeom_SweptSurface>,
}

impl GeomToStep_MakeSweptSurface {
    pub fn new() -> Self {
        GeomToStep_MakeSweptSurface {
            done: false,
            result: None,
        }
    }

    pub fn from_type(swept_type: &str) -> Self {
        let mut conv = Self::new();
        if !swept_type.is_empty() {
            conv.result = Some(StepGeom_SweptSurface {
                swept_type: swept_type.to_string(),
            });
            conv.done = true;
        }
        conv
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn value(&self) -> Option<&StepGeom_SweptSurface> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakeSweptSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_type() {
        let conv = GeomToStep_MakeSweptSurface::from_type("RevolutionSurface");
        assert!(conv.is_done());
    }
}
