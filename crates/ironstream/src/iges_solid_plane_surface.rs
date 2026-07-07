// FILE: iges_solid_plane_surface.rs
// occt: IGESSolid_PlaneSurface

//! Plane Surface entity (IGES Type 190, Form 0 or 1).
//!
//! Defines a bounded or unbounded plane surface.

#[derive(Clone)]
pub struct Point {
    id: usize,
}

impl Point {
    pub fn new(id: usize) -> Self {
        Point { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn is_null(&self) -> bool {
        self.id == 0
    }
}

#[derive(Clone)]
pub struct Direction {
    id: usize,
}

impl Direction {
    pub fn new(id: usize) -> Self {
        Direction { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn is_null(&self) -> bool {
        self.id == 0
    }
}

#[derive(Clone)]
pub struct Curve {
    id: usize,
}

impl Curve {
    pub fn new(id: usize) -> Self {
        Curve { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn is_null(&self) -> bool {
        self.id == 0
    }
}

/// Plane Surface entity
pub struct IGESSolidPlaneSurface {
    point: Option<Point>,
    normal: Option<Direction>,
    boundary_curve: Option<Curve>,
    form_number: i32,
}

impl IGESSolidPlaneSurface {
    /// Creates a new plane surface
    pub fn new() -> Self {
        IGESSolidPlaneSurface {
            point: None,
            normal: None,
            boundary_curve: None,
            form_number: 0,
        }
    }

    /// Initializes the plane surface
    pub fn init(
        &mut self,
        point: Point,
        normal: Direction,
        boundary_curve: Option<Curve>,
    ) {
        self.point = Some(point);
        self.normal = Some(normal);
        self.boundary_curve = boundary_curve.clone();
        // Form 1 if bounded (has boundary), Form 0 if unbounded
        self.form_number = if boundary_curve.is_some() { 1 } else { 0 };
    }

    /// Returns the point on the plane
    pub fn point(&self) -> Option<&Point> {
        self.point.as_ref()
    }

    /// Returns the normal direction
    pub fn normal(&self) -> Option<&Direction> {
        self.normal.as_ref()
    }

    /// Returns the boundary curve (if bounded)
    pub fn boundary_curve(&self) -> Option<&Curve> {
        self.boundary_curve.as_ref()
    }

    /// Returns true if the plane is bounded (Form 1)
    pub fn is_bounded(&self) -> bool {
        self.form_number == 1
    }

    pub fn form_number(&self) -> i32 {
        self.form_number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plane_surface_creation() {
        let ps = IGESSolidPlaneSurface::new();
        assert!(ps.point().is_none());
        assert!(!ps.is_bounded());
    }

    #[test]
    fn test_plane_surface_init_unbounded() {
        let mut ps = IGESSolidPlaneSurface::new();
        let point = Point::new(1);
        let normal = Direction::new(2);

        ps.init(point, normal, None);

        assert!(ps.point().is_some());
        assert!(ps.normal().is_some());
        assert!(!ps.is_bounded());
        assert_eq!(ps.form_number(), 0);
    }

    #[test]
    fn test_plane_surface_init_bounded() {
        let mut ps = IGESSolidPlaneSurface::new();
        let point = Point::new(1);
        let normal = Direction::new(2);
        let boundary = Curve::new(3);

        ps.init(point, normal, Some(boundary));

        assert!(ps.is_bounded());
        assert!(ps.boundary_curve().is_some());
        assert_eq!(ps.form_number(), 1);
    }

    #[test]
    fn test_point_null() {
        let p = Point::new(0);
        assert!(p.is_null());
    }

    #[test]
    fn test_direction_null() {
        let d = Direction::new(0);
        assert!(d.is_null());
    }

    #[test]
    fn test_curve_null() {
        let c = Curve::new(0);
        assert!(c.is_null());
    }
}
