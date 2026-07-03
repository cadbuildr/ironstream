// FILE: interface_graph.rs
// occt: Interface_Graph, Interface_ShareTool, Interface_EntityIterator,
//       Interface_GeneralLib

/// Status of an entity within a transfer graph.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum InterfaceEntityStatus {
    #[default]
    Unknown,
    Loaded,
    Invalid,
    Erroneous,
    Sharing,
    Shared,
    Root,
}

impl InterfaceEntityStatus {
    pub fn is_valid(&self) -> bool {
        matches!(self, Self::Loaded | Self::Sharing | Self::Shared | Self::Root)
    }
    pub fn is_root(&self) -> bool { *self == Self::Root }
}

/// A single entity node in the interface graph.
/// occt: Interface_Graph entity entry
#[derive(Clone, Debug)]
pub struct InterfaceEntityNode {
    pub entity_id: u32,
    pub entity_type: String,
    pub status: InterfaceEntityStatus,
    pub shared_by: Vec<u32>,   // entity IDs that share this one
    pub shares: Vec<u32>,      // entity IDs this one references
}

impl InterfaceEntityNode {
    pub fn new(entity_id: u32, entity_type: impl Into<String>) -> Self {
        Self {
            entity_id,
            entity_type: entity_type.into(),
            status: InterfaceEntityStatus::Loaded,
            shared_by: Vec::new(),
            shares: Vec::new(),
        }
    }

    pub fn add_share(&mut self, other_id: u32) {
        if !self.shares.contains(&other_id) { self.shares.push(other_id); }
    }

    pub fn add_shared_by(&mut self, parent_id: u32) {
        if !self.shared_by.contains(&parent_id) { self.shared_by.push(parent_id); }
    }

    pub fn nb_shares(&self) -> usize { self.shares.len() }
    pub fn nb_shared_by(&self) -> usize { self.shared_by.len() }
    pub fn is_root(&self) -> bool { self.shared_by.is_empty() }
    pub fn is_leaf(&self) -> bool { self.shares.is_empty() }
}

/// Interface transfer graph.
/// occt: Interface_Graph
#[derive(Clone, Debug, Default)]
pub struct InterfaceGraph {
    pub nodes: Vec<InterfaceEntityNode>,
}

impl InterfaceGraph {
    pub fn new() -> Self { Self::default() }

    pub fn add_entity(&mut self, mut node: InterfaceEntityNode) {
        if node.shared_by.is_empty() {
            node.status = InterfaceEntityStatus::Root;
        }
        self.nodes.push(node);
    }

    pub fn add_sharing(&mut self, from_id: u32, to_id: u32) {
        // Record that from_id shares to_id
        if let Some(from) = self.nodes.iter_mut().find(|n| n.entity_id == from_id) {
            from.add_share(to_id);
        }
        if let Some(to) = self.nodes.iter_mut().find(|n| n.entity_id == to_id) {
            to.add_shared_by(from_id);
            if to.status == InterfaceEntityStatus::Root {
                to.status = InterfaceEntityStatus::Shared;
            }
        }
    }

    pub fn find(&self, entity_id: u32) -> Option<&InterfaceEntityNode> {
        self.nodes.iter().find(|n| n.entity_id == entity_id)
    }

    pub fn roots(&self) -> Vec<&InterfaceEntityNode> {
        self.nodes.iter().filter(|n| n.is_root()).collect()
    }

    pub fn leaves(&self) -> Vec<&InterfaceEntityNode> {
        self.nodes.iter().filter(|n| n.is_leaf()).collect()
    }

    pub fn nb_entities(&self) -> usize { self.nodes.len() }

    pub fn entities_of_type(&self, t: &str) -> Vec<&InterfaceEntityNode> {
        self.nodes.iter().filter(|n| n.entity_type == t).collect()
    }

    /// Collect all entities transitively shared by `root_id`.
    pub fn all_shared(&self, root_id: u32) -> Vec<u32> {
        let mut visited = Vec::new();
        let mut stack = vec![root_id];
        while let Some(id) = stack.pop() {
            if visited.contains(&id) { continue; }
            visited.push(id);
            if let Some(node) = self.find(id) {
                for &child in &node.shares { stack.push(child); }
            }
        }
        visited.into_iter().filter(|&id| id != root_id).collect()
    }
}

