// FILE: step_visual_surface_style_transparent.rs
// occt: StepVisual_SurfaceStyleTransparent

pub struct SurfaceStyleTransparent {
    transparency: f64,
}

impl SurfaceStyleTransparent {
    pub fn new() -> Self {
        SurfaceStyleTransparent {
            transparency: 0.0,
        }
    }

    pub fn init(&mut self, transparency: f64) {
        self.transparency = transparency;
    }

    pub fn transparency(&self) -> f64 {
        self.transparency
    }

    pub fn set_transparency(&mut self, value: f64) {
        self.transparency = value;
    }
}

impl Default for SurfaceStyleTransparent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sst = SurfaceStyleTransparent::new();
        assert_eq!(sst.transparency(), 0.0);
    }

    #[test]
    fn test_set_and_get_transparency() {
        let mut sst = SurfaceStyleTransparent::new();
        sst.set_transparency(0.5);
        assert_eq!(sst.transparency(), 0.5);
    }

    #[test]
    fn test_init() {
        let mut sst = SurfaceStyleTransparent::new();
        sst.init(0.75);
        assert_eq!(sst.transparency(), 0.75);
    }

    #[test]
    fn test_transparency_range() {
        let mut sst = SurfaceStyleTransparent::new();
        sst.set_transparency(1.0);
        assert_eq!(sst.transparency(), 1.0);

        sst.set_transparency(0.0);
        assert_eq!(sst.transparency(), 0.0);
    }
}
