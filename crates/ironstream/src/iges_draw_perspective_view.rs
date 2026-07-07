// FILE: iges_draw_perspective_view.rs
// occt: IGESDraw_PerspectiveView

/// Perspective view entity
pub struct IgesDrawPerspectiveView {
    view_point: (f64, f64, f64),
    center: (f64, f64, f64),
}

impl IgesDrawPerspectiveView {
    pub fn new() -> Self {
        IgesDrawPerspectiveView {
            view_point: (0.0, 0.0, 0.0),
            center: (0.0, 0.0, 0.0),
        }
    }

    pub fn init(&mut self, view_point: (f64, f64, f64), center: (f64, f64, f64)) {
        self.view_point = view_point;
        self.center = center;
    }

    pub fn view_point(&self) -> (f64, f64, f64) {
        self.view_point
    }

    pub fn center(&self) -> (f64, f64, f64) {
        self.center
    }
}

impl Default for IgesDrawPerspectiveView {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pv = IgesDrawPerspectiveView::new();
        assert_eq!(pv.view_point(), (0.0, 0.0, 0.0));
    }

    #[test]
    fn test_init() {
        let mut pv = IgesDrawPerspectiveView::new();
        pv.init((1.0, 2.0, 3.0), (4.0, 5.0, 6.0));
        assert_eq!(pv.view_point(), (1.0, 2.0, 3.0));
        assert_eq!(pv.center(), (4.0, 5.0, 6.0));
    }
}
