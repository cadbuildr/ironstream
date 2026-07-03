// FILE: gce_make_elips.rs
// occt: gce_MakeElips

//! Construction algorithms for ellipses.
pub struct GceMakeElips {
    ellipse: Option<(f64, f64, f64, f64)>, // center_x, center_y, major, minor
    done: bool,
    error_status: i32,
}

impl GceMakeElips {
    const STATUS_OK: i32 = 0;
    const STATUS_NEGATIVE_RADIUS: i32 = 1;
    const STATUS_CONFUSED_POINTS: i32 = 2;
    const STATUS_MAJOR_LESS_MINOR: i32 = 3;

    pub fn new() -> Self {
        Self {
            ellipse: None,
            done: false,
            error_status: Self::STATUS_OK,
        }
    }

    pub fn from_center_and_radii(cx: f64, cy: f64, major: f64, minor: f64) -> Self {
        let mut maker = Self::new();

        if major < 0.0 || minor < 0.0 {
            maker.error_status = Self::STATUS_NEGATIVE_RADIUS;
            return maker;
        }

        if major < minor {
            maker.error_status = Self::STATUS_MAJOR_LESS_MINOR;
            return maker;
        }

        maker.ellipse = Some((cx, cy, major, minor));
        maker.done = true;
        maker
    }

    pub fn from_axis_and_radii(
        cx: f64,
        cy: f64,
        major_axis_x: f64,
        major_axis_y: f64,
        major: f64,
        minor: f64,
    ) -> Self {
        let mut maker = Self::new();

        if major < 0.0 || minor < 0.0 {
            maker.error_status = Self::STATUS_NEGATIVE_RADIUS;
            return maker;
        }

        if major < minor {
            maker.error_status = Self::STATUS_MAJOR_LESS_MINOR;
            return maker;
        }

        let axis_len_sq = major_axis_x * major_axis_x + major_axis_y * major_axis_y;
        if axis_len_sq < 1e-10 {
            maker.error_status = Self::STATUS_CONFUSED_POINTS;
            return maker;
        }

        maker.ellipse = Some((cx, cy, major, minor));
        maker.done = true;
        maker
    }

    pub fn from_two_foci_and_major(
        f1: (f64, f64),
        f2: (f64, f64),
        major: f64,
    ) -> Self {
        let mut maker = Self::new();

        if major < 0.0 {
            maker.error_status = Self::STATUS_NEGATIVE_RADIUS;
            return maker;
        }

        let dist_sq = (f2.0 - f1.0).powi(2) + (f2.1 - f1.1).powi(2);
        if dist_sq < 1e-10 {
            maker.error_status = Self::STATUS_CONFUSED_POINTS;
            return maker;
        }

        let cx = (f1.0 + f2.0) / 2.0;
        let cy = (f1.1 + f2.1) / 2.0;

        let c = dist_sq.sqrt() / 2.0;
        let minor = (major * major - c * c).sqrt();

        if minor < 0.0 {
            maker.error_status = Self::STATUS_MAJOR_LESS_MINOR;
            return maker;
        }

        maker.ellipse = Some((cx, cy, major, minor));
        maker.done = true;
        maker
    }

    pub fn from_three_points(
        p1: (f64, f64),
        p2: (f64, f64),
        p3: (f64, f64),
    ) -> Self {
        let mut maker = Self::new();

        let dx12 = p2.0 - p1.0;
        let dy12 = p2.1 - p1.1;
        let dx13 = p3.0 - p1.0;
        let dy13 = p3.1 - p1.1;

        let det = dx12 * dy13 - dx13 * dy12;

        if det.abs() < 1e-10 {
            maker.error_status = Self::STATUS_CONFUSED_POINTS;
            return maker;
        }

        let cx = p1.0 + (dx12 + dx13) / 2.0;
        let cy = p1.1 + (dy12 + dy13) / 2.0;

        let major = ((p2.0 - cx).powi(2) + (p2.1 - cy).powi(2)).sqrt();
        let minor = ((p3.0 - cx).powi(2) + (p3.1 - cy).powi(2)).sqrt();

        maker.ellipse = Some((cx, cy, major, minor));
        maker.done = true;
        maker
    }

    pub fn value(&self) -> Option<(f64, f64, f64, f64)> {
        if self.done {
            self.ellipse
        } else {
            None
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn error_status(&self) -> i32 {
        self.error_status
    }
}

impl Default for GceMakeElips {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_center_and_radii() {
        let maker = GceMakeElips::from_center_and_radii(0.0, 0.0, 5.0, 3.0);
        assert!(maker.is_done());
        let elips = maker.value().unwrap();
        assert_eq!(elips.0, 0.0);
        assert_eq!(elips.2, 5.0);
        assert_eq!(elips.3, 3.0);
    }

    #[test]
    fn test_negative_radius() {
        let maker = GceMakeElips::from_center_and_radii(0.0, 0.0, 5.0, -1.0);
        assert!(!maker.is_done());
    }

    #[test]
    fn test_major_less_minor() {
        let maker = GceMakeElips::from_center_and_radii(0.0, 0.0, 2.0, 5.0);
        assert!(!maker.is_done());
    }

    #[test]
    fn test_from_two_foci() {
        let maker = GceMakeElips::from_two_foci_and_major((1.0, 0.0), (-1.0, 0.0), 5.0);
        assert!(maker.is_done());
    }

    #[test]
    fn test_from_three_points() {
        let maker = GceMakeElips::from_three_points((0.0, 0.0), (5.0, 0.0), (0.0, 3.0));
        assert!(maker.is_done());
    }

    #[test]
    fn test_from_axis_and_radii() {
        let maker = GceMakeElips::from_axis_and_radii(0.0, 0.0, 1.0, 0.0, 5.0, 3.0);
        assert!(maker.is_done());
    }
}
