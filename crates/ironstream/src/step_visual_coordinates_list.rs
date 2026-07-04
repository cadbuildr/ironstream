// FILE: step_visual_coordinates_list.rs
// occt: StepVisual_CoordinatesList

/// A 3D coordinate in STEP representation.
#[derive(Clone, Debug, PartialEq)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    /// Creates a new 3D point.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D { x, y, z }
    }
}

/// A coordinates list in STEP representation.
///
/// This contains a list of 3D points.
pub struct CoordinatesList {
    name: String,
    points: Vec<Point3D>,
}

impl CoordinatesList {
    /// Creates a new coordinates list.
    pub fn new(name: String) -> Self {
        CoordinatesList {
            name,
            points: Vec::new(),
        }
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the points.
    pub fn set_points(&mut self, points: Vec<Point3D>) {
        self.points = points;
    }

    /// Returns the points.
    pub fn points(&self) -> &[Point3D] {
        &self.points
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_3d_new() {
        let p = Point3D::new(1.0, 2.0, 3.0);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 3.0);
    }

    #[test]
    fn test_coordinates_list_new() {
        let list = CoordinatesList::new("Points".to_string());
        assert_eq!(list.name(), "Points");
        assert_eq!(list.points().len(), 0);
    }

    #[test]
    fn test_set_points() {
        let mut list = CoordinatesList::new("MyPoints".to_string());
        let points = vec![
            Point3D::new(0.0, 0.0, 0.0),
            Point3D::new(1.0, 0.0, 0.0),
            Point3D::new(0.0, 1.0, 0.0),
        ];
        list.set_points(points);
        assert_eq!(list.points().len(), 3);
        assert_eq!(list.points()[0], Point3D::new(0.0, 0.0, 0.0));
    }
}
