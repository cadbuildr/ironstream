// FILE: d_data_std_draw_presentation.rs
// occt: DDataStd_DrawPresentation

//! Draw presentation for DDataStd attributes.

/// DDataStd_DrawPresentation: presentation of attributes.
#[derive(Clone, Debug)]
pub struct DDataStdDrawPresentation {
    id: u32,
    visible: bool,
}

impl DDataStdDrawPresentation {
    /// Create a new presentation.
    pub fn new(id: u32) -> Self {
        DDataStdDrawPresentation {
            id,
            visible: true,
        }
    }

    /// Show or hide the presentation.
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Check if presentation is visible.
    pub fn is_visible(&self) -> bool {
        self.visible
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presentation_creation() {
        let pres = DDataStdDrawPresentation::new(1);
        assert_eq!(pres.id, 1);
        assert!(pres.is_visible());
    }

    #[test]
    fn test_visibility() {
        let mut pres = DDataStdDrawPresentation::new(1);
        pres.set_visible(false);
        assert!(!pres.is_visible());
    }
}
