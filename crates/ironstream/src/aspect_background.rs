// FILE: aspect_background.rs
// occt: Aspect_Background, Aspect_GradientBackground, Aspect_GradientFillMethod

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GradientFillMethod {
    None,
    Horizontal,
    Vertical,
    Diagonal1,
    Diagonal2,
    Corner1,
    Corner2,
    Corner3,
    Corner4,
    Elliptical,
}

/// Solid-color background.
#[derive(Clone, Debug)]
pub struct Background {
    pub color: [f32; 3],
}

impl Background {
    pub const BLACK: Self = Self { color: [0.0, 0.0, 0.0] };
    pub const WHITE: Self = Self { color: [1.0, 1.0, 1.0] };
    pub const DARK_GREY: Self = Self { color: [0.2, 0.2, 0.2] };

    pub fn new(r: f32, g: f32, b: f32) -> Self { Self { color: [r, g, b] } }
    pub fn brightness(&self) -> f32 { 0.2126*self.color[0] + 0.7152*self.color[1] + 0.0722*self.color[2] }
    pub fn is_dark(&self) -> bool { self.brightness() < 0.5 }
}

impl Default for Background {
    fn default() -> Self { Self::DARK_GREY }
}

/// Gradient background between two colors.
#[derive(Clone, Debug)]
pub struct GradientBackground {
    pub color1: [f32; 3],
    pub color2: [f32; 3],
    pub method: GradientFillMethod,
}

impl GradientBackground {
    pub fn new(c1: [f32; 3], c2: [f32; 3], method: GradientFillMethod) -> Self {
        Self { color1: c1, color2: c2, method }
    }

    pub fn vertical(top: [f32; 3], bottom: [f32; 3]) -> Self {
        Self::new(top, bottom, GradientFillMethod::Vertical)
    }

    pub fn horizontal(left: [f32; 3], right: [f32; 3]) -> Self {
        Self::new(left, right, GradientFillMethod::Horizontal)
    }

    pub fn color_at(&self, t: f32) -> [f32; 3] {
        let t = t.clamp(0.0, 1.0);
        [
            self.color1[0] + (self.color2[0] - self.color1[0]) * t,
            self.color1[1] + (self.color2[1] - self.color1[1]) * t,
            self.color1[2] + (self.color2[2] - self.color1[2]) * t,
        ]
    }

    pub fn is_gradient(&self) -> bool { self.method != GradientFillMethod::None }
}

impl Default for GradientBackground {
    fn default() -> Self { Self::new([0.1,0.1,0.15],[0.2,0.2,0.3], GradientFillMethod::Vertical) }
}

/// Combined background that can be either solid or gradient.
#[derive(Clone, Debug)]
pub enum AnyBackground {
    Solid(Background),
    Gradient(GradientBackground),
}

impl AnyBackground {
    pub fn is_gradient(&self) -> bool { matches!(self, Self::Gradient(_)) }

    pub fn solid_color(&self) -> [f32; 3] {
        match self {
            Self::Solid(b) => b.color,
            Self::Gradient(g) => g.color_at(0.5),
        }
    }
}

impl Default for AnyBackground {
    fn default() -> Self { Self::Solid(Background::DARK_GREY) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn background_brightness() {
        let b = Background::new(0.0, 0.0, 0.0);
        assert!(b.is_dark());
        let w = Background::WHITE;
        assert!(!w.is_dark());
    }

    #[test]
    fn gradient_color_at() {
        let g = GradientBackground::vertical([0.0,0.0,0.0],[1.0,1.0,1.0]);
        let mid = g.color_at(0.5);
        assert!((mid[0] - 0.5).abs() < 1e-6);
    }

    #[test]
    fn gradient_endpoints() {
        let g = GradientBackground::vertical([1.0,0.0,0.0],[0.0,0.0,1.0]);
        let start = g.color_at(0.0);
        let end = g.color_at(1.0);
        assert!((start[0] - 1.0).abs() < 1e-6);
        assert!((end[2] - 1.0).abs() < 1e-6);
    }

    #[test]
    fn any_background_solid() {
        let bg = AnyBackground::Solid(Background::new(0.3, 0.3, 0.3));
        assert!(!bg.is_gradient());
    }
}