/// Share tool: query sharing relations.
/// occt: Interface_ShareTool
#[derive(Clone, Debug)]
pub struct InterfaceShareTool<'a> {
    pub graph: &'a InterfaceGraph,
}

impl<'a> InterfaceShareTool<'a> {
    pub fn new(graph: &'a InterfaceGraph) -> Self { Self { graph } }

    pub fn shares(&self, entity_id: u32) -> Vec<u32> {
        self.graph.find(entity_id)
            .map(|n| n.shares.clone())
            .unwrap_or_default()
    }

    pub fn shared_by(&self, entity_id: u32) -> Vec<u32> {
        self.graph.find(entity_id)
            .map(|n| n.shared_by.clone())
            .unwrap_or_default()
    }

    pub fn is_shared(&self, entity_id: u32) -> bool {
        self.graph.find(entity_id)
            .map(|n| !n.shared_by.is_empty())
            .unwrap_or(false)
    }
}

/// Entity iterator: iterate over entity IDs.
/// occt: Interface_EntityIterator
#[derive(Clone, Debug)]
pub struct InterfaceEntityIterator {
    pub entity_ids: Vec<u32>,
    index: usize,
}

impl InterfaceEntityIterator {
    pub fn new(ids: Vec<u32>) -> Self { Self { entity_ids: ids, index: 0 } }

    pub fn more(&self) -> bool { self.index < self.entity_ids.len() }

    pub fn next(&mut self) -> Option<u32> {
        if self.more() {
            let id = self.entity_ids[self.index];
            self.index += 1;
            Some(id)
        } else {
            None
        }
    }

    pub fn reset(&mut self) { self.index = 0; }
    pub fn nb_entities(&self) -> usize { self.entity_ids.len() }
    pub fn start(&mut self) { self.index = 0; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_roots_leaves() {
        let mut g = InterfaceGraph::new();
        g.add_entity(InterfaceEntityNode::new(1, "Product"));
        g.add_entity(InterfaceEntityNode::new(2, "Shape"));
        g.add_entity(InterfaceEntityNode::new(3, "Surface"));
        g.add_sharing(1, 2);
        g.add_sharing(2, 3);
        assert_eq!(g.roots().len(), 1);
        assert_eq!(g.roots()[0].entity_id, 1);
        assert_eq!(g.leaves().len(), 1);
        assert_eq!(g.leaves()[0].entity_id, 3);
    }

    #[test]
    fn graph_all_shared() {
        let mut g = InterfaceGraph::new();
        g.add_entity(InterfaceEntityNode::new(1, "A"));
        g.add_entity(InterfaceEntityNode::new(2, "B"));
        g.add_entity(InterfaceEntityNode::new(3, "C"));
        g.add_sharing(1, 2);
        g.add_sharing(1, 3);
        let shared = g.all_shared(1);
        assert_eq!(shared.len(), 2);
        assert!(shared.contains(&2));
        assert!(shared.contains(&3));
    }

    #[test]
    fn share_tool() {
        let mut g = InterfaceGraph::new();
        g.add_entity(InterfaceEntityNode::new(10, "X"));
        g.add_entity(InterfaceEntityNode::new(20, "Y"));
        g.add_sharing(10, 20);
        let st = InterfaceShareTool::new(&g);
        assert_eq!(st.shares(10), vec![20]);
        assert!(st.is_shared(20));
        assert!(!st.is_shared(10));
    }

    #[test]
    fn entity_iterator() {
        let mut it = InterfaceEntityIterator::new(vec![5, 6, 7]);
        assert_eq!(it.nb_entities(), 3);
        let mut collected = Vec::new();
        while it.more() { collected.push(it.next().unwrap()); }
        assert_eq!(collected, vec![5, 6, 7]);
        assert!(!it.more());
        it.reset();
        assert!(it.more());
    }

    #[test]
    fn entities_of_type() {
        let mut g = InterfaceGraph::new();
        g.add_entity(InterfaceEntityNode::new(1, "Face"));
        g.add_entity(InterfaceEntityNode::new(2, "Face"));
        g.add_entity(InterfaceEntityNode::new(3, "Edge"));
        assert_eq!(g.entities_of_type("Face").len(), 2);
        assert_eq!(g.entities_of_type("Vertex").len(), 0);
    }
}
