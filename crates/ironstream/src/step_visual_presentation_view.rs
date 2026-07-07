// FILE: step_visual_presentation_view.rs
// occt: StepVisual_PresentationView

pub struct PresentationView {
    _data: (),
}

impl PresentationView {
    pub fn new() -> Self {
        PresentationView { _data: () }
    }
}

impl Default for PresentationView {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pv = PresentationView::new();
        let _pv2 = PresentationView::new();
        let _ = pv;
    }

    #[test]
    fn test_default() {
        let pv = PresentationView::default();
        let _pv2 = PresentationView::new();
        let _ = pv;
    }
}
