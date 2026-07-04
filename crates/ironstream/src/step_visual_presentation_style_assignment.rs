// FILE: step_visual_presentation_style_assignment.rs
// occt: StepVisual_PresentationStyleAssignment

use std::sync::Arc;

pub struct PresentationStyleSelect;

pub struct PresentationStyleAssignment {
    styles: Option<Arc<Vec<PresentationStyleSelect>>>,
}

impl PresentationStyleAssignment {
    pub fn new() -> Self {
        PresentationStyleAssignment { styles: None }
    }

    pub fn init(&mut self, styles: Option<Arc<Vec<PresentationStyleSelect>>>) {
        self.styles = styles;
    }

    pub fn set_styles(&mut self, styles: Option<Arc<Vec<PresentationStyleSelect>>>) {
        self.styles = styles;
    }

    pub fn styles(&self) -> Option<&Arc<Vec<PresentationStyleSelect>>> {
        self.styles.as_ref()
    }

    pub fn styles_value(&self, num: usize) -> Option<&PresentationStyleSelect> {
        self.styles.as_ref().and_then(|styles| styles.get(num))
    }

    pub fn nb_styles(&self) -> usize {
        self.styles.as_ref().map(|s| s.len()).unwrap_or(0)
    }
}

impl Default for PresentationStyleAssignment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let psa = PresentationStyleAssignment::new();
        assert_eq!(psa.nb_styles(), 0);
        assert!(psa.styles().is_none());
    }

    #[test]
    fn test_set_and_get_styles() {
        let mut psa = PresentationStyleAssignment::new();
        let styles = vec![PresentationStyleSelect, PresentationStyleSelect];
        psa.set_styles(Some(Arc::new(styles)));
        assert_eq!(psa.nb_styles(), 2);
        assert!(psa.styles().is_some());
        assert!(psa.styles_value(0).is_some());
        assert!(psa.styles_value(1).is_some());
        assert!(psa.styles_value(2).is_none());
    }

    #[test]
    fn test_init() {
        let mut psa = PresentationStyleAssignment::new();
        let styles = vec![PresentationStyleSelect];
        psa.init(Some(Arc::new(styles)));
        assert_eq!(psa.nb_styles(), 1);
        assert!(psa.styles_value(0).is_some());
    }

    #[test]
    fn test_empty_styles() {
        let psa = PresentationStyleAssignment::new();
        assert_eq!(psa.nb_styles(), 0);
        assert!(psa.styles_value(0).is_none());
    }
}
