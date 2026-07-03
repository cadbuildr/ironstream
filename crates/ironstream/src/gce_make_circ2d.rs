// FILE: gce_make_circ2d.rs
// occt: gce_MakeCirc2d

//! Construction algorithms for 2D circles.
pub struct GceMakeCirc2d {
    circle: Option<(f64, f64, f64)>, // center_x, center_y, radius
    done: bool,
    error_status: i32, // 0 = no error, 1 = NegativeRadius, 2 = ConfusedPoints, etc.
}

impl GceMakeCirc2d {
    const STATUS_OK: i32 = 0;
    const STATUS_NEGATIVE_RADIUS: i32 = 1;
    const STATUS_CONFUSED_POINTS: i32 = 2;

    pub fn new() -> Self {
        Self {
            circle: None,
            done: false,
            error_status: Self::STATUS_OK,
        }
    }

    pub fn from_axis_and_radius(cx: f64, cy: f64, radius: f64) -> Self {
        let mut maker = Self::new();
        if radius < 0.0 {
            maker.error_status = Self::STATUS_NEGATIVE_RADIUS;
        } else {
            maker.circle = Some((cx, cy, radius));
            maker.done = true;
        }
        maker
    }

    pub fn concentric_with_distance(cx: f64, cy: f64, radius: f64, dist: f64) -> Self {
        let new_radius = (radius + dist).abs();
        Self::from_axis_and_radius(cx, cy, new_radius)
    }

    pub fn from_three_points(p1: (f64, f64), p2: (f64, f64), p3: (f64, f64)) -> Self {
        let mut maker = Self::new();

        let (x1, y1) = p1;
        let (x2, y2) = p2;
        let (x3, y3) = p3;

        let dx12 = x2 - x1;
        let dy12 = y2 - y1;
        let dx13 = x3 - x1;
        let dy13 = y3 - y1;

        let det = dx12 * dy13 - dx13 * dy12;

        if det.abs() < 1e-10 {
            maker.error_status = Self::STATUS_CONFUSED_POINTS;
            return maker;
        }

        let d1_sq = dx12 * dx12 + dy12 * dy12;
        let d2_sq = dx13 * dx13 + dy13 * dy13;

        let cx = x1 + (dy13 * d1_sq - dy12 * d2_sq) / (2.0 * det);
        let cy = y1 + (dx12 * d2_sq - dx13 * d1_sq) / (2.0 * det);
        let radius = ((cx - x1).powi(2) + (cy - y1).powi(2)).sqrt();

        maker.circle = Some((cx, cy, radius));
        maker.done = true;
        maker
    }

    pub fn from_center_and_radius(cx: f64, cy: f64, radius: f64) -> Self {
        Self::from_axis_and_radius(cx, cy, radius)
    }

    pub fn from_center_and_point(cx: f64, cy: f64, px: f64, py: f64) -> Self {
        let radius = ((px - cx).powi(2) + (py - cy).powi(2)).sqrt();
        Self::from_axis_and_radius(cx, cy, radius)
    }

    pub fn value(&self) -> Option<(f64, f64, f64)> {
        if self.done {
            self.circle
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

impl Default for GceMakeCirc2d {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_center_and_radius() {
        let maker = GceMakeCirc2d::from_center_and_radius(1.0, 2.0, 3.0);
        assert!(maker.is_done());
        let circle = maker.value().unwrap();
        assert_eq!(circle.0, 1.0);
        assert_eq!(circle.1, 2.0);
        assert_eq!(circle.2, 3.0);
    }

    #[test]
    fn test_negative_radius() {
        let maker = GceMakeCirc2d::from_center_and_radius(0.0, 0.0, -1.0);
        assert!(!maker.is_done());
        assert_eq!(maker.error_status(), 1);
    }

    #[test]
    fn test_from_three_points() {
        let maker = GceMakeCirc2d::from_three_points((0.0, 0.0), (2.0, 0.0), (1.0, 1.732));
        assert!(maker.is_done());
        let circle = maker.value().unwrap();
        assert!(circle.2 > 0.0);
    }

    #[test]
    fn test_from_center_and_point() {
        let maker = GceMakeCirc2d::from_center_and_point(0.0, 0.0, 3.0, 4.0);
        let circle = maker.value().unwrap();
        assert!(circle.2.abs() < 5.1 && circle.2.abs() > 4.9);
    }

    #[test]
    fn test_concentric_with_distance() {
        let maker = GceMakeCirc2d::concentric_with_distance(0.0, 0.0, 5.0, 2.0);
        let circle = maker.value().unwrap();
        assert_eq!(circle.2, 7.0);
    }

    #[test]
    fn test_three_points_collinear() {
        let maker = GceMakeCirc2d::from_three_points((0.0, 0.0), (1.0, 1.0), (2.0, 2.0));
        assert!(!maker.is_done());
    }
}
