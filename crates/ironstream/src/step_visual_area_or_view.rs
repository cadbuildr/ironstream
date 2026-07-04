// FILE: step_visual_area_or_view.rs
// occt: StepVisual_AreaOrView

/// Represents a union type for Area or View
#[derive(Debug, Clone)]
pub enum StepVisual_AreaOrView {
    Area(String),
    View(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_variant() {
        let aov = StepVisual_AreaOrView::Area("area1".to_string());
        match aov {
            StepVisual_AreaOrView::Area(ref a) => assert_eq!(a, "area1"),
            _ => panic!("Expected Area variant"),
        }
    }

    #[test]
    fn test_view_variant() {
        let aov = StepVisual_AreaOrView::View("view1".to_string());
        match aov {
            StepVisual_AreaOrView::View(ref v) => assert_eq!(v, "view1"),
            _ => panic!("Expected View variant"),
        }
    }
}
