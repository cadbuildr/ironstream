// FILE: step_visual_presentation_set.rs
// occt: StepVisual_PresentationSet

pub struct PresentationSet {
    // Inherits from Standard_Transient, base class with reference counting
    _data: (),
}

impl PresentationSet {
    pub fn new() -> Self {
        PresentationSet { _data: () }
    }
}

impl Default for PresentationSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ps = PresentationSet::new();
        let _ps2 = PresentationSet::new();
        let _ = ps;
    }

    #[test]
    fn test_default() {
        let ps = PresentationSet::default();
        let _ps2 = PresentationSet::new();
        let _ = ps;
    }
}
