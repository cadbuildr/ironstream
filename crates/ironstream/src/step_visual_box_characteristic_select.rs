// FILE: step_visual_box_characteristic_select.rs
// occt: StepVisual_BoxCharacteristicSelect

/// Represents a union type for box characteristics
#[derive(Debug, Clone)]
pub enum StepVisual_BoxCharacteristicSelect {
    LineWidth(f64),
    LineStyle(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_width() {
        let bcs = StepVisual_BoxCharacteristicSelect::LineWidth(2.5);
        match bcs {
            StepVisual_BoxCharacteristicSelect::LineWidth(w) => assert_eq!(w, 2.5),
            _ => panic!("Expected LineWidth variant"),
        }
    }

    #[test]
    fn test_line_style() {
        let bcs = StepVisual_BoxCharacteristicSelect::LineStyle("solid".to_string());
        match bcs {
            StepVisual_BoxCharacteristicSelect::LineStyle(ref s) => assert_eq!(s, "solid"),
            _ => panic!("Expected LineStyle variant"),
        }
    }
}
