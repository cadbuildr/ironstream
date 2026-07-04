// FILE: step_visual_fill_style_select.rs
// occt: StepVisual_FillStyleSelect

/// A union type selecting a fill style in STEP representation.
#[derive(Clone, Debug, PartialEq)]
pub enum FillStyleSelect {
    Colour(i32),
    HatchStyle(i32),
    PatternFill(i32),
}

impl FillStyleSelect {
    /// Creates a FillStyleSelect from a colour.
    pub fn colour(id: i32) -> Self {
        FillStyleSelect::Colour(id)
    }

    /// Creates a FillStyleSelect from a hatch style.
    pub fn hatch_style(id: i32) -> Self {
        FillStyleSelect::HatchStyle(id)
    }

    /// Creates a FillStyleSelect from a pattern fill.
    pub fn pattern_fill(id: i32) -> Self {
        FillStyleSelect::PatternFill(id)
    }

    /// Returns the case number.
    pub fn case_num(&self) -> i32 {
        match self {
            FillStyleSelect::Colour(_) => 1,
            FillStyleSelect::HatchStyle(_) => 2,
            FillStyleSelect::PatternFill(_) => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fill_style_select_colour() {
        let sel = FillStyleSelect::colour(1);
        assert_eq!(sel.case_num(), 1);
    }

    #[test]
    fn test_fill_style_select_hatch_style() {
        let sel = FillStyleSelect::hatch_style(2);
        assert_eq!(sel.case_num(), 2);
    }

    #[test]
    fn test_fill_style_select_pattern_fill() {
        let sel = FillStyleSelect::pattern_fill(3);
        assert_eq!(sel.case_num(), 3);
    }
}
