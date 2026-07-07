// FILE: step_visual_rendering_properties_select.rs
// occt: StepVisual_RenderingPropertiesSelect

pub struct SurfaceStyleReflectanceAmbient;
pub struct SurfaceStyleTransparent;

pub struct RenderingPropertiesSelect {
    case: Option<SelectCase>,
    value: Option<Box<dyn std::any::Any>>,
}

#[derive(Clone, Copy)]
enum SelectCase {
    SurfaceStyleReflectanceAmbient = 1,
    SurfaceStyleTransparent = 2,
}

impl RenderingPropertiesSelect {
    pub fn new() -> Self {
        RenderingPropertiesSelect {
            case: None,
            value: None,
        }
    }

    pub fn case_num(&self) -> i32 {
        match self.case {
            Some(SelectCase::SurfaceStyleReflectanceAmbient) => 1,
            Some(SelectCase::SurfaceStyleTransparent) => 2,
            None => 0,
        }
    }

    pub fn surface_style_reflectance_ambient(&self) -> Option<&SurfaceStyleReflectanceAmbient> {
        if matches!(self.case, Some(SelectCase::SurfaceStyleReflectanceAmbient)) {
            self.value
                .as_ref()
                .and_then(|v| v.downcast_ref::<SurfaceStyleReflectanceAmbient>())
        } else {
            None
        }
    }

    pub fn surface_style_transparent(&self) -> Option<&SurfaceStyleTransparent> {
        if matches!(self.case, Some(SelectCase::SurfaceStyleTransparent)) {
            self.value
                .as_ref()
                .and_then(|v| v.downcast_ref::<SurfaceStyleTransparent>())
        } else {
            None
        }
    }

    pub fn set_surface_style_reflectance_ambient(&mut self, style: SurfaceStyleReflectanceAmbient) {
        self.case = Some(SelectCase::SurfaceStyleReflectanceAmbient);
        self.value = Some(Box::new(style));
    }

    pub fn set_surface_style_transparent(&mut self, style: SurfaceStyleTransparent) {
        self.case = Some(SelectCase::SurfaceStyleTransparent);
        self.value = Some(Box::new(style));
    }
}

impl Default for RenderingPropertiesSelect {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let rps = RenderingPropertiesSelect::new();
        assert_eq!(rps.case_num(), 0);
        assert!(rps.surface_style_reflectance_ambient().is_none());
        assert!(rps.surface_style_transparent().is_none());
    }

    #[test]
    fn test_set_reflectance_ambient() {
        let mut rps = RenderingPropertiesSelect::new();
        rps.set_surface_style_reflectance_ambient(SurfaceStyleReflectanceAmbient);
        assert_eq!(rps.case_num(), 1);
        assert!(rps.surface_style_reflectance_ambient().is_some());
        assert!(rps.surface_style_transparent().is_none());
    }

    #[test]
    fn test_set_transparent() {
        let mut rps = RenderingPropertiesSelect::new();
        rps.set_surface_style_transparent(SurfaceStyleTransparent);
        assert_eq!(rps.case_num(), 2);
        assert!(rps.surface_style_reflectance_ambient().is_none());
        assert!(rps.surface_style_transparent().is_some());
    }

    #[test]
    fn test_switch_selection() {
        let mut rps = RenderingPropertiesSelect::new();
        rps.set_surface_style_reflectance_ambient(SurfaceStyleReflectanceAmbient);
        assert_eq!(rps.case_num(), 1);
        rps.set_surface_style_transparent(SurfaceStyleTransparent);
        assert_eq!(rps.case_num(), 2);
        assert!(rps.surface_style_reflectance_ambient().is_none());
    }
}
