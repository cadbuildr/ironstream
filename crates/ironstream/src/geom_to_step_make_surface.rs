// FILE: geom_to_step_make_surface.rs
// occt: GeomToStep_MakeSurface

#[derive(Clone, Debug)]
pub struct StepGeom_Surface {
    pub surface_type: String,
}

pub struct GeomToStep_MakeSurface {
    done: bool,
    result: Option<StepGeom_Surface>,
}

impl GeomToStep_MakeSurface {
    pub fn new() -> Self {
        GeomToStep_MakeSurface {
            done: false,
            result: None,
        }
    }

    pub fn from_type(surface_type: &str) -> Self {
        let mut conv = Self::new();
        if !surface_type.is_empty() {
            conv.result = Some(StepGeom_Surface {
                surface_type: surface_type.to_string(),
            });
            conv.done = true;
        }
        conv
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn value(&self) -> Option<&StepGeom_Surface> {
        self.result.as_ref()
    }
}

impl Default for GeomToStep_MakeSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_type() {
        let conv = GeomToStep_MakeSurface::from_type("BSplineSurface");
        assert!(conv.is_done());
    }
}
