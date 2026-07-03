// FILE: poly_coherent_triangulation.rs
// occt: Poly_CoherentTriangulation

use crate::poly_coherent_node::CoherentNode;
use crate::poly_coherent_link::CoherentLink;

/// Triangulation structure that stores connectivity of triangles and nodes.
/// Allows analysis and editing of triangulated meshes.
#[derive(Debug, Clone)]
pub struct CoherentTriangulation {
    nodes: Vec<CoherentNode>,
    triangles: Vec<[i32; 3]>,  // Triangle node indices
    links: Vec<CoherentLink>,
}

impl CoherentTriangulation {
    /// Create new empty triangulation.
    pub fn new() -> Self {
        CoherentTriangulation {
            nodes: Vec::new(),
            triangles: Vec::new(),
            links: Vec::new(),
        }
    }

    /// Add a node to the triangulation.
    pub fn add_node(&mut self, node: CoherentNode) -> usize {
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    /// Get mutable reference to a node.
    pub fn node_mut(&mut self, idx: usize) -> Option<&mut CoherentNode> {
        self.nodes.get_mut(idx)
    }

    /// Get reference to a node.
    pub fn node(&self, idx: usize) -> Option<&CoherentNode> {
        self.nodes.get(idx)
    }

    /// Get number of nodes.
    pub fn nb_nodes(&self) -> usize {
        self.nodes.len()
    }

    /// Add a triangle to the triangulation.
    pub fn add_triangle(&mut self, n0: i32, n1: i32, n2: i32) -> usize {
        self.triangles.push([n0, n1, n2]);
        self.triangles.len() - 1
    }

    /// Get triangle by index.
    pub fn triangle(&self, idx: usize) -> Option<[i32; 3]> {
        self.triangles.get(idx).copied()
    }

    /// Get number of triangles.
    pub fn nb_triangles(&self) -> usize {
        self.triangles.len()
    }

    /// Remove triangle by index (mark as empty).
    pub fn remove_triangle(&mut self, idx: usize) {
        if let Some(tri) = self.triangles.get_mut(idx) {
            tri[0] = -1;
            tri[1] = -1;
            tri[2] = -1;
        }
    }

    /// Check if triangle is empty/removed.
    pub fn is_triangle_empty(&self, idx: usize) -> bool {
        if let Some(tri) = self.triangles.get(idx) {
            tri[0] == -1 && tri[1] == -1 && tri[2] == -1
        } else {
            true
        }
    }

    /// Compute links from current triangles.
    pub fn compute_links(&mut self) {
        self.links.clear();
        let mut link_map = std::collections::HashMap::new();

        for (tri_idx, tri) in self.triangles.iter().enumerate() {
            if tri[0] == -1 {
                continue; // Skip empty triangles
            }

            // Extract three edges from triangle
            let edges = [
                (tri[0].min(tri[1]), tri[0].max(tri[1]), tri[2]),
                (tri[1].min(tri[2]), tri[1].max(tri[2]), tri[0]),
                (tri[2].min(tri[0]), tri[2].max(tri[0]), tri[1]),
            ];

            for (n0, n1, opposite) in edges.iter() {
                let key = (*n0, *n1);
                link_map
                    .entry(key)
                    .or_insert_with(Vec::new)
                    .push(*opposite);
            }
        }

        for ((n0, n1), opposites) in link_map.iter() {
            let mut link = CoherentLink::with_nodes(*n0, *n1);
            if opposites.len() > 0 {
                link.opposite_node[0] = opposites[0];
            }
            if opposites.len() > 1 {
                link.opposite_node[1] = opposites[1];
            }
            self.links.push(link);
        }
    }

    /// Get number of links.
    pub fn nb_links(&self) -> usize {
        self.links.len()
    }

    /// Get link by index.
    pub fn link(&self, idx: usize) -> Option<&CoherentLink> {
        self.links.get(idx)
    }

    /// Clear all links.
    pub fn clear_links(&mut self) {
        self.links.clear();
    }

    /// Find triangle by two nodes.
    pub fn find_triangle(&self, n0: i32, n1: i32) -> Option<usize> {
        for (idx, tri) in self.triangles.iter().enumerate() {
            if tri[0] == -1 {
                continue;
            }
            let nodes = [tri[0], tri[1], tri[2]];
            if (nodes.contains(&n0) && nodes.contains(&n1)) {
                return Some(idx);
            }
        }
        None
    }

    /// Check if node is free (no incident triangles).
    pub fn is_free_node(&self, node_idx: usize) -> bool {
        for tri in self.triangles.iter() {
            if tri[0] != -1 && (tri[0] == node_idx as i32 || tri[1] == node_idx as i32 || tri[2] == node_idx as i32) {
                return false;
            }
        }
        true
    }
}

impl Default for CoherentTriangulation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coherent_triangulation_new() {
        let tri = CoherentTriangulation::new();
        assert_eq!(tri.nb_nodes(), 0);
        assert_eq!(tri.nb_triangles(), 0);
    }

    #[test]
    fn test_add_node() {
        let mut tri = CoherentTriangulation::new();
        let node = CoherentNode::from_xyz(1.0, 2.0, 3.0);
        let idx = tri.add_node(node);
        assert_eq!(idx, 0);
        assert_eq!(tri.nb_nodes(), 1);
    }

    #[test]
    fn test_add_triangle() {
        let mut tri = CoherentTriangulation::new();
        let idx = tri.add_triangle(0, 1, 2);
        assert_eq!(idx, 0);
        assert_eq!(tri.nb_triangles(), 1);
        assert_eq!(tri.triangle(0), Some([0, 1, 2]));
    }

    #[test]
    fn test_remove_triangle() {
        let mut tri = CoherentTriangulation::new();
        tri.add_triangle(0, 1, 2);
        tri.remove_triangle(0);
        assert!(tri.is_triangle_empty(0));
    }

    #[test]
    fn test_compute_links() {
        let mut tri = CoherentTriangulation::new();
        tri.add_node(CoherentNode::from_xyz(0.0, 0.0, 0.0));
        tri.add_node(CoherentNode::from_xyz(1.0, 0.0, 0.0));
        tri.add_node(CoherentNode::from_xyz(0.0, 1.0, 0.0));
        tri.add_triangle(0, 1, 2);
        tri.compute_links();
        assert!(tri.nb_links() > 0);
    }

    #[test]
    fn test_is_free_node() {
        let mut tri = CoherentTriangulation::new();
        tri.add_node(CoherentNode::new());
        tri.add_node(CoherentNode::new());
        tri.add_node(CoherentNode::new());
        assert!(tri.is_free_node(0));
        tri.add_triangle(0, 1, 2);
        assert!(!tri.is_free_node(0));
    }
}
