// FILE: hlrb_rep_list_of_b_pnt2_d.rs
// occt: HLRBRep_ListOfBPnt2D

//! Deprecated: Use Vec<BPnt2D> directly.
//! List of 2D points for HLR.

#[derive(Clone, Debug)]
pub struct BPnt2D {
    pub x: f64,
    pub y: f64,
}

impl BPnt2D {
    pub fn new(x: f64, y: f64) -> Self {
        BPnt2D { x, y }
    }

    pub fn distance_to(&self, other: &BPnt2D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

pub type HLRBRepListOfBPnt2D = Vec<BPnt2D>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_creation() {
        let mut list: HLRBRepListOfBPnt2D = Vec::new();
        list.push(BPnt2D::new(1.0, 2.0));

        assert_eq!(list.len(), 1);
        assert_eq!(list[0].x, 1.0);
    }

    #[test]
    fn test_point_distance() {
        let p1 = BPnt2D::new(0.0, 0.0);
        let p2 = BPnt2D::new(3.0, 4.0);
        assert_eq!(p1.distance_to(&p2), 5.0);
    }

    #[test]
    fn test_list_operations() {
        let list = vec![
            BPnt2D::new(0.0, 0.0),
            BPnt2D::new(1.0, 1.0),
            BPnt2D::new(2.0, 2.0),
        ];

        assert_eq!(list.len(), 3);
        assert_eq!(list[2].y, 2.0);
    }

    #[test]
    fn test_list_iteration() {
        let list = vec![
            BPnt2D::new(1.0, 0.0),
            BPnt2D::new(0.0, 1.0),
        ];

        let sum: f64 = list.iter().map(|p| p.x + p.y).sum();
        assert_eq!(sum, 2.0);
    }
}
