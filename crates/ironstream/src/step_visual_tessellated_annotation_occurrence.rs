// FILE: step_visual_tessellated_annotation_occurrence.rs
// occt: StepVisual_TessellatedAnnotationOccurrence

pub struct TessellatedAnnotationOccurrence {
    _data: (),
}

impl TessellatedAnnotationOccurrence {
    pub fn new() -> Self {
        TessellatedAnnotationOccurrence { _data: () }
    }
}

impl Default for TessellatedAnnotationOccurrence {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tao = TessellatedAnnotationOccurrence::new();
        let _tao2 = TessellatedAnnotationOccurrence::new();
        let _ = tao;
    }

    #[test]
    fn test_default() {
        let tao = TessellatedAnnotationOccurrence::default();
        let _tao2 = TessellatedAnnotationOccurrence::new();
        let _ = tao;
    }
}
