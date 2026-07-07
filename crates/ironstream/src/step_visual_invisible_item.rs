// FILE: step_visual_invisible_item.rs
// occt: StepVisual_InvisibleItem

/// A union type selecting an invisible item in STEP representation.
#[derive(Clone, Debug, PartialEq)]
pub enum InvisibleItem {
    StyledItem(i32),
    PresentationItem(i32),
}

impl InvisibleItem {
    /// Creates an InvisibleItem from a styled item.
    pub fn styled_item(id: i32) -> Self {
        InvisibleItem::StyledItem(id)
    }

    /// Creates an InvisibleItem from a presentation item.
    pub fn presentation_item(id: i32) -> Self {
        InvisibleItem::PresentationItem(id)
    }

    /// Returns the case number.
    pub fn case_num(&self) -> i32 {
        match self {
            InvisibleItem::StyledItem(_) => 1,
            InvisibleItem::PresentationItem(_) => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invisible_item_styled_item() {
        let item = InvisibleItem::styled_item(5);
        assert_eq!(item.case_num(), 1);
    }

    #[test]
    fn test_invisible_item_presentation_item() {
        let item = InvisibleItem::presentation_item(10);
        assert_eq!(item.case_num(), 2);
    }
}
