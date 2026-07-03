// FILE: step_construct_rendering_properties.rs
// occt: STEPConstruct_RenderingProperties

//! STEP rendering properties for visual presentation.

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            r: r.clamp(0.0, 1.0),
            g: g.clamp(0.0, 1.0),
            b: b.clamp(0.0, 1.0),
        }
    }

    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn red() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }
}

#[derive(Clone, Debug)]
pub struct StepConstructRenderingProperties {
    surface_color: Color,
    transparency: f32,
    shininess: f32,
    is_set: bool,
}

impl StepConstructRenderingProperties {
    pub fn new() -> Self {
        Self {
            surface_color: Color::white(),
            transparency: 0.0,
            shininess: 0.5,
            is_set: false,
        }
    }

    pub fn set_surface_color(&mut self, color: Color) {
        self.surface_color = color;
        self.is_set = true;
    }

    pub fn surface_color(&self) -> Color {
        self.surface_color
    }

    pub fn set_transparency(&mut self, trans: f32) {
        self.transparency = trans.clamp(0.0, 1.0);
        self.is_set = true;
    }

    pub fn transparency(&self) -> f32 {
        self.transparency
    }

    pub fn set_shininess(&mut self, shine: f32) {
        self.shininess = shine.clamp(0.0, 1.0);
    }

    pub fn shininess(&self) -> f32 {
        self.shininess
    }

    pub fn is_set(&self) -> bool {
        self.is_set
    }

    pub fn reset(&mut self) {
        self.surface_color = Color::white();
        self.transparency = 0.0;
        self.shininess = 0.5;
        self.is_set = false;
    }
}

impl Default for StepConstructRenderingProperties {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_construction() {
        let c = Color::new(0.5, 0.5, 0.5);
        assert!((c.r - 0.5).abs() < 1e-6);
    }

    #[test]
    fn constructor() {
        let p = StepConstructRenderingProperties::new();
        assert!(!p.is_set());
    }

    #[test]
    fn set_color() {
        let mut p = StepConstructRenderingProperties::new();
        p.set_surface_color(Color::red());
        assert!(p.is_set());
    }
}
