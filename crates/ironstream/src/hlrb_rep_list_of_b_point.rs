// FILE: hlrb_rep_list_of_b_point.rs
// occt: HLRBRep_ListOfBPoint

//! Deprecated: Use Vec<BPoint> directly.
//! List of 3D points for HLR.

#[derive(Clone, Debug)]
pub struct BPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl BPoint {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        BPoint { x, y, z }
    }

    pub fn distance_to(&self, other: &BPoint) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

pub type HLRBRepListOfBPoint = Vec<BPoint>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_creation() {
        let mut list: HLRBRepListOfBPoint = Vec::new();
        list.push(BPoint::new(1.0, 2.0, 3.0));

        assert_eq!(list.len(), 1);
        assert_eq!(list[0].x, 1.0);
    }

    #[test]
    fn test_point_distance() {
        let p1 = BPoint::new(0.0, 0.0, 0.0);
        let p2 = BPoint::new(3.0, 4.0, 0.0);
        assert_eq!(p1.distance_to(&p2), 5.0);
    }

    #[test]
    fn test_list_operations() {
        let list = vec![
            BPoint::new(0.0, 0.0, 0.0),
            BPoint::new(1.0, 1.0, 1.0),
        ];

        assert_eq!(list.len(), 2);
        assert_eq!(list[1].z, 1.0);
    }

    #[test]
    fn test_list_iteration() {
        let list = vec![
            BPoint::new(1.0, 0.0, 0.0),
            BPoint::new(0.0, 1.0, 0.0),
        ];

        let sum_y: f64 = list.iter().map(|p| p.y).sum();
        assert_eq!(sum_y, 1.0);
    }
}
