// FILE: gce_make_hypr.rs
// occt: gce_MakeHypr

//! Construction algorithms for hyperbolas.
pub struct GceMakeHypr {
    hyperbola: Option<(f64, f64, f64, f64)>, // center_x, center_y, major, minor
    done: bool,
    error_status: i32,
}

impl GceMakeHypr {
    const STATUS_OK: i32 = 0;
    const STATUS_NEGATIVE_RADIUS: i32 = 1;
    const STATUS_CONFUSED_POINTS: i32 = 2;
    const STATUS_ZERO_AXIS: i32 = 3;

    pub fn new() -> Self {
        Self {
            hyperbola: None,
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

        if major < 1e-10 || minor < 1e-10 {
            maker.error_status = Self::STATUS_ZERO_AXIS;
            return maker;
        }

        maker.hyperbola = Some((cx, cy, major, minor));
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

        let axis_len_sq = major_axis_x * major_axis_x + major_axis_y * major_axis_y;
        if axis_len_sq < 1e-10 {
            maker.error_status = Self::STATUS_ZERO_AXIS;
            return maker;
        }

        maker.hyperbola = Some((cx, cy, major, minor));
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
        if c < major {
            maker.error_status = Self::STATUS_NEGATIVE_RADIUS;
            return maker;
        }

        let minor = (c * c - major * major).sqrt();

        maker.hyperbola = Some((cx, cy, major, minor));
        maker.done = true;
        maker
    }

    pub fn from_asymptotes_and_major(
        asymptote1: (f64, f64, f64),
        asymptote2: (f64, f64, f64),
        major: f64,
    ) -> Self {
        let mut maker = Self::new();

        if major < 0.0 {
            maker.error_status = Self::STATUS_NEGATIVE_RADIUS;
            return maker;
        }

        let (a1x, a1y, a1_const) = asymptote1;
        let (a2x, a2y, a2_const) = asymptote2;

        let det = a1x * a2y - a1y * a2x;
        if det.abs() < 1e-10 {
            maker.error_status = Self::STATUS_CONFUSED_POINTS;
            return maker;
        }

        let cx = (a1_const * a2y - a1y * a2_const) / det;
        let cy = (a1x * a2_const - a1_const * a2x) / det;

        maker.hyperbola = Some((cx, cy, major, major));
        maker.done = true;
        maker
    }

    pub fn value(&self) -> Option<(f64, f64, f64, f64)> {
        if self.done {
            self.hyperbola
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

impl Default for GceMakeHypr {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_center_and_radii() {
        let maker = GceMakeHypr::from_center_and_radii(0.0, 0.0, 3.0, 4.0);
        assert!(maker.is_done());
        let hypr = maker.value().unwrap();
        assert_eq!(hypr.0, 0.0);
        assert_eq!(hypr.2, 3.0);
    }

    #[test]
    fn test_negative_radius() {
        let maker = GceMakeHypr::from_center_and_radii(0.0, 0.0, -1.0, 2.0);
        assert!(!maker.is_done());
    }

    #[test]
    fn test_zero_axis() {
        let maker = GceMakeHypr::from_center_and_radii(0.0, 0.0, 0.0, 2.0);
        assert!(!maker.is_done());
    }

    #[test]
    fn test_from_two_foci() {
        let maker = GceMakeHypr::from_two_foci_and_major((3.0, 0.0), (-3.0, 0.0), 2.0);
        assert!(maker.is_done());
    }

    #[test]
    fn test_from_two_foci_invalid() {
        let maker = GceMakeHypr::from_two_foci_and_major((0.0, 0.0), (1.0, 0.0), 2.0);
        assert!(!maker.is_done());
    }

    #[test]
    fn test_from_axis_and_radii() {
        let maker = GceMakeHypr::from_axis_and_radii(0.0, 0.0, 1.0, 0.0, 3.0, 4.0);
        assert!(maker.is_done());
    }

    #[test]
    fn test_from_asymptotes() {
        let maker = GceMakeHypr::from_asymptotes_and_major((1.0, 1.0, 0.0), (1.0, -1.0, 0.0), 1.0);
        assert!(maker.is_done());
    }
}
