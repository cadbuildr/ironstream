// FILE: i_mesh_data_p_curve.rs
// occt: IMeshData_PCurve

/// Interface class representing pcurve of edge associated with discrete face.
/// Indexation of points starts from zero.
pub struct PCurve {
    points: Vec<(f64, f64)>,
    indices: Vec<i32>,
    orientation: i32,
}

impl PCurve {
    /// Constructor.
    pub fn new(orientation: i32) -> Self {
        PCurve {
            points: Vec::new(),
            indices: Vec::new(),
            orientation,
        }
    }

    /// Inserts new discretization point at the given position.
    pub fn insert_point(&mut self, position: usize, point_x: f64, point_y: f64, _param: f64) {
        if position <= self.points.len() {
            self.points.insert(position, (point_x, point_y));
            self.indices.insert(position, -1);
        }
    }

    /// Adds new discretization point to pcurve.
    pub fn add_point(&mut self, point_x: f64, point_y: f64, _param: f64) {
        self.points.push((point_x, point_y));
        self.indices.push(-1);
    }

    /// Returns discretization point with the given index.
    pub fn get_point(&self, index: usize) -> Option<(f64, f64)> {
        self.points.get(index).copied()
    }

    /// Returns index in mesh corresponded to discretization point with the given index.
    pub fn get_index(&self, index: usize) -> Option<i32> {
        self.indices.get(index).copied()
    }

    /// Sets index in mesh for the discretization point with the given index.
    pub fn set_index(&mut self, index: usize, mesh_index: i32) {
        if index < self.indices.len() {
            self.indices[index] = mesh_index;
        }
    }

    /// Removes point with the given index.
    pub fn remove_point(&mut self, index: usize) {
        if index < self.points.len() {
            self.points.remove(index);
            self.indices.remove(index);
        }
    }

    /// Returns forward flag of this pcurve.
    pub fn is_forward(&self) -> bool {
        self.orientation != 1 // 1 is reversed in TopAbs_Orientation
    }

    /// Returns internal flag of this pcurve.
    pub fn is_internal(&self) -> bool {
        self.orientation == 2 // 2 is internal in TopAbs_Orientation
    }

    /// Returns orientation of the edge associated with current pcurve.
    pub fn get_orientation(&self) -> i32 {
        self.orientation
    }

    /// Returns number of points.
    pub fn points_count(&self) -> usize {
        self.points.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p_curve_new() {
        let pcurve = PCurve::new(0);
        assert_eq!(pcurve.points_count(), 0);
        assert!(pcurve.is_forward());
    }

    #[test]
    fn test_p_curve_add_point() {
        let mut pcurve = PCurve::new(0);
        pcurve.add_point(0.5, 0.7, 0.1);
        assert_eq!(pcurve.points_count(), 1);
        assert_eq!(pcurve.get_point(0), Some((0.5, 0.7)));
    }

    #[test]
    fn test_p_curve_insert_point() {
        let mut pcurve = PCurve::new(0);
        pcurve.add_point(0.5, 0.7, 0.1);
        pcurve.insert_point(0, 0.2, 0.3, 0.05);
        assert_eq!(pcurve.points_count(), 2);
        assert_eq!(pcurve.get_point(0), Some((0.2, 0.3)));
        assert_eq!(pcurve.get_point(1), Some((0.5, 0.7)));
    }

    #[test]
    fn test_p_curve_remove_point() {
        let mut pcurve = PCurve::new(0);
        pcurve.add_point(0.5, 0.7, 0.1);
        pcurve.add_point(0.8, 0.9, 0.2);
        pcurve.remove_point(0);
        assert_eq!(pcurve.points_count(), 1);
        assert_eq!(pcurve.get_point(0), Some((0.8, 0.9)));
    }

    #[test]
    fn test_p_curve_index() {
        let mut pcurve = PCurve::new(0);
        pcurve.add_point(0.5, 0.7, 0.1);
        pcurve.set_index(0, 42);
        assert_eq!(pcurve.get_index(0), Some(42));
    }

    #[test]
    fn test_p_curve_orientation() {
        let pcurve = PCurve::new(0);
        assert!(pcurve.is_forward());
        assert!(!pcurve.is_internal());
    }
}
