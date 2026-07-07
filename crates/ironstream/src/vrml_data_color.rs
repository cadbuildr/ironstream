// FILE: vrml_data_color.rs
// occt: VrmlData_Color

#[derive(Clone, Debug)]
pub struct VrmlDataColor {
    r: f32,
    g: f32,
    b: f32,
}

impl VrmlDataColor {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        VrmlDataColor { r, g, b }
    }

    pub fn r(&self) -> f32 {
        self.r
    }

    pub fn g(&self) -> f32 {
        self.g
    }

    pub fn b(&self) -> f32 {
        self.b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let c = VrmlDataColor::new(1.0, 0.5, 0.0);
        assert_eq!(c.r(), 1.0);
        assert_eq!(c.g(), 0.5);
        assert_eq!(c.b(), 0.0);
    }
}
