// FILE: step_visual_presentation_style_by_context.rs
// occt: StepVisual_PresentationStyleByContext

use std::sync::Arc;

pub struct PresentationStyleSelect;
pub struct StyleContextSelect;

pub struct PresentationStyleByContext {
    styles: Option<Arc<Vec<PresentationStyleSelect>>>,
    style_context: Option<StyleContextSelect>,
}

impl PresentationStyleByContext {
    pub fn new() -> Self {
        PresentationStyleByContext {
            styles: None,
            style_context: None,
        }
    }

    pub fn init(
        &mut self,
        styles: Option<Arc<Vec<PresentationStyleSelect>>>,
        style_context: Option<StyleContextSelect>,
    ) {
        self.styles = styles;
        self.style_context = style_context;
    }

    pub fn set_style_context(&mut self, context: Option<StyleContextSelect>) {
        self.style_context = context;
    }

    pub fn style_context(&self) -> Option<&StyleContextSelect> {
        self.style_context.as_ref()
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

impl Default for PresentationStyleByContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let psbc = PresentationStyleByContext::new();
        assert!(psbc.style_context().is_none());
        assert_eq!(psbc.nb_styles(), 0);
    }

    #[test]
    fn test_set_and_get_style_context() {
        let mut psbc = PresentationStyleByContext::new();
        let context = StyleContextSelect::new();
        psbc.set_style_context(Some(context));
        assert!(psbc.style_context().is_some());
    }

    #[test]
    fn test_set_and_get_styles() {
        let mut psbc = PresentationStyleByContext::new();
        let styles = vec![PresentationStyleSelect];
        psbc.set_styles(Some(Arc::new(styles)));
        assert_eq!(psbc.nb_styles(), 1);
        assert!(psbc.styles().is_some());
    }

    #[test]
    fn test_init() {
        let mut psbc = PresentationStyleByContext::new();
        let styles = vec![PresentationStyleSelect];
        let context = StyleContextSelect::new();
        psbc.init(Some(Arc::new(styles)), Some(context));

        assert!(psbc.style_context().is_some());
        assert_eq!(psbc.nb_styles(), 1);
    }
}
