// FILE: bisector_poly_bis.rs
// occt: Bisector_PolyBis

use crate::bisector_point_on_bis::BisectorPointOnBis;

/// Polygon of points on bisector.
#[derive(Debug, Clone)]
pub struct BisectorPolyBis {
    points: Vec<BisectorPointOnBis>,
}

impl BisectorPolyBis {
    /// Create an empty bisector polygon.
    pub fn new() -> Self {
        BisectorPolyBis {
            points: Vec::new(),
        }
    }

    /// Append a point to the polygon.
    pub fn append(&mut self, point: BisectorPointOnBis) {
        if self.points.len() < 30 {
            self.points.push(point);
        }
    }

    /// Return the number of points.
    pub fn length(&self) -> usize {
        self.points.len()
    }

    /// Check if polygon is empty.
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Get point by index (1-based indexing for OCCT compat).
    pub fn value(&self, index: usize) -> Option<&BisectorPointOnBis> {
        if index > 0 && index <= self.points.len() {
            Some(&self.points[index - 1])
        } else {
            None
        }
    }

    /// Get first point.
    pub fn first(&self) -> Option<&BisectorPointOnBis> {
        self.points.first()
    }

    /// Get last point.
    pub fn last(&self) -> Option<&BisectorPointOnBis> {
        self.points.last()
    }

    /// Find interval containing parameter U.
    pub fn interval(&self, u: f64) -> Option<usize> {
        for (i, point) in self.points.iter().enumerate() {
            if point.param_on_bis() >= u {
                return Some(i + 1);
            }
        }
        None
    }
}

impl Default for BisectorPolyBis {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poly_bis_creation() {
        let poly = BisectorPolyBis::new();
        assert!(poly.is_empty());
        assert_eq!(poly.length(), 0);
    }

    #[test]
    fn test_poly_bis_append() {
        let mut poly = BisectorPolyBis::new();
        let p1 = BisectorPointOnBis::with_params(1.0, 2.0, 3.0, 4.0, (5.0, 6.0));
        poly.append(p1);
        assert_eq!(poly.length(), 1);
        assert!(!poly.is_empty());
    }

    #[test]
    fn test_poly_bis_access() {
        let mut poly = BisectorPolyBis::new();
        let p1 = BisectorPointOnBis::with_params(1.0, 2.0, 3.0, 4.0, (5.0, 6.0));
        poly.append(p1.clone());

        assert!(poly.first().is_some());
        assert!(poly.last().is_some());
        assert!(poly.value(1).is_some());
        assert!(poly.value(0).is_none());
    }
}
