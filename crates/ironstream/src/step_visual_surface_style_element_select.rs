// FILE: step_visual_surface_style_element_select.rs
// occt: StepVisual_SurfaceStyleElementSelect

pub struct SurfaceStyleFillArea;
pub struct SurfaceStyleBoundary;
pub struct SurfaceStyleParameterLine;
pub struct SurfaceStyleSilhouette;
pub struct SurfaceStyleSegmentationCurve;
pub struct SurfaceStyleControlGrid;
pub struct SurfaceStyleRendering;

pub struct SurfaceStyleElementSelect {
    case: Option<SelectCase>,
    value: Option<Box<dyn std::any::Any>>,
}

#[derive(Clone, Copy)]
enum SelectCase {
    SurfaceStyleFillArea = 1,
    SurfaceStyleBoundary = 2,
    SurfaceStyleParameterLine = 3,
    SurfaceStyleSilhouette = 4,
    SurfaceStyleSegmentationCurve = 5,
    SurfaceStyleControlGrid = 6,
    SurfaceStyleRendering = 7,
}

impl SurfaceStyleElementSelect {
    pub fn new() -> Self {
        SurfaceStyleElementSelect {
            case: None,
            value: None,
        }
    }

    pub fn case_num(&self) -> i32 {
        match self.case {
            Some(SelectCase::SurfaceStyleFillArea) => 1,
            Some(SelectCase::SurfaceStyleBoundary) => 2,
            Some(SelectCase::SurfaceStyleParameterLine) => 3,
            Some(SelectCase::SurfaceStyleSilhouette) => 4,
            Some(SelectCase::SurfaceStyleSegmentationCurve) => 5,
            Some(SelectCase::SurfaceStyleControlGrid) => 6,
            Some(SelectCase::SurfaceStyleRendering) => 7,
            None => 0,
        }
    }

    pub fn surface_style_fill_area(&self) -> Option<&SurfaceStyleFillArea> {
        if matches!(self.case, Some(SelectCase::SurfaceStyleFillArea)) {
            self.value
                .as_ref()
                .and_then(|v| v.downcast_ref::<SurfaceStyleFillArea>())
        } else {
            None
        }
    }

    pub fn surface_style_boundary(&self) -> Option<&SurfaceStyleBoundary> {
        if matches!(self.case, Some(SelectCase::SurfaceStyleBoundary)) {
            self.value
                .as_ref()
                .and_then(|v| v.downcast_ref::<SurfaceStyleBoundary>())
        } else {
            None
        }
    }

    pub fn surface_style_parameter_line(&self) -> Option<&SurfaceStyleParameterLine> {
        if matches!(self.case, Some(SelectCase::SurfaceStyleParameterLine)) {
            self.value
                .as_ref()
                .and_then(|v| v.downcast_ref::<SurfaceStyleParameterLine>())
        } else {
            None
        }
    }

    pub fn surface_style_rendering(&self) -> Option<&SurfaceStyleRendering> {
        if matches!(self.case, Some(SelectCase::SurfaceStyleRendering)) {
            self.value
                .as_ref()
                .and_then(|v| v.downcast_ref::<SurfaceStyleRendering>())
        } else {
            None
        }
    }

    pub fn set_surface_style_fill_area(&mut self, area: SurfaceStyleFillArea) {
        self.case = Some(SelectCase::SurfaceStyleFillArea);
        self.value = Some(Box::new(area));
    }

    pub fn set_surface_style_boundary(&mut self, boundary: SurfaceStyleBoundary) {
        self.case = Some(SelectCase::SurfaceStyleBoundary);
        self.value = Some(Box::new(boundary));
    }

    pub fn set_surface_style_parameter_line(&mut self, line: SurfaceStyleParameterLine) {
        self.case = Some(SelectCase::SurfaceStyleParameterLine);
        self.value = Some(Box::new(line));
    }

    pub fn set_surface_style_rendering(&mut self, rendering: SurfaceStyleRendering) {
        self.case = Some(SelectCase::SurfaceStyleRendering);
        self.value = Some(Box::new(rendering));
    }
}

impl Default for SurfaceStyleElementSelect {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sses = SurfaceStyleElementSelect::new();
        assert_eq!(sses.case_num(), 0);
    }

    #[test]
    fn test_set_fill_area() {
        let mut sses = SurfaceStyleElementSelect::new();
        sses.set_surface_style_fill_area(SurfaceStyleFillArea);
        assert_eq!(sses.case_num(), 1);
        assert!(sses.surface_style_fill_area().is_some());
    }

    #[test]
    fn test_set_boundary() {
        let mut sses = SurfaceStyleElementSelect::new();
        sses.set_surface_style_boundary(SurfaceStyleBoundary);
        assert_eq!(sses.case_num(), 2);
        assert!(sses.surface_style_boundary().is_some());
    }

    #[test]
    fn test_set_parameter_line() {
        let mut sses = SurfaceStyleElementSelect::new();
        sses.set_surface_style_parameter_line(SurfaceStyleParameterLine);
        assert_eq!(sses.case_num(), 3);
        assert!(sses.surface_style_parameter_line().is_some());
    }

    #[test]
    fn test_set_rendering() {
        let mut sses = SurfaceStyleElementSelect::new();
        sses.set_surface_style_rendering(SurfaceStyleRendering);
        assert_eq!(sses.case_num(), 7);
        assert!(sses.surface_style_rendering().is_some());
    }
}
