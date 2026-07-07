// FILE: step_visual_surface_style_segmentation_curve.rs
// occt: StepVisual_SurfaceStyleSegmentationCurve

use std::sync::Arc;

pub struct CurveStyle;

pub struct SurfaceStyleSegmentationCurve {
    style_of_segmentation_curve: Option<Arc<CurveStyle>>,
}

impl SurfaceStyleSegmentationCurve {
    pub fn new() -> Self {
        SurfaceStyleSegmentationCurve {
            style_of_segmentation_curve: None,
        }
    }

    pub fn init(&mut self, style: Option<Arc<CurveStyle>>) {
        self.style_of_segmentation_curve = style;
    }

    pub fn set_style_of_segmentation_curve(&mut self, style: Option<Arc<CurveStyle>>) {
        self.style_of_segmentation_curve = style;
    }

    pub fn style_of_segmentation_curve(&self) -> Option<&Arc<CurveStyle>> {
        self.style_of_segmentation_curve.as_ref()
    }
}

impl Default for SurfaceStyleSegmentationCurve {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sssc = SurfaceStyleSegmentationCurve::new();
        assert!(sssc.style_of_segmentation_curve().is_none());
    }

    #[test]
    fn test_set_and_get_style() {
        let mut sssc = SurfaceStyleSegmentationCurve::new();
        let style = Arc::new(CurveStyle);
        sssc.set_style_of_segmentation_curve(Some(style.clone()));
        assert!(sssc.style_of_segmentation_curve().is_some());
    }

    #[test]
    fn test_init() {
        let mut sssc = SurfaceStyleSegmentationCurve::new();
        let style = Arc::new(CurveStyle);
        sssc.init(Some(style));
        assert!(sssc.style_of_segmentation_curve().is_some());
    }
}
