// FILE: vrml_coordinate3.rs
// occt: Vrml_Coordinate3
//
// Faithful port of OCCT Vrml_Coordinate3 (DataExchange/TKDEVRML/Vrml/
// Vrml_Coordinate3.hxx/.cxx): VRML 1.0 Coordinate3 node.
// Stores a collection of 3D points for geometry vertex definitions.

use std::cell::RefCell;
use std::rc::Rc;

/// Single 3D coordinate (f32 for VRML compact storage).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinate3Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Coordinate3Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Coordinate3Point { x, y, z }
    }

    /// Euclidean distance to another point.
    pub fn distance_to(&self, other: &Coordinate3Point) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Magnitude (distance from origin).
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Normalize this point as a direction vector (in-place).
    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        if mag > 1e-7 {
            self.x /= mag;
            self.y /= mag;
            self.z /= mag;
        }
    }

    /// Return the normalized version as a direction vector.
    pub fn normalized(&self) -> Self {
        let mag = self.magnitude();
        if mag > 1e-7 {
            Coordinate3Point {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            }
        } else {
            *self
        }
    }

    /// Dot product with another vector.
    pub fn dot(&self, other: &Coordinate3Point) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Cross product with another vector.
    pub fn cross(&self, other: &Coordinate3Point) -> Coordinate3Point {
        Coordinate3Point {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Default for Coordinate3Point {
    fn default() -> Self {
        Coordinate3Point::new(0.0, 0.0, 0.0)
    }
}

impl std::ops::Add for Coordinate3Point {
    type Output = Coordinate3Point;

    fn add(self, other: Coordinate3Point) -> Coordinate3Point {
        Coordinate3Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Coordinate3Point {
    type Output = Coordinate3Point;

    fn sub(self, other: Coordinate3Point) -> Coordinate3Point {
        Coordinate3Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul<f32> for Coordinate3Point {
    type Output = Coordinate3Point;

    fn mul(self, scalar: f32) -> Coordinate3Point {
        Coordinate3Point {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

/// VRML 1.0 Coordinate3 node: collection of 3D vertex coordinates.
/// Used by geometric nodes to specify vertex positions.
pub struct VrmlCoordinate3 {
    my_points: Vec<Coordinate3Point>,
    my_name: String,
}

impl VrmlCoordinate3 {
    /// Constructor: empty coordinate list.
    pub fn new(name: Option<&str>) -> Self {
        VrmlCoordinate3 {
            my_points: Vec::new(),
            my_name: name.unwrap_or("").to_string(),
        }
    }

    /// Query the name.
    pub fn name(&self) -> &str {
        &self.my_name
    }

    /// Set the name.
    pub fn set_name(&mut self, name: &str) {
        self.my_name = name.to_string();
    }

    /// Add a coordinate point.
    pub fn add_point(&mut self, point: Coordinate3Point) {
        self.my_points.push(point);
    }

    /// Get the number of points.
    pub fn count(&self) -> usize {
        self.my_points.len()
    }

    /// Get a point by index (0-based). Returns None if out of range.
    pub fn get(&self, index: usize) -> Option<Coordinate3Point> {
        self.my_points.get(index).copied()
    }

    /// Get all points as a slice.
    pub fn points(&self) -> &[Coordinate3Point] {
        &self.my_points
    }

    /// Set all points from a vector.
    pub fn set_points(&mut self, points: Vec<Coordinate3Point>) {
        self.my_points = points;
    }

    /// Clear all points.
    pub fn clear(&mut self) {
        self.my_points.clear();
    }

    /// Check if in default state (empty).
    pub fn is_default(&self) -> bool {
        self.my_points.is_empty()
    }

    /// Compute axis-aligned bounding box.
    pub fn compute_bounds(&self) -> Option<(Coordinate3Point, Coordinate3Point)> {
        if self.my_points.is_empty() {
            return None;
        }

        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut min_z = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;
        let mut max_z = f32::MIN;

        for point in &self.my_points {
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            min_z = min_z.min(point.z);
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
            max_z = max_z.max(point.z);
        }

        Some((
            Coordinate3Point::new(min_x, min_y, min_z),
            Coordinate3Point::new(max_x, max_y, max_z),
        ))
    }

    /// Compute the center (average) of all points.
    pub fn compute_center(&self) -> Option<Coordinate3Point> {
        if self.my_points.is_empty() {
            return None;
        }

        let mut sum = Coordinate3Point::default();
        for point in &self.my_points {
            sum = sum + *point;
        }

        let count = self.my_points.len() as f32;
        Some(Coordinate3Point {
            x: sum.x / count,
            y: sum.y / count,
            z: sum.z / count,
        })
    }
}

impl Default for VrmlCoordinate3 {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Clone for VrmlCoordinate3 {
    fn clone(&self) -> Self {
        VrmlCoordinate3 {
            my_points: self.my_points.clone(),
            my_name: self.my_name.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coordinate3_point_creation() {
        let p = Coordinate3Point::new(1.0, 2.0, 3.0);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 3.0);
    }

    #[test]
    fn coordinate3_point_default() {
        let p = Coordinate3Point::default();
        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.0);
        assert_eq!(p.z, 0.0);
    }

    #[test]
    fn coordinate3_point_distance() {
        let p1 = Coordinate3Point::new(0.0, 0.0, 0.0);
        let p2 = Coordinate3Point::new(3.0, 4.0, 0.0);
        assert!((p1.distance_to(&p2) - 5.0).abs() < 1e-6);
    }

    #[test]
    fn coordinate3_point_magnitude() {
        let p = Coordinate3Point::new(3.0, 4.0, 0.0);
        assert!((p.magnitude() - 5.0).abs() < 1e-6);
    }

    #[test]
    fn coordinate3_point_normalize() {
        let mut p = Coordinate3Point::new(3.0, 4.0, 0.0);
        p.normalize();
        assert!((p.x - 0.6).abs() < 1e-6);
        assert!((p.y - 0.8).abs() < 1e-6);
    }

    #[test]
    fn coordinate3_point_dot() {
        let p1 = Coordinate3Point::new(1.0, 0.0, 0.0);
        let p2 = Coordinate3Point::new(0.0, 1.0, 0.0);
        assert!(p1.dot(&p2).abs() < 1e-7);

        let p3 = Coordinate3Point::new(2.0, 0.0, 0.0);
        assert!((p1.dot(&p3) - 2.0).abs() < 1e-7);
    }

    #[test]
    fn coordinate3_point_cross() {
        let p1 = Coordinate3Point::new(1.0, 0.0, 0.0);
        let p2 = Coordinate3Point::new(0.0, 1.0, 0.0);
        let cross = p1.cross(&p2);
        // (1,0,0) x (0,1,0) = (0,0,1)
        assert!(cross.x.abs() < 1e-7);
        assert!(cross.y.abs() < 1e-7);
        assert!((cross.z - 1.0).abs() < 1e-7);
    }

    #[test]
    fn coordinate3_list_creation() {
        let coord = VrmlCoordinate3::new(Some("Coords"));
        assert_eq!(coord.name(), "Coords");
        assert_eq!(coord.count(), 0);
        assert!(coord.is_default());
    }

    #[test]
    fn add_points() {
        let mut coord = VrmlCoordinate3::new(None);
        coord.add_point(Coordinate3Point::new(0.0, 0.0, 0.0));
        coord.add_point(Coordinate3Point::new(1.0, 1.0, 1.0));
        assert_eq!(coord.count(), 2);
        assert!(!coord.is_default());
    }

    #[test]
    fn get_point() {
        let mut coord = VrmlCoordinate3::new(None);
        let p = Coordinate3Point::new(0.5, 0.5, 0.5);
        coord.add_point(p);
        assert_eq!(coord.get(0), Some(p));
        assert_eq!(coord.get(1), None);
    }

    #[test]
    fn set_points() {
        let mut coord = VrmlCoordinate3::new(None);
        let points = vec![
            Coordinate3Point::new(0.0, 0.0, 0.0),
            Coordinate3Point::new(1.0, 0.0, 0.0),
            Coordinate3Point::new(1.0, 1.0, 0.0),
        ];
        coord.set_points(points.clone());
        assert_eq!(coord.count(), 3);
    }

    #[test]
    fn compute_bounds() {
        let mut coord = VrmlCoordinate3::new(None);
        coord.add_point(Coordinate3Point::new(1.0, 2.0, 3.0));
        coord.add_point(Coordinate3Point::new(4.0, 5.0, 6.0));
        coord.add_point(Coordinate3Point::new(2.5, 3.5, 4.5));

        let bounds = coord.compute_bounds();
        assert!(bounds.is_some());
        let (min, max) = bounds.unwrap();
        assert!((min.x - 1.0).abs() < 1e-6);
        assert!((max.x - 4.0).abs() < 1e-6);
    }

    #[test]
    fn compute_center() {
        let mut coord = VrmlCoordinate3::new(None);
        coord.add_point(Coordinate3Point::new(0.0, 0.0, 0.0));
        coord.add_point(Coordinate3Point::new(2.0, 2.0, 2.0));

        let center = coord.compute_center();
        assert!(center.is_some());
        let c = center.unwrap();
        assert!((c.x - 1.0).abs() < 1e-6);
        assert!((c.y - 1.0).abs() < 1e-6);
        assert!((c.z - 1.0).abs() < 1e-6);
    }

    #[test]
    fn point_add() {
        let p1 = Coordinate3Point::new(1.0, 2.0, 3.0);
        let p2 = Coordinate3Point::new(4.0, 5.0, 6.0);
        let sum = p1 + p2;
        assert_eq!(sum, Coordinate3Point::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn point_sub() {
        let p1 = Coordinate3Point::new(5.0, 7.0, 9.0);
        let p2 = Coordinate3Point::new(1.0, 2.0, 3.0);
        let diff = p1 - p2;
        assert_eq!(diff, Coordinate3Point::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn point_mul() {
        let p = Coordinate3Point::new(1.0, 2.0, 3.0);
        let scaled = p * 2.0;
        assert_eq!(scaled, Coordinate3Point::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn clone_preserves_data() {
        let mut coord = VrmlCoordinate3::new(Some("Original"));
        coord.add_point(Coordinate3Point::new(1.0, 2.0, 3.0));
        let cloned = coord.clone();
        assert_eq!(cloned.name(), "Original");
        assert_eq!(cloned.count(), 1);
    }
}
