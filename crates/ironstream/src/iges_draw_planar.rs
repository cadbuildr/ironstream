// FILE: iges_draw_planar.rs
// occt: IGESDraw_Planar

/// Planar entity
pub struct IgesDrawPlanar {
    plane: (f64, f64, f64, f64),
}

impl IgesDrawPlanar {
    pub fn new() -> Self {
        IgesDrawPlanar {
            plane: (0.0, 0.0, 0.0, 1.0),
        }
    }

    pub fn init(&mut self, a: f64, b: f64, c: f64, d: f64) {
        self.plane = (a, b, c, d);
    }

    pub fn plane(&self) -> (f64, f64, f64, f64) {
        self.plane
    }
}

impl Default for IgesDrawPlanar {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let planar = IgesDrawPlanar::new();
        assert_eq!(planar.plane(), (0.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn test_init() {
        let mut planar = IgesDrawPlanar::new();
        planar.init(1.0, 2.0, 3.0, 4.0);
        assert_eq!(planar.plane(), (1.0, 2.0, 3.0, 4.0));
    }
}
