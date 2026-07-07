// FILE: step_visual_font_select.rs
// occt: StepVisual_FontSelect

/// A union type selecting a text font in STEP representation.
#[derive(Clone, Debug, PartialEq)]
pub enum FontSelect {
    PreDefinedFont(i32),
    ExternallyDefinedFont(i32),
}

impl FontSelect {
    /// Creates a FontSelect from a pre-defined font.
    pub fn pre_defined_font(id: i32) -> Self {
        FontSelect::PreDefinedFont(id)
    }

    /// Creates a FontSelect from an externally defined font.
    pub fn externally_defined_font(id: i32) -> Self {
        FontSelect::ExternallyDefinedFont(id)
    }

    /// Returns the case number.
    pub fn case_num(&self) -> i32 {
        match self {
            FontSelect::PreDefinedFont(_) => 1,
            FontSelect::ExternallyDefinedFont(_) => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_select_pre_defined() {
        let sel = FontSelect::pre_defined_font(1);
        assert_eq!(sel.case_num(), 1);
    }

    #[test]
    fn test_font_select_externally_defined() {
        let sel = FontSelect::externally_defined_font(2);
        assert_eq!(sel.case_num(), 2);
    }
}
