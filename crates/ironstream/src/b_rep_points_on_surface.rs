// FILE: b_rep_points_on_surface.rs
// occt: BRep_PointsOnSurface

use std::collections::HashMap;

/// Manages points defined on a surface.
pub struct BRepPointsOnSurface {
    points: HashMap<u32, (f64, f64, f64)>, // point_id -> (u, v, tolerance)
    surface_id: u32,
}

impl BRepPointsOnSurface {
    /// Create a new points-on-surface container.
    pub fn new(surface_id: u32) -> Self {
        Self {
            points: HashMap::new(),
            surface_id,
        }
    }

    /// Get the surface id.
    pub fn surface_id(&self) -> u32 {
        self.surface_id
    }

    /// Add a point on the surface.
    pub fn add_point(&mut self, point_id: u32, u: f64, v: f64, tolerance: f64) {
        self.points.insert(point_id, (u, v, tolerance));
    }

    /// Get point parameters.
    pub fn get_point(&self, point_id: u32) -> Option<(f64, f64, f64)> {
        self.points.get(&point_id).copied()
    }

    /// Remove a point.
    pub fn remove_point(&mut self, point_id: u32) {
        self.points.remove(&point_id);
    }

    /// Get the number of points.
    pub fn point_count(&self) -> usize {
        self.points.len()
    }

    /// Clear all points.
    pub fn clear(&mut self) {
        self.points.clear();
    }

    /// Get all point ids.
    pub fn point_ids(&self) -> Vec<u32> {
        self.points.keys().copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pts = BRepPointsOnSurface::new(1);
        assert_eq!(pts.surface_id(), 1);
        assert_eq!(pts.point_count(), 0);
    }

    #[test]
    fn test_add_point() {
        let mut pts = BRepPointsOnSurface::new(1);
        pts.add_point(1, 0.5, 0.5, 0.01);

        assert_eq!(pts.get_point(1), Some((0.5, 0.5, 0.01)));
        assert_eq!(pts.point_count(), 1);
    }

    #[test]
    fn test_remove_point() {
        let mut pts = BRepPointsOnSurface::new(1);
        pts.add_point(1, 0.5, 0.5, 0.01);
        pts.remove_point(1);

        assert_eq!(pts.get_point(1), None);
        assert_eq!(pts.point_count(), 0);
    }

    #[test]
    fn test_point_ids() {
        let mut pts = BRepPointsOnSurface::new(1);
        pts.add_point(1, 0.5, 0.5, 0.01);
        pts.add_point(2, 0.6, 0.6, 0.01);

        let ids = pts.point_ids();
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&1));
        assert!(ids.contains(&2));
    }
}
