// FILE: n_collection_kd_tree.rs
// occt: NCollection_KDTree

/// K-Dimensional tree for spatial searches.
pub struct KDTree {
    points: Vec<(f64, f64, f64, usize)>, // (x, y, z, index)
}

impl KDTree {
    pub fn new() -> Self {
        KDTree {
            points: Vec::new(),
        }
    }

    pub fn add_point(&mut self, x: f64, y: f64, z: f64, index: usize) {
        self.points.push((x, y, z, index));
    }

    pub fn find_nearest(&self, x: f64, y: f64, z: f64) -> Option<usize> {
        let mut nearest_idx = None;
        let mut nearest_dist_sq = f64::INFINITY;

        for (px, py, pz, idx) in &self.points {
            let dx = x - px;
            let dy = y - py;
            let dz = z - pz;
            let dist_sq = dx * dx + dy * dy + dz * dz;

            if dist_sq < nearest_dist_sq {
                nearest_dist_sq = dist_sq;
                nearest_idx = Some(*idx);
            }
        }

        nearest_idx
    }

    pub fn find_in_sphere(&self, x: f64, y: f64, z: f64, radius: f64) -> Vec<usize> {
        let r_sq = radius * radius;
        let mut result = Vec::new();

        for (px, py, pz, idx) in &self.points {
            let dx = x - px;
            let dy = y - py;
            let dz = z - pz;
            let dist_sq = dx * dx + dy * dy + dz * dz;

            if dist_sq <= r_sq {
                result.push(*idx);
            }
        }

        result
    }

    pub fn size(&self) -> usize {
        self.points.len()
    }
}

impl Default for KDTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kd_tree() {
        let mut tree = KDTree::new();
        tree.add_point(0.0, 0.0, 0.0, 0);
        tree.add_point(1.0, 1.0, 1.0, 1);
        assert_eq!(tree.size(), 2);
        assert_eq!(tree.find_nearest(0.1, 0.1, 0.1), Some(0));
    }

    #[test]
    fn test_kd_tree_sphere() {
        let mut tree = KDTree::new();
        tree.add_point(0.0, 0.0, 0.0, 0);
        tree.add_point(2.0, 0.0, 0.0, 1);
        let found = tree.find_in_sphere(0.0, 0.0, 0.0, 1.5);
        assert_eq!(found.len(), 1);
    }
}
