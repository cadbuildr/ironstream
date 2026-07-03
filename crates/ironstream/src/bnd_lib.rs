// FILE: bnd_lib.rs
// occt: BndLib

//! Bounding box computation utility library for geometric shapes.

#[derive(Clone, Copy, Debug)]
pub struct Box {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub z_min: f64,
    pub z_max: f64,
}

impl Box {
    pub fn new(x_min: f64, x_max: f64, y_min: f64, y_max: f64, z_min: f64, z_max: f64) -> Self {
        Box {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.x_min > self.x_max || self.y_min > self.y_max || self.z_min > self.z_max
    }

    pub fn extend(&mut self, point: (f64, f64, f64)) {
        self.x_min = self.x_min.min(point.0);
        self.x_max = self.x_max.max(point.0);
        self.y_min = self.y_min.min(point.1);
        self.y_max = self.y_max.max(point.1);
        self.z_min = self.z_min.min(point.2);
        self.z_max = self.z_max.max(point.2);
    }

    pub fn contains(&self, point: (f64, f64, f64)) -> bool {
        point.0 >= self.x_min
            && point.0 <= self.x_max
            && point.1 >= self.y_min
            && point.1 <= self.y_max
            && point.2 >= self.z_min
            && point.2 <= self.z_max
    }
}

pub struct BndLib;

impl BndLib {
    pub fn compute_box_for_points(points: &[(f64, f64, f64)]) -> Option<Box> {
        if points.is_empty() {
            return None;
        }

        let mut bbox = Box::new(f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::INFINITY, f64::NEG_INFINITY);
        for &point in points {
            bbox.extend(point);
        }

        Some(bbox)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_creation() {
        let bbox = Box::new(0.0, 10.0, 0.0, 10.0, 0.0, 10.0);
        assert_eq!(bbox.x_min, 0.0);
        assert_eq!(bbox.x_max, 10.0);
    }

    #[test]
    fn test_is_empty() {
        let bbox = Box::new(0.0, 10.0, 0.0, 10.0, 0.0, 10.0);
        assert!(!bbox.is_empty());

        let empty = Box::new(10.0, 0.0, 0.0, 10.0, 0.0, 10.0);
        assert!(empty.is_empty());
    }

    #[test]
    fn test_extend() {
        let mut bbox = Box::new(5.0, 5.0, 5.0, 5.0, 5.0, 5.0);
        bbox.extend((0.0, 0.0, 0.0));
        bbox.extend((10.0, 10.0, 10.0));
        assert_eq!(bbox.x_min, 0.0);
        assert_eq!(bbox.x_max, 10.0);
    }

    #[test]
    fn test_contains() {
        let bbox = Box::new(0.0, 10.0, 0.0, 10.0, 0.0, 10.0);
        assert!(bbox.contains((5.0, 5.0, 5.0)));
        assert!(!bbox.contains((15.0, 5.0, 5.0)));
    }

    #[test]
    fn test_compute_box_for_points() {
        let points = vec![(0.0, 0.0, 0.0), (1.0, 2.0, 3.0), (2.0, 1.0, 0.0)];
        let bbox = BndLib::compute_box_for_points(&points).unwrap();
        assert_eq!(bbox.x_min, 0.0);
        assert_eq!(bbox.x_max, 2.0);
        assert_eq!(bbox.y_min, 0.0);
        assert_eq!(bbox.y_max, 2.0);
    }
}
