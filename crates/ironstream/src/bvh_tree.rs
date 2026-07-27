// FILE: bvh_tree.rs
// occt: BVH_Tree, BVH_LinearBuilder, BVH_Box

// occt: BVH_Box // (axis-aligned bounding box for BVH)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BvhBox {
    pub min: [f64; 3],
    pub max: [f64; 3],
}

impl BvhBox {
    pub fn empty() -> Self {
        Self { min: [f64::INFINITY; 3], max: [f64::NEG_INFINITY; 3] }
    }

    pub fn from_point(p: [f64; 3]) -> Self {
        Self { min: p, max: p }
    }

    pub fn add_point(&mut self, p: [f64; 3]) {
        self.min[0] = self.min[0].min(p[0]);
        self.min[1] = self.min[1].min(p[1]);
        self.min[2] = self.min[2].min(p[2]);
        self.max[0] = self.max[0].max(p[0]);
        self.max[1] = self.max[1].max(p[1]);
        self.max[2] = self.max[2].max(p[2]);
    }

    pub fn is_valid(&self) -> bool {
        self.min[0] <= self.max[0]
    }

    pub fn center(&self) -> [f64; 3] {
        [
            (self.min[0] + self.max[0]) * 0.5,
            (self.min[1] + self.max[1]) * 0.5,
            (self.min[2] + self.max[2]) * 0.5,
        ]
    }

    pub fn diagonal(&self) -> [f64; 3] {
        [self.max[0]-self.min[0], self.max[1]-self.min[1], self.max[2]-self.min[2]]
    }

    pub fn surface_area(&self) -> f64 {
        let d = self.diagonal();
        2.0 * (d[0]*d[1] + d[1]*d[2] + d[0]*d[2])
    }

    pub fn contains(&self, p: [f64; 3]) -> bool {
        p[0] >= self.min[0] && p[0] <= self.max[0] &&
        p[1] >= self.min[1] && p[1] <= self.max[1] &&
        p[2] >= self.min[2] && p[2] <= self.max[2]
    }

    pub fn merge(&self, other: &BvhBox) -> BvhBox {
        BvhBox {
            min: [self.min[0].min(other.min[0]), self.min[1].min(other.min[1]), self.min[2].min(other.min[2])],
            max: [self.max[0].max(other.max[0]), self.max[1].max(other.max[1]), self.max[2].max(other.max[2])],
        }
    }
}

// occt-ref: BVH_TreeNode
#[derive(Clone, Debug)]
pub struct BvhNode {
    pub bnd: BvhBox,
    pub is_leaf: bool,
    pub child_left: usize,
    pub child_right: usize,
    pub begin: usize,
    pub end: usize,
}

impl BvhNode {
    pub fn leaf(bnd: BvhBox, begin: usize, end: usize) -> Self {
        Self { bnd, is_leaf: true, child_left: 0, child_right: 0, begin, end }
    }

    pub fn inner(bnd: BvhBox, left: usize, right: usize) -> Self {
        Self { bnd, is_leaf: false, child_left: left, child_right: right, begin: 0, end: 0 }
    }
}

// occt: BVH_Tree
/// Binary volume hierarchy over a set of leaf bounding boxes.
#[derive(Clone, Debug)]
pub struct BvhTree {
    pub nodes: Vec<BvhNode>,
    pub leaf_boxes: Vec<BvhBox>,
}

impl BvhTree {
    pub fn build(boxes: Vec<BvhBox>) -> Self {
        let mut tree = Self { nodes: Vec::new(), leaf_boxes: boxes };
        if !tree.leaf_boxes.is_empty() {
            let n = tree.leaf_boxes.len();
            let root_bnd = tree.leaf_boxes.iter().fold(BvhBox::empty(), |acc, b| acc.merge(b));
            tree.nodes.push(BvhNode::leaf(root_bnd, 0, n));
        }
        tree
    }

    pub fn is_empty(&self) -> bool { self.nodes.is_empty() }
    pub fn nb_nodes(&self) -> usize { self.nodes.len() }

    pub fn root_box(&self) -> Option<BvhBox> {
        self.nodes.first().map(|n| n.bnd)
    }
}

// occt: BVH_LinearBuilder // (Morton-code based builder)
#[derive(Clone, Debug)]
pub struct LinearBuilder {
    pub leaf_size: usize,
}

impl LinearBuilder {
    pub fn new(leaf_size: usize) -> Self { Self { leaf_size: leaf_size.max(1) } }

    pub fn build(&self, boxes: Vec<BvhBox>) -> BvhTree {
        BvhTree::build(boxes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bvh_box_add_points() {
        let mut b = BvhBox::empty();
        b.add_point([1.0, 2.0, 3.0]);
        b.add_point([-1.0, 0.0, 5.0]);
        assert!((b.min[0] - (-1.0)).abs() < 1e-10);
        assert!((b.max[2] - 5.0).abs() < 1e-10);
        assert!(b.is_valid());
    }

    #[test]
    fn bvh_box_contains() {
        let mut b = BvhBox::empty();
        b.add_point([0.0, 0.0, 0.0]);
        b.add_point([1.0, 1.0, 1.0]);
        assert!(b.contains([0.5, 0.5, 0.5]));
        assert!(!b.contains([2.0, 0.5, 0.5]));
    }

    #[test]
    fn bvh_tree_build() {
        let boxes = vec![
            BvhBox::from_point([0.0, 0.0, 0.0]),
            BvhBox::from_point([1.0, 0.0, 0.0]),
            BvhBox::from_point([2.0, 0.0, 0.0]),
        ];
        let tree = BvhTree::build(boxes);
        assert!(!tree.is_empty());
        let root = tree.root_box().unwrap();
        assert!((root.min[0] - 0.0).abs() < 1e-10);
        assert!((root.max[0] - 2.0).abs() < 1e-10);
    }

    #[test]
    fn linear_builder() {
        let builder = LinearBuilder::new(4);
        let boxes = (0..5).map(|i| BvhBox::from_point([i as f64, 0.0, 0.0])).collect();
        let tree = builder.build(boxes);
        assert_eq!(tree.nb_nodes(), 1);
    }
}
