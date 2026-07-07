// FILE: step_visual_surface_style_rendering.rs
// occt: StepVisual_SurfaceStyleRendering

use std::sync::Arc;

pub struct ShadingSurfaceMethod;
pub struct Colour;

pub struct SurfaceStyleRendering {
    rendering_method: u32,
    surface_colour: Option<Arc<Colour>>,
}

impl SurfaceStyleRendering {
    pub fn new() -> Self {
        SurfaceStyleRendering {
            rendering_method: 0,
            surface_colour: None,
        }
    }

    pub fn init(&mut self, rendering_method: u32, surface_colour: Option<Arc<Colour>>) {
        self.rendering_method = rendering_method;
        self.surface_colour = surface_colour;
    }

    pub fn rendering_method(&self) -> u32 {
        self.rendering_method
    }

    pub fn set_rendering_method(&mut self, method: u32) {
        self.rendering_method = method;
    }

    pub fn surface_colour(&self) -> Option<&Arc<Colour>> {
        self.surface_colour.as_ref()
    }

    pub fn set_surface_colour(&mut self, colour: Option<Arc<Colour>>) {
        self.surface_colour = colour;
    }
}

impl Default for SurfaceStyleRendering {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ssr = SurfaceStyleRendering::new();
        assert_eq!(ssr.rendering_method(), 0);
        assert!(ssr.surface_colour().is_none());
    }

    #[test]
    fn test_init() {
        let mut ssr = SurfaceStyleRendering::new();
        let colour = Arc::new(Colour);
        ssr.init(1, Some(colour));
        assert_eq!(ssr.rendering_method(), 1);
        assert!(ssr.surface_colour().is_some());
    }

    #[test]
    fn test_set_rendering_method() {
        let mut ssr = SurfaceStyleRendering::new();
        ssr.set_rendering_method(3);
        assert_eq!(ssr.rendering_method(), 3);
    }

    #[test]
    fn test_set_surface_colour() {
        let mut ssr = SurfaceStyleRendering::new();
        let colour = Arc::new(Colour);
        ssr.set_surface_colour(Some(colour.clone()));
        assert!(ssr.surface_colour().is_some());
    }
}
