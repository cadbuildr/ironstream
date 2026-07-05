// FILE: i_vtk_draw_highlight_and_selection_pipeline.rs
// occt: IVtkDraw_HighlightAndSelectionPipeline

/// VTK pipeline for handling highlight and selection of shapes in Draw module.
#[derive(Clone, Debug)]
pub struct IVtkDraw_HighlightAndSelectionPipeline {
    highlight_enabled: bool,
    selection_enabled: bool,
}

impl IVtkDraw_HighlightAndSelectionPipeline {
    /// Create a new highlight and selection pipeline.
    pub fn new() -> Self {
        IVtkDraw_HighlightAndSelectionPipeline {
            highlight_enabled: false,
            selection_enabled: false,
        }
    }

    /// Enable highlighting.
    pub fn set_highlight_enabled(&mut self, enabled: bool) {
        self.highlight_enabled = enabled;
    }

    /// Check if highlighting is enabled.
    pub fn is_highlight_enabled(&self) -> bool {
        self.highlight_enabled
    }

    /// Enable selection.
    pub fn set_selection_enabled(&mut self, enabled: bool) {
        self.selection_enabled = enabled;
    }

    /// Check if selection is enabled.
    pub fn is_selection_enabled(&self) -> bool {
        self.selection_enabled
    }
}

impl Default for IVtkDraw_HighlightAndSelectionPipeline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_pipeline() {
        let pipeline = IVtkDraw_HighlightAndSelectionPipeline::new();
        assert!(!pipeline.is_highlight_enabled());
        assert!(!pipeline.is_selection_enabled());
    }

    #[test]
    fn test_enable_highlight() {
        let mut pipeline = IVtkDraw_HighlightAndSelectionPipeline::new();
        pipeline.set_highlight_enabled(true);
        assert!(pipeline.is_highlight_enabled());
    }

    #[test]
    fn test_enable_selection() {
        let mut pipeline = IVtkDraw_HighlightAndSelectionPipeline::new();
        pipeline.set_selection_enabled(true);
        assert!(pipeline.is_selection_enabled());
    }
}
