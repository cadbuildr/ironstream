// FILE: iges_draw_view.rs
// occt: IGESDraw_View

/// View entity
pub struct IgesDrawView {
    origin: (f64, f64, f64),
    normal: (f64, f64, f64),
}

impl IgesDrawView {
    pub fn new() -> Self {
        IgesDrawView {
            origin: (0.0, 0.0, 0.0),
            normal: (0.0, 0.0, 1.0),
        }
    }

    pub fn init(&mut self, origin: (f64, f64, f64), normal: (f64, f64, f64)) {
        self.origin = origin;
        self.normal = normal;
    }

    pub fn origin(&self) -> (f64, f64, f64) {
        self.origin
    }

    pub fn normal(&self) -> (f64, f64, f64) {
        self.normal
    }
}

impl Default for IgesDrawView {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let view = IgesDrawView::new();
        assert_eq!(view.origin(), (0.0, 0.0, 0.0));
        assert_eq!(view.normal(), (0.0, 0.0, 1.0));
    }

    #[test]
    fn test_init() {
        let mut view = IgesDrawView::new();
        view.init((1.0, 2.0, 3.0), (0.0, 1.0, 0.0));
        assert_eq!(view.origin(), (1.0, 2.0, 3.0));
        assert_eq!(view.normal(), (0.0, 1.0, 0.0));
    }
}
