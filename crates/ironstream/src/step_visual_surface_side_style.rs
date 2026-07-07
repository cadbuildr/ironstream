// FILE: step_visual_surface_side_style.rs
// occt: StepVisual_SurfaceSideStyle

use std::sync::Arc;

pub struct HasciiString;
pub struct SurfaceStyleElementSelect;

impl SurfaceStyleElementSelect {
    pub fn new() -> Self {
        SurfaceStyleElementSelect
    }
}

impl Default for SurfaceStyleElementSelect {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SurfaceSideStyle {
    name: Option<Arc<HasciiString>>,
    styles: Option<Arc<Vec<SurfaceStyleElementSelect>>>,
}

impl SurfaceSideStyle {
    pub fn new() -> Self {
        SurfaceSideStyle {
            name: None,
            styles: None,
        }
    }

    pub fn init(
        &mut self,
        name: Option<Arc<HasciiString>>,
        styles: Option<Arc<Vec<SurfaceStyleElementSelect>>>,
    ) {
        self.name = name;
        self.styles = styles;
    }

    pub fn set_name(&mut self, name: Option<Arc<HasciiString>>) {
        self.name = name;
    }

    pub fn name(&self) -> Option<&Arc<HasciiString>> {
        self.name.as_ref()
    }

    pub fn set_styles(&mut self, styles: Option<Arc<Vec<SurfaceStyleElementSelect>>>) {
        self.styles = styles;
    }

    pub fn styles(&self) -> Option<&Arc<Vec<SurfaceStyleElementSelect>>> {
        self.styles.as_ref()
    }

    pub fn styles_value(&self, num: usize) -> Option<&SurfaceStyleElementSelect> {
        self.styles.as_ref().and_then(|s| s.get(num))
    }

    pub fn nb_styles(&self) -> usize {
        self.styles.as_ref().map(|s| s.len()).unwrap_or(0)
    }
}

impl Default for SurfaceSideStyle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sss = SurfaceSideStyle::new();
        assert!(sss.name().is_none());
        assert_eq!(sss.nb_styles(), 0);
    }

    #[test]
    fn test_set_and_get_name() {
        let mut sss = SurfaceSideStyle::new();
        let name = Arc::new(HasciiString);
        sss.set_name(Some(name));
        assert!(sss.name().is_some());
    }

    #[test]
    fn test_set_and_get_styles() {
        let mut sss = SurfaceSideStyle::new();
        let styles = vec![SurfaceStyleElementSelect::new()];
        sss.set_styles(Some(Arc::new(styles)));
        assert_eq!(sss.nb_styles(), 1);
    }

    #[test]
    fn test_init() {
        let mut sss = SurfaceSideStyle::new();
        let name = Arc::new(HasciiString);
        let styles = vec![SurfaceStyleElementSelect::new()];
        sss.init(Some(name), Some(Arc::new(styles)));

        assert!(sss.name().is_some());
        assert_eq!(sss.nb_styles(), 1);
    }
}
