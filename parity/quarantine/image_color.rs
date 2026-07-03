// FILE: image_color.rs
// occt: Image_Color

//! RGB color (3 bytes).
#[derive(Clone, Copy, Debug, Default)]
pub struct ColorRgb {
    pub v: [u8; 3],
}

impl ColorRgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { v: [r, g, b] }
    }

    pub fn r(&self) -> u8 { self.v[0] }
    pub fn g(&self) -> u8 { self.v[1] }
    pub fn b(&self) -> u8 { self.v[2] }

    pub fn len() -> usize { 3 }
}

//! RGBA color (4 bytes).
#[derive(Clone, Copy, Debug, Default)]
pub struct ColorRgba {
    pub v: [u8; 4],
}

impl ColorRgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { v: [r, g, b, a] }
    }

    pub fn r(&self) -> u8 { self.v[0] }
    pub fn g(&self) -> u8 { self.v[1] }
    pub fn b(&self) -> u8 { self.v[2] }
    pub fn a(&self) -> u8 { self.v[3] }

    pub fn len() -> usize { 4 }
}

//! Float RGB color (3 floats).
#[derive(Clone, Copy, Debug, Default)]
pub struct ColorRgbf {
    pub v: [f32; 3],
}

impl ColorRgbf {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { v: [r, g, b] }
    }

    pub fn r(&self) -> f32 { self.v[0] }
    pub fn g(&self) -> f32 { self.v[1] }
    pub fn b(&self) -> f32 { self.v[2] }

    pub fn len() -> usize { 3 }
}

//! Float RGBA color (4 floats).
#[derive(Clone, Copy, Debug, Default)]
pub struct ColorRgbaf {
    pub v: [f32; 4],
}

impl ColorRgbaf {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { v: [r, g, b, a] }
    }

    pub fn r(&self) -> f32 { self.v[0] }
    pub fn g(&self) -> f32 { self.v[1] }
    pub fn b(&self) -> f32 { self.v[2] }
    pub fn a(&self) -> f32 { self.v[3] }

    pub fn len() -> usize { 4 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_rgb() {
        let c = ColorRgb::new(255, 128, 64);
        assert_eq!(c.r(), 255);
        assert_eq!(ColorRgb::len(), 3);
    }

    #[test]
    fn test_color_rgba() {
        let c = ColorRgba::new(100, 150, 200, 255);
        assert_eq!(c.r(), 100);
        assert_eq!(c.a(), 255);
        assert_eq!(ColorRgba::len(), 4);
    }
}
