// FILE: poly_coherent_node.rs
// occt: Poly_CoherentNode

/// Node of coherent triangulation.
/// Contains 3D coordinates, 2D coordinates, list of incident triangles, and index.
#[derive(Debug, Clone)]
pub struct CoherentNode {
    pub x: f64,          // 3D coordinate X
    pub y: f64,          // 3D coordinate Y
    pub z: f64,          // 3D coordinate Z
    pub uv: [f64; 2],    // 2D coordinates
    pub index: i32,      // Original node index
}

impl CoherentNode {
    /// Empty constructor.
    pub fn new() -> Self {
        CoherentNode {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            uv: [f64::INFINITY, f64::INFINITY],
            index: -1,
        }
    }

    /// Constructor from 3D coordinates.
    pub fn from_xyz(x: f64, y: f64, z: f64) -> Self {
        CoherentNode {
            x,
            y,
            z,
            uv: [f64::INFINITY, f64::INFINITY],
            index: -1,
        }
    }

    /// Constructor from 3D and 2D coordinates.
    pub fn from_xyz_uv(x: f64, y: f64, z: f64, u: f64, v: f64) -> Self {
        CoherentNode {
            x,
            y,
            z,
            uv: [u, v],
            index: -1,
        }
    }

    /// Get X coordinate.
    pub fn get_x(&self) -> f64 {
        self.x
    }

    /// Get Y coordinate.
    pub fn get_y(&self) -> f64 {
        self.y
    }

    /// Get Z coordinate.
    pub fn get_z(&self) -> f64 {
        self.z
    }

    /// Set coordinates.
    pub fn set_xyz(&mut self, x: f64, y: f64, z: f64) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    /// Get U coordinate.
    pub fn get_u(&self) -> f64 {
        self.uv[0]
    }

    /// Get V coordinate.
    pub fn get_v(&self) -> f64 {
        self.uv[1]
    }

    /// Set 2D coordinates.
    pub fn set_uv(&mut self, u: f64, v: f64) {
        self.uv[0] = u;
        self.uv[1] = v;
    }

    /// Get node index.
    pub fn get_index(&self) -> i32 {
        self.index
    }

    /// Set node index.
    pub fn set_index(&mut self, idx: i32) {
        self.index = idx;
    }

    /// Distance to another node.
    pub fn distance_to(&self, other: &CoherentNode) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Squared distance to another node.
    pub fn distance_sq_to(&self, other: &CoherentNode) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

impl Default for CoherentNode {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coherent_node_new() {
        let node = CoherentNode::new();
        assert_eq!(node.x, 0.0);
        assert_eq!(node.y, 0.0);
        assert_eq!(node.z, 0.0);
        assert!(node.uv[0].is_infinite());
    }

    #[test]
    fn test_coherent_node_from_xyz() {
        let node = CoherentNode::from_xyz(1.0, 2.0, 3.0);
        assert_eq!(node.get_x(), 1.0);
        assert_eq!(node.get_y(), 2.0);
        assert_eq!(node.get_z(), 3.0);
    }

    #[test]
    fn test_coherent_node_from_xyz_uv() {
        let node = CoherentNode::from_xyz_uv(1.0, 2.0, 3.0, 0.5, 0.6);
        assert_eq!(node.get_x(), 1.0);
        assert_eq!(node.get_u(), 0.5);
        assert_eq!(node.get_v(), 0.6);
    }

    #[test]
    fn test_coherent_node_set_xyz() {
        let mut node = CoherentNode::new();
        node.set_xyz(5.0, 6.0, 7.0);
        assert_eq!(node.get_x(), 5.0);
        assert_eq!(node.get_y(), 6.0);
        assert_eq!(node.get_z(), 7.0);
    }

    #[test]
    fn test_coherent_node_index() {
        let mut node = CoherentNode::new();
        assert_eq!(node.get_index(), -1);
        node.set_index(42);
        assert_eq!(node.get_index(), 42);
    }

    #[test]
    fn test_coherent_node_distance() {
        let n1 = CoherentNode::from_xyz(0.0, 0.0, 0.0);
        let n2 = CoherentNode::from_xyz(3.0, 4.0, 0.0);
        assert!((n1.distance_to(&n2) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_coherent_node_distance_sq() {
        let n1 = CoherentNode::from_xyz(0.0, 0.0, 0.0);
        let n2 = CoherentNode::from_xyz(3.0, 4.0, 0.0);
        assert!((n1.distance_sq_to(&n2) - 25.0).abs() < 1e-10);
    }
}
