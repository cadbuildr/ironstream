// FILE: step_visual_surface_style_rendering_with_properties.rs
// occt: StepVisual_SurfaceStyleRenderingWithProperties

use std::sync::Arc;

pub struct Colour;
pub struct RenderingPropertiesSelect;

pub struct SurfaceStyleRenderingWithProperties {
    rendering_method: u32,
    surface_colour: Option<Arc<Colour>>,
    properties: Option<Arc<Vec<RenderingPropertiesSelect>>>,
}

impl SurfaceStyleRenderingWithProperties {
    pub fn new() -> Self {
        SurfaceStyleRenderingWithProperties {
            rendering_method: 0,
            surface_colour: None,
            properties: None,
        }
    }

    pub fn init(
        &mut self,
        rendering_method: u32,
        surface_colour: Option<Arc<Colour>>,
        properties: Option<Arc<Vec<RenderingPropertiesSelect>>>,
    ) {
        self.rendering_method = rendering_method;
        self.surface_colour = surface_colour;
        self.properties = properties;
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

    pub fn properties(&self) -> Option<&Arc<Vec<RenderingPropertiesSelect>>> {
        self.properties.as_ref()
    }

    pub fn set_properties(&mut self, props: Option<Arc<Vec<RenderingPropertiesSelect>>>) {
        self.properties = props;
    }
}

impl Default for SurfaceStyleRenderingWithProperties {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ssrwp = SurfaceStyleRenderingWithProperties::new();
        assert_eq!(ssrwp.rendering_method(), 0);
        assert!(ssrwp.surface_colour().is_none());
        assert!(ssrwp.properties().is_none());
    }

    #[test]
    fn test_init() {
        let mut ssrwp = SurfaceStyleRenderingWithProperties::new();
        let colour = Arc::new(Colour);
        let props = vec![RenderingPropertiesSelect::new()];
        ssrwp.init(1, Some(colour), Some(Arc::new(props)));
        assert_eq!(ssrwp.rendering_method(), 1);
        assert!(ssrwp.surface_colour().is_some());
        assert!(ssrwp.properties().is_some());
    }

    #[test]
    fn test_set_properties() {
        let mut ssrwp = SurfaceStyleRenderingWithProperties::new();
        let props = vec![RenderingPropertiesSelect::new()];
        ssrwp.set_properties(Some(Arc::new(props)));
        assert!(ssrwp.properties().is_some());
    }
}

impl RenderingPropertiesSelect {
    fn new() -> Self {
        RenderingPropertiesSelect
    }
}
