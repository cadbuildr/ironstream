// FILE: step_visual_presentation_style_select.rs
// occt: StepVisual_PresentationStyleSelect

pub struct PointStyle;
pub struct CurveStyle;
pub struct NullStyleMember;
pub struct SurfaceStyleUsage;

pub struct PresentationStyleSelect {
    case: Option<SelectCase>,
    value: Option<Box<dyn std::any::Any>>,
}

#[derive(Clone, Copy)]
enum SelectCase {
    PointStyle = 1,
    CurveStyle = 2,
    SurfaceStyleUsage = 3,
    SymbolStyle = 4,
    FillAreaStyle = 5,
    TextStyle = 6,
    NullStyle = 7,
}

impl PresentationStyleSelect {
    pub fn new() -> Self {
        PresentationStyleSelect {
            case: None,
            value: None,
        }
    }

    pub fn case_num(&self) -> i32 {
        match self.case {
            Some(SelectCase::PointStyle) => 1,
            Some(SelectCase::CurveStyle) => 2,
            Some(SelectCase::SurfaceStyleUsage) => 3,
            Some(SelectCase::SymbolStyle) => 4,
            Some(SelectCase::FillAreaStyle) => 5,
            Some(SelectCase::TextStyle) => 6,
            Some(SelectCase::NullStyle) => 7,
            None => 0,
        }
    }

    pub fn point_style(&self) -> Option<&PointStyle> {
        if matches!(self.case, Some(SelectCase::PointStyle)) {
            self.value
                .as_ref()
                .and_then(|v| v.downcast_ref::<PointStyle>())
        } else {
            None
        }
    }

    pub fn curve_style(&self) -> Option<&CurveStyle> {
        if matches!(self.case, Some(SelectCase::CurveStyle)) {
            self.value
                .as_ref()
                .and_then(|v| v.downcast_ref::<CurveStyle>())
        } else {
            None
        }
    }

    pub fn null_style(&self) -> Option<&NullStyleMember> {
        if matches!(self.case, Some(SelectCase::NullStyle)) {
            self.value
                .as_ref()
                .and_then(|v| v.downcast_ref::<NullStyleMember>())
        } else {
            None
        }
    }

    pub fn surface_style_usage(&self) -> Option<&SurfaceStyleUsage> {
        if matches!(self.case, Some(SelectCase::SurfaceStyleUsage)) {
            self.value
                .as_ref()
                .and_then(|v| v.downcast_ref::<SurfaceStyleUsage>())
        } else {
            None
        }
    }

    pub fn set_point_style(&mut self, style: PointStyle) {
        self.case = Some(SelectCase::PointStyle);
        self.value = Some(Box::new(style));
    }

    pub fn set_curve_style(&mut self, style: CurveStyle) {
        self.case = Some(SelectCase::CurveStyle);
        self.value = Some(Box::new(style));
    }

    pub fn set_null_style(&mut self, style: NullStyleMember) {
        self.case = Some(SelectCase::NullStyle);
        self.value = Some(Box::new(style));
    }

    pub fn set_surface_style_usage(&mut self, usage: SurfaceStyleUsage) {
        self.case = Some(SelectCase::SurfaceStyleUsage);
        self.value = Some(Box::new(usage));
    }
}

impl Default for PresentationStyleSelect {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pss = PresentationStyleSelect::new();
        assert_eq!(pss.case_num(), 0);
        assert!(pss.point_style().is_none());
        assert!(pss.curve_style().is_none());
        assert!(pss.null_style().is_none());
        assert!(pss.surface_style_usage().is_none());
    }

    #[test]
    fn test_set_point_style() {
        let mut pss = PresentationStyleSelect::new();
        pss.set_point_style(PointStyle);
        assert_eq!(pss.case_num(), 1);
        assert!(pss.point_style().is_some());
    }

    #[test]
    fn test_set_curve_style() {
        let mut pss = PresentationStyleSelect::new();
        pss.set_curve_style(CurveStyle);
        assert_eq!(pss.case_num(), 2);
        assert!(pss.curve_style().is_some());
    }

    #[test]
    fn test_set_null_style() {
        let mut pss = PresentationStyleSelect::new();
        pss.set_null_style(NullStyleMember);
        assert_eq!(pss.case_num(), 7);
        assert!(pss.null_style().is_some());
    }

    #[test]
    fn test_set_surface_style_usage() {
        let mut pss = PresentationStyleSelect::new();
        pss.set_surface_style_usage(SurfaceStyleUsage);
        assert_eq!(pss.case_num(), 3);
        assert!(pss.surface_style_usage().is_some());
    }

    #[test]
    fn test_switch_styles() {
        let mut pss = PresentationStyleSelect::new();
        pss.set_point_style(PointStyle);
        assert_eq!(pss.case_num(), 1);
        pss.set_curve_style(CurveStyle);
        assert_eq!(pss.case_num(), 2);
    }
}
