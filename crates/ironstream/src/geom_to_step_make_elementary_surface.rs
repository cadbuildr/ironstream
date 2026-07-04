// FILE: geom_to_step_make_elementary_surface.rs
// occt: GeomToStep_MakeElementarySurface

#[derive(Clone, Debug)]
pub struct StepGeom_ElementarySurface {
    pub surface_type: String,
}

pub struct GeomToStep_MakeElementarySurface {
    done: bool,
    result: Option<StepGeom_ElementarySurface>,
}

impl GeomToStep_MakeElementarySurface {
    pub fn new() -> Self {
        GeomToStep_MakeElementarySurface {
            done: false,
            result: None,
        }
    }

    pub fn from_type(surface_type: &str) -> Self {
        let mut conv = Self::new();
        if !surface_type.is_empty() {
            conv.result = Some(StepGeom_ElementarySurface {
                surface_type: surface_type.to_string(),
            });
            conv.done = true;
        }
        conv
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn value(&self) -> Option<&StepGeom_ElementarySurface> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakeElementarySurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_type() {
        let conv = GeomToStep_MakeElementarySurface::from_type("Plane");
        assert!(conv.is_done());
    }
}
