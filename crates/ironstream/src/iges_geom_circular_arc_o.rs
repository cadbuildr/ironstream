// FILE: iges_geom_circular_arc_o.rs
// occt: IGESGeom_CircularArc

pub struct Point2D {
    x: f64,
    y: f64,
}

pub struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

pub struct Direction {
    x: f64,
    y: f64,
    z: f64,
}

pub struct IgesGeomCircularArc {
    z_t: f64,
    center: Point2D,
    start: Point2D,
    end: Point2D,
}

impl IgesGeomCircularArc {
    pub fn new() -> Self {
        IgesGeomCircularArc {
            z_t: 0.0,
            center: Point2D { x: 0.0, y: 0.0 },
            start: Point2D { x: 0.0, y: 0.0 },
            end: Point2D { x: 0.0, y: 0.0 },
        }
    }

    pub fn init(&mut self, z_t: f64, center: (f64, f64), start: (f64, f64), end: (f64, f64)) {
        self.z_t = z_t;
        self.center = Point2D { x: center.0, y: center.1 };
        self.start = Point2D { x: start.0, y: start.1 };
        self.end = Point2D { x: end.0, y: end.1 };
    }

    pub fn center(&self) -> (f64, f64) {
        (self.center.x, self.center.y)
    }

    pub fn start_point(&self) -> (f64, f64) {
        (self.start.x, self.start.y)
    }

    pub fn end_point(&self) -> (f64, f64) {
        (self.end.x, self.end.y)
    }

    pub fn z_plane(&self) -> f64 {
        self.z_t
    }

    pub fn radius(&self) -> f64 {
        let dx = self.start.x - self.center.x;
        let dy = self.start.y - self.center.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn angle(&self) -> f64 {
        let start_angle = (self.start.y - self.center.y).atan2(self.start.x - self.center.x);
        let end_angle = (self.end.y - self.center.y).atan2(self.end.x - self.center.x);
        let mut angle = end_angle - start_angle;
        if angle < 0.0 {
            angle += 2.0 * std::f64::consts::PI;
        }
        angle
    }

    pub fn axis(&self) -> (f64, f64, f64) {
        (0.0, 0.0, 1.0)
    }

    pub fn is_closed(&self) -> bool {
        (self.start.x - self.end.x).abs() < 1e-10 && (self.start.y - self.end.y).abs() < 1e-10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_arc_creation() {
        let arc = IgesGeomCircularArc::new();
        assert_eq!(arc.z_plane(), 0.0);
        assert_eq!(arc.center(), (0.0, 0.0));
    }

    #[test]
    fn test_circular_arc_init() {
        let mut arc = IgesGeomCircularArc::new();
        arc.init(1.5, (0.0, 0.0), (1.0, 0.0), (0.0, 1.0));

        assert_eq!(arc.z_plane(), 1.5);
        assert_eq!(arc.center(), (0.0, 0.0));
        assert_eq!(arc.start_point(), (1.0, 0.0));
        assert_eq!(arc.end_point(), (0.0, 1.0));
    }

    #[test]
    fn test_radius_calculation() {
        let mut arc = IgesGeomCircularArc::new();
        arc.init(0.0, (0.0, 0.0), (1.0, 0.0), (0.0, 1.0));
        let r = arc.radius();
        assert!((r - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_is_closed() {
        let mut arc1 = IgesGeomCircularArc::new();
        arc1.init(0.0, (0.0, 0.0), (1.0, 0.0), (0.0, 1.0));
        assert!(!arc1.is_closed());

        let mut arc2 = IgesGeomCircularArc::new();
        arc2.init(0.0, (0.0, 0.0), (1.0, 0.0), (1.0, 0.0));
        assert!(arc2.is_closed());
    }

    #[test]
    fn test_axis() {
        let arc = IgesGeomCircularArc::new();
        assert_eq!(arc.axis(), (0.0, 0.0, 1.0));
    }
}
