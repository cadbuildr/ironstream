// FILE: xcaf_assembly.rs

// occt: XCAFDoc_Location
/// A 4×4 homogeneous transformation matrix representing a placement in 3-D space.
/// Row-major: row `i`, column `j` is `matrix[i][j]`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Location {
    pub matrix: [[f64; 4]; 4],
}

impl Location {
    /// Return the identity transformation (no translation, no rotation).
    pub fn identity() -> Self {
        Self {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Return a pure-translation transformation by vector `v = [tx, ty, tz]`.
    pub fn from_translation(v: [f64; 3]) -> Self {
        Self {
            matrix: [
                [1.0, 0.0, 0.0, v[0]],
                [0.0, 1.0, 0.0, v[1]],
                [0.0, 0.0, 1.0, v[2]],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    /// Compose `self` with `other` by standard 4×4 matrix multiplication.
    /// The result represents first applying `self`, then `other`.
    pub fn compose(&self, other: &Location) -> Location {
        let a = &self.matrix;
        let b = &other.matrix;
        let mut m = [[0.0f64; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                let mut s = 0.0f64;
                for k in 0..4 {
                    s += a[i][k] * b[k][j];
                }
                m[i][j] = s;
            }
        }
        Location { matrix: m }
    }

    /// Return `true` if this location is (within floating-point equality) the identity.
    pub fn is_identity(&self) -> bool {
        let id = Self::identity();
        for i in 0..4 {
            for j in 0..4 {
                if (self.matrix[i][j] - id.matrix[i][j]).abs() > f64::EPSILON {
                    return false;
                }
            }
        }
        true
    }
}

// occt: XCAFDoc_AssemblyGraph
/// A node in the assembly graph, analogous to a labelled entry in an XDE document.
/// Each node carries a human-readable label, its own placement `Location`, and an
/// ordered list of child node indices into the owning `AssemblyGraph`.
#[derive(Clone, Debug)]
pub struct AssemblyNode {
    pub label: String,
    pub location: Location,
    pub children: Vec<usize>,
}

impl AssemblyNode {
    /// Create a new leaf node with the given label, an identity location, and no children.
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_owned(),
            location: Location::identity(),
            children: Vec::new(),
        }
    }

    /// Append the index `i` as a child of this node.
    pub fn add_child(&mut self, i: usize) {
        self.children.push(i);
    }

    /// Return `true` if this node has no children (i.e. it is a leaf / part node).
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

// occt: XCAFDoc_AssemblyGraph
/// A directed acyclic graph of `AssemblyNode` entries, mirroring the XDE assembly tree
/// maintained by `XCAFDoc_AssemblyGraph`.  Nodes are stored in a flat `Vec`; edges are
/// expressed as child-index lists inside each node.  Multiple roots are supported.
pub struct AssemblyGraph {
    pub nodes: Vec<AssemblyNode>,
    pub roots: Vec<usize>,
}

impl AssemblyGraph {
    /// Create an empty assembly graph with no nodes and no roots.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            roots: Vec::new(),
        }
    }

    /// Append `n` to the node list and return its index.
    pub fn add_node(&mut self, n: AssemblyNode) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(n);
        idx
    }

    /// Mark node `i` as a root of the graph.
    pub fn add_root(&mut self, i: usize) {
        self.roots.push(i);
    }

    /// Return the maximum depth of the assembly tree.
    /// An empty graph has depth 0; a graph consisting of only roots has depth 1.
    pub fn depth(&self) -> usize {
        fn node_depth(graph: &AssemblyGraph, idx: usize) -> usize {
            let node = &graph.nodes[idx];
            if node.is_leaf() {
                1
            } else {
                1 + node
                    .children
                    .iter()
                    .map(|&c| node_depth(graph, c))
                    .max()
                    .unwrap_or(0)
            }
        }

        self.roots
            .iter()
            .map(|&r| node_depth(self, r))
            .max()
            .unwrap_or(0)
    }

    /// Return the total number of nodes in the graph.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_is_identity() {
        assert!(Location::identity().is_identity());
    }

    #[test]
    fn translation_is_not_identity() {
        let loc = Location::from_translation([1.0, 0.0, 0.0]);
        assert!(!loc.is_identity());
    }

    #[test]
    fn compose_translation() {
        let a = Location::from_translation([1.0, 0.0, 0.0]);
        let b = Location::from_translation([0.0, 2.0, 0.0]);
        let c = a.compose(&b);
        // combined translation should be [1, 2, 0]
        assert!((c.matrix[0][3] - 1.0).abs() < 1e-12);
        assert!((c.matrix[1][3] - 2.0).abs() < 1e-12);
        assert!((c.matrix[2][3] - 0.0).abs() < 1e-12);
    }

    #[test]
    fn leaf_and_child() {
        let mut parent = AssemblyNode::new("root");
        assert!(parent.is_leaf());
        parent.add_child(1);
        assert!(!parent.is_leaf());
    }

    #[test]
    fn graph_depth_and_count() {
        let mut g = AssemblyGraph::new();
        let leaf = g.add_node(AssemblyNode::new("leaf"));
        let mut mid = AssemblyNode::new("mid");
        mid.add_child(leaf);
        let mid_idx = g.add_node(mid);
        let mut root = AssemblyNode::new("root");
        root.add_child(mid_idx);
        let root_idx = g.add_node(root);
        g.add_root(root_idx);

        assert_eq!(g.node_count(), 3);
        assert_eq!(g.depth(), 3);
    }
}
