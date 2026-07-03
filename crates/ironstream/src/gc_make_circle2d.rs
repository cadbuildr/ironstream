// FILE: gc_make_circle2d.rs
// occt: GC_MakeCircle2d

//! Construction algorithms for circles in the plane.
//! Creates Geom2d_Circle objects from various input data.

/// Construction error status.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MakeStatus {
    Ok,
    NegativeRadius,
    InsufficientData,
    ColinearPoints,
}

/// A 2D circle representation.
#[derive(Clone, Debug)]
pub struct Circle2d {
    /// Center point (x, y)
    pub center: (f64, f64),
    /// Radius
    pub radius: f64,
    /// Counter-clockwise orientation
    pub ccw: bool,
}

impl Circle2d {
    /// Create a circle from center and radius.
    pub fn new(center: (f64, f64), radius: f64, ccw: bool) -> Option<Self> {
        if radius < 0.0 {
            return None;
        }
        Some(Circle2d { center, radius, ccw })
    }

    /// Distance from point to circle center.
    pub fn distance_to_center(&self, p: (f64, f64)) -> f64 {
        let dx = p.0 - self.center.0;
        let dy = p.1 - self.center.1;
        (dx * dx + dy * dy).sqrt()
    }

    /// Check if a point lies on the circle (within tolerance).
    pub fn contains_point(&self, p: (f64, f64), tolerance: f64) -> bool {
        let dist = self.distance_to_center(p);
        (dist - self.radius).abs() < tolerance
    }
}

/// Circle builder for 2D circles.
pub struct MakeCircle2d {
    circle: Option<Circle2d>,
    status: MakeStatus,
}

impl MakeCircle2d {
    /// Create from an axis and radius.
    pub fn from_axis_and_radius(center: (f64, f64), radius: f64, ccw: bool) -> Self {
        if radius < 0.0 {
            return MakeCircle2d {
                circle: None,
                status: MakeStatus::NegativeRadius,
            };
        }

        MakeCircle2d {
            circle: Some(Circle2d {
                center,
                radius,
                ccw,
            }),
            status: MakeStatus::Ok,
        }
    }

    /// Create from center and radius.
    pub fn from_center_and_radius(center: (f64, f64), radius: f64, ccw: bool) -> Self {
        Self::from_axis_and_radius(center, radius, ccw)
    }

    /// Create from center point and another point on circle.
    pub fn from_center_and_point(center: (f64, f64), point: (f64, f64), ccw: bool) -> Self {
        let dx = point.0 - center.0;
        let dy = point.1 - center.1;
        let radius = (dx * dx + dy * dy).sqrt();

        MakeCircle2d {
            circle: Some(Circle2d { center, radius, ccw }),
            status: MakeStatus::Ok,
        }
    }

    /// Create from three points.
    pub fn from_three_points(p1: (f64, f64), p2: (f64, f64), p3: (f64, f64)) -> Self {
        // Use circumcircle formula
        let (cx, cy, r) = match circumcircle(p1, p2, p3) {
            Some((cx, cy, r)) => (cx, cy, r),
            None => {
                return MakeCircle2d {
                    circle: None,
                    status: MakeStatus::ColinearPoints,
                }
            }
        };

        MakeCircle2d {
            circle: Some(Circle2d {
                center: (cx, cy),
                radius: r,
                ccw: true,
            }),
            status: MakeStatus::Ok,
        }
    }

    /// Create a circle parallel to another at signed distance.
    pub fn from_circle_and_distance(circle: &Circle2d, distance: f64) -> Self {
        let new_radius = circle.radius + distance;
        if new_radius < 0.0 {
            return MakeCircle2d {
                circle: None,
                status: MakeStatus::NegativeRadius,
            };
        }

        MakeCircle2d {
            circle: Some(Circle2d {
                center: circle.center,
                radius: new_radius,
                ccw: circle.ccw,
            }),
            status: MakeStatus::Ok,
        }
    }

    /// Get the constructed circle.
    pub fn value(&self) -> Option<&Circle2d> {
        self.circle.as_ref()
    }

    /// Get construction status.
    pub fn status(&self) -> MakeStatus {
        self.status
    }

    /// Check if construction succeeded.
    pub fn is_done(&self) -> bool {
        self.status == MakeStatus::Ok
    }
}

/// Compute the circumcircle of three points.
fn circumcircle(p1: (f64, f64), p2: (f64, f64), p3: (f64, f64)) -> Option<(f64, f64, f64)> {
    // Use determinant formula for circumcircle
    let ax = p1.0;
    let ay = p1.1;
    let bx = p2.0;
    let by = p2.1;
    let cx = p3.0;
    let cy = p3.1;

    let d = 2.0 * (ax * (by - cy) + bx * (cy - ay) + cx * (ay - by));

    if d.abs() < 1e-15 {
        return None; // Collinear points
    }

    let ux = ((ax * ax + ay * ay) * (by - cy) + (bx * bx + by * by) * (cy - ay) + (cx * cx + cy * cy) * (ay - by)) / d;
    let uy = ((ax * ax + ay * ay) * (cx - bx) + (bx * bx + by * by) * (ax - cx) + (cx * cx + cy * cy) * (bx - ax)) / d;

    let r = ((ax - ux).powi(2) + (ay - uy).powi(2)).sqrt();

    Some((ux, uy, r))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_from_center_and_radius() {
        let maker = MakeCircle2d::from_center_and_radius((0.0, 0.0), 5.0, true);
        assert!(maker.is_done());
        let circle = maker.value().unwrap();
        assert!((circle.radius - 5.0).abs() < 1e-9);
        assert_eq!(circle.center, (0.0, 0.0));
    }

    #[test]
    fn test_negative_radius() {
        let maker = MakeCircle2d::from_center_and_radius((0.0, 0.0), -1.0, true);
        assert!(!maker.is_done());
        assert_eq!(maker.status(), MakeStatus::NegativeRadius);
    }

    #[test]
    fn test_circle_from_center_and_point() {
        let maker = MakeCircle2d::from_center_and_point((0.0, 0.0), (3.0, 4.0), true);
        assert!(maker.is_done());
        let circle = maker.value().unwrap();
        assert!((circle.radius - 5.0).abs() < 1e-9);
    }

    #[test]
    fn test_circle_from_three_points() {
        let maker = MakeCircle2d::from_three_points((0.0, 0.0), (4.0, 0.0), (2.0, 2.0));
        assert!(maker.is_done());
        let circle = maker.value().unwrap();
        assert!(circle.radius > 0.0);
    }

    #[test]
    fn test_collinear_points() {
        let maker = MakeCircle2d::from_three_points((0.0, 0.0), (1.0, 1.0), (2.0, 2.0));
        assert!(!maker.is_done());
        assert_eq!(maker.status(), MakeStatus::ColinearPoints);
    }

    #[test]
    fn test_parallel_circle() {
        let circle = Circle2d::new((0.0, 0.0), 5.0, true).unwrap();
        let maker = MakeCircle2d::from_circle_and_distance(&circle, 2.0);
        assert!(maker.is_done());
        let new_circle = maker.value().unwrap();
        assert!((new_circle.radius - 7.0).abs() < 1e-9);
    }

    #[test]
    fn test_circle_contains_point() {
        let circle = Circle2d::new((0.0, 0.0), 5.0, true).unwrap();
        assert!(circle.contains_point((5.0, 0.0), 1e-6));
        assert!(circle.contains_point((0.0, 5.0), 1e-6));
        assert!(!circle.contains_point((0.0, 0.0), 1e-6));
    }
}
