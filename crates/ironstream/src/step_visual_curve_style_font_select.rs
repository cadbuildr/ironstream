// FILE: step_visual_curve_style_font_select.rs
// occt: StepVisual_CurveStyleFontSelect

/// A union type selecting a curve style font in STEP representation.
#[derive(Clone, Debug, PartialEq)]
pub enum CurveStyleFontSelect {
    CurveStyleFont(i32),
    PreDefinedCurveFont(i32),
    ExternallyDefinedCurveFont(i32),
}

impl CurveStyleFontSelect {
    /// Creates a CurveStyleFontSelect from a CurveStyleFont.
    pub fn curve_style_font(value: i32) -> Self {
        CurveStyleFontSelect::CurveStyleFont(value)
    }

    /// Creates a CurveStyleFontSelect from a PreDefinedCurveFont.
    pub fn pre_defined_curve_font(value: i32) -> Self {
        CurveStyleFontSelect::PreDefinedCurveFont(value)
    }

    /// Creates a CurveStyleFontSelect from an ExternallyDefinedCurveFont.
    pub fn externally_defined_curve_font(value: i32) -> Self {
        CurveStyleFontSelect::ExternallyDefinedCurveFont(value)
    }

    /// Returns the case number (1 = CurveStyleFont, 2 = PreDefinedCurveFont, 3 = ExternallyDefinedCurveFont).
    pub fn case_num(&self) -> i32 {
        match self {
            CurveStyleFontSelect::CurveStyleFont(_) => 1,
            CurveStyleFontSelect::PreDefinedCurveFont(_) => 2,
            CurveStyleFontSelect::ExternallyDefinedCurveFont(_) => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curve_style_font_select() {
        let sel = CurveStyleFontSelect::curve_style_font(1);
        assert_eq!(sel.case_num(), 1);

        let sel2 = CurveStyleFontSelect::pre_defined_curve_font(2);
        assert_eq!(sel2.case_num(), 2);

        let sel3 = CurveStyleFontSelect::externally_defined_curve_font(3);
        assert_eq!(sel3.case_num(), 3);
    }
}
