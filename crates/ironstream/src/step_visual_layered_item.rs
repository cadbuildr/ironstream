// FILE: step_visual_layered_item.rs
// occt: StepVisual_LayeredItem

/// A union type selecting a layered item in STEP representation.
#[derive(Clone, Debug, PartialEq)]
pub enum LayeredItem {
    StyledItem(i32),
    PresentationRepresentation(i32),
}

impl LayeredItem {
    /// Creates a LayeredItem from a styled item.
    pub fn styled_item(id: i32) -> Self {
        LayeredItem::StyledItem(id)
    }

    /// Creates a LayeredItem from a presentation representation.
    pub fn presentation_representation(id: i32) -> Self {
        LayeredItem::PresentationRepresentation(id)
    }

    /// Returns the case number.
    pub fn case_num(&self) -> i32 {
        match self {
            LayeredItem::StyledItem(_) => 1,
            LayeredItem::PresentationRepresentation(_) => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layered_item_styled_item() {
        let item = LayeredItem::styled_item(3);
        assert_eq!(item.case_num(), 1);
    }

    #[test]
    fn test_layered_item_presentation_representation() {
        let item = LayeredItem::presentation_representation(8);
        assert_eq!(item.case_num(), 2);
    }
}
