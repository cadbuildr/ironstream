// FILE: geom_to_step_make_bounded_surface.rs
// occt: GeomToStep_MakeBoundedSurface

#[derive(Clone, Debug)]
pub struct StepGeom_BoundedSurface {
    pub surface_type: String,
}

pub struct GeomToStep_MakeBoundedSurface {
    done: bool,
    result: Option<StepGeom_BoundedSurface>,
}

impl GeomToStep_MakeBoundedSurface {
    pub fn new() -> Self {
        GeomToStep_MakeBoundedSurface {
            done: false,
            result: None,
        }
    }

    pub fn from_surface_type(surface_type: &str) -> Self {
        let mut conv = Self::new();
        if !surface_type.is_empty() {
            conv.result = Some(StepGeom_BoundedSurface {
                surface_type: surface_type.to_string(),
            });
            conv.done = true;
        }
        conv
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn value(&self) -> Option<&StepGeom_BoundedSurface> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakeBoundedSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let conv = GeomToStep_MakeBoundedSurface::new();
        assert!(!conv.is_done());
    }

    #[test]
    fn test_from_surface_type() {
        let conv = GeomToStep_MakeBoundedSurface::from_surface_type("TrimmedSurface");
        assert!(conv.is_done());
    }
}
