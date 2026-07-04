// FILE: step_visual_tessellated_shape_representation.rs
// occt: StepVisual_TessellatedShapeRepresentation

pub struct TessellatedShapeRepresentation {
    _data: (),
}

impl TessellatedShapeRepresentation {
    pub fn new() -> Self {
        TessellatedShapeRepresentation { _data: () }
    }
}

impl Default for TessellatedShapeRepresentation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tsr = TessellatedShapeRepresentation::new();
        let _tsr2 = TessellatedShapeRepresentation::new();
        let _ = tsr;
    }

    #[test]
    fn test_default() {
        let tsr = TessellatedShapeRepresentation::default();
        let _tsr2 = TessellatedShapeRepresentation::new();
        let _ = tsr;
    }
}
