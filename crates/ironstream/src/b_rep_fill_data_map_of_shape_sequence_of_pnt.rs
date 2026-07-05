// FILE: b_rep_fill_data_map_of_shape_sequence_of_pnt.rs
// occt: BRepFill_DataMapOfShapeSequenceOfPnt

//! Deprecated type alias for backward compatibility.
//! Maps shapes to sequences of 3D points.

use std::collections::HashMap;

/// A 3D point representation.
#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Pnt {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Pnt {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn distance_to(&self, other: &Pnt) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

/// A data map from shape identifiers to sequences of 3D points.
pub type BRepFillDataMapOfShapeSequenceOfPnt = HashMap<usize, Vec<Pnt>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pnt_creation() {
        let p = Pnt::new(1.0, 2.0, 3.0);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 3.0);
    }

    #[test]
    fn test_pnt_distance() {
        let p1 = Pnt::new(0.0, 0.0, 0.0);
        let p2 = Pnt::new(3.0, 4.0, 0.0);
        assert!((p1.distance_to(&p2) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_map_creation() {
        let mut map: BRepFillDataMapOfShapeSequenceOfPnt = HashMap::new();
        let mut points = Vec::new();
        points.push(Pnt::new(0.0, 0.0, 0.0));
        points.push(Pnt::new(1.0, 2.0, 3.0));
        points.push(Pnt::new(4.0, 5.0, 6.0));

        map.insert(1, points);

        assert_eq!(map.len(), 1);
        assert_eq!(map[&1].len(), 3);
        assert_eq!(map[&1][0], Pnt::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_map_multiple_shapes() {
        let mut map: BRepFillDataMapOfShapeSequenceOfPnt = HashMap::new();

        for shape_id in 0..3 {
            let mut points = Vec::new();
            for i in 0..2 {
                points.push(Pnt::new(i as f64, shape_id as f64, 0.0));
            }
            map.insert(shape_id, points);
        }

        assert_eq!(map.len(), 3);
        assert_eq!(map[&1].len(), 2);
        assert_eq!(map[&1][0].y, 1.0);
    }
}
