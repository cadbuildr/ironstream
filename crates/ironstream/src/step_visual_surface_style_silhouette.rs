// FILE: step_visual_surface_style_silhouette.rs
// occt: StepVisual_SurfaceStyleSilhouette

use std::sync::Arc;

pub struct CurveStyle;

pub struct SurfaceStyleSilhouette {
    style_of_silhouette: Option<Arc<CurveStyle>>,
}

impl SurfaceStyleSilhouette {
    pub fn new() -> Self {
        SurfaceStyleSilhouette {
            style_of_silhouette: None,
        }
    }

    pub fn init(&mut self, style: Option<Arc<CurveStyle>>) {
        self.style_of_silhouette = style;
    }

    pub fn set_style_of_silhouette(&mut self, style: Option<Arc<CurveStyle>>) {
        self.style_of_silhouette = style;
    }

    pub fn style_of_silhouette(&self) -> Option<&Arc<CurveStyle>> {
        self.style_of_silhouette.as_ref()
    }
}

impl Default for SurfaceStyleSilhouette {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sss = SurfaceStyleSilhouette::new();
        assert!(sss.style_of_silhouette().is_none());
    }

    #[test]
    fn test_set_and_get_style() {
        let mut sss = SurfaceStyleSilhouette::new();
        let style = Arc::new(CurveStyle);
        sss.set_style_of_silhouette(Some(style.clone()));
        assert!(sss.style_of_silhouette().is_some());
    }

    #[test]
    fn test_init() {
        let mut sss = SurfaceStyleSilhouette::new();
        let style = Arc::new(CurveStyle);
        sss.init(Some(style));
        assert!(sss.style_of_silhouette().is_some());
    }
}
