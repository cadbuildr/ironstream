// FILE: std_l_persistent_tree_node.rs
// occt: StdLPersistent_TreeNode

//! Persistent tree-node attribute, following StdLPersistent_TreeNode.hxx/.cxx.
//!
//! The persistent node stores a reference to its first child and to its next
//! sibling plus a tree GUID (in a lazily allocated `dynamic` block). During
//! import the persistent sibling chain is converted into the transient
//! attribute's child list and the chain references are released.
//!
//! External plumbing (StdObjMgt read/write data streams, the transient
//! TDataStd_TreeNode attribute, Standard_GUID) is modelled with local helper
//! types; the node's Read/Write/PChildren/CreateAttribute/ImportAttribute
//! behaviour mirrors the C++ source.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// ---------------------------------------------------------------------------
// Local plumbing
// ---------------------------------------------------------------------------

/// Models Standard_GUID.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Guid(pub String);

/// Models the transient TDataStd_TreeNode attribute.
#[derive(Debug, Default)]
pub struct TransientTreeNode {
    tree_id: Guid,
    children: Vec<Rc<RefCell<TransientTreeNode>>>,
}

impl TransientTreeNode {
    pub fn set_tree_id(&mut self, id: Guid) {
        self.tree_id = id;
    }

    pub fn tree_id(&self) -> &Guid {
        &self.tree_id
    }

    /// TDataStd_TreeNode::Append.
    pub fn append(&mut self, child: Rc<RefCell<TransientTreeNode>>) {
        self.children.push(child);
    }

    pub fn nb_children(&self) -> usize {
        self.children.len()
    }

    pub fn child(&self, index: usize) -> Rc<RefCell<TransientTreeNode>> {
        Rc::clone(&self.children[index])
    }
}

/// Shared reference to a persistent tree node.
pub type NodeRef = Rc<RefCell<StdLPersistentTreeNode>>;

/// One value in a persistent data stream.
#[derive(Clone, Debug)]
pub enum StreamItem {
    /// Persistent reference (None models a null handle).
    Ref(Option<NodeRef>),
    Guid(Guid),
}

/// Models StdObjMgt_ReadData: a sequential stream of persistent values.
#[derive(Debug, Default)]
pub struct ReadData {
    items: VecDeque<StreamItem>,
}

impl ReadData {
    pub fn new(items: Vec<StreamItem>) -> Self {
        Self { items: items.into() }
    }

    /// Reads a persistent reference (operator>> for handles).
    pub fn read_ref(&mut self) -> Option<NodeRef> {
        match self.items.pop_front() {
            Some(StreamItem::Ref(r)) => r,
            other => panic!("expected persistent reference, got {:?}", other),
        }
    }

    /// Reads a GUID (operator>> for Standard_GUID).
    pub fn read_guid(&mut self) -> Guid {
        match self.items.pop_front() {
            Some(StreamItem::Guid(g)) => g,
            other => panic!("expected GUID, got {:?}", other),
        }
    }
}

/// Models StdObjMgt_WriteData: collects written persistent values.
#[derive(Debug, Default)]
pub struct WriteData {
    items: Vec<StreamItem>,
}

impl WriteData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn write_ref(&mut self, r: Option<NodeRef>) {
        self.items.push(StreamItem::Ref(r));
    }

    pub fn write_guid(&mut self, g: Guid) {
        self.items.push(StreamItem::Guid(g));
    }

    pub fn items(&self) -> &[StreamItem] {
        &self.items
    }
}

// ---------------------------------------------------------------------------
// The persistent tree node (StdLPersistent_TreeNode)
// ---------------------------------------------------------------------------

/// The lazily allocated `dynamic` block of the node.
#[derive(Debug, Default)]
struct DynamicData {
    first: Option<NodeRef>,
    tree_id: Guid,
}

/// Persistent tree-node attribute.
#[derive(Debug, Default)]
pub struct StdLPersistentTreeNode {
    dynamic_data: Option<DynamicData>,
    next: Option<NodeRef>,
    transient: Option<Rc<RefCell<TransientTreeNode>>>,
}

impl StdLPersistentTreeNode {
    pub fn new() -> Self {
        Self::default()
    }

    /// Reads persistent data: First, Next, TreeID. Mirrors Read.
    pub fn read(&mut self, read_data: &mut ReadData) {
        let first = read_data.read_ref();
        self.next = read_data.read_ref();
        let tree_id = read_data.read_guid();
        self.dynamic_data = Some(DynamicData { first, tree_id });
    }

    /// Writes persistent data: First, Next, TreeID. Mirrors Write.
    pub fn write(&self, write_data: &mut WriteData) {
        let dyn_data = self
            .dynamic_data
            .as_ref()
            .expect("Write requires the dynamic data block");
        write_data.write_ref(dyn_data.first.clone());
        write_data.write_ref(self.next.clone());
        write_data.write_guid(dyn_data.tree_id.clone());
    }

    /// Gets referenced child objects: Next always, First when the dynamic
    /// block exists (null handles included). Mirrors PChildren.
    pub fn p_children(&self) -> Vec<Option<NodeRef>> {
        let mut children = vec![self.next.clone()];
        if let Some(dyn_data) = &self.dynamic_data {
            children.push(dyn_data.first.clone());
        }
        children
    }

    /// Persistent type name. Mirrors PName.
    pub fn p_name(&self) -> &'static str {
        "PDataStd_TreeNode"
    }

    /// Creates the transient attribute and sets its tree ID.
    /// Mirrors CreateAttribute.
    pub fn create_attribute(&mut self) -> Rc<RefCell<TransientTreeNode>> {
        let transient = Rc::new(RefCell::new(TransientTreeNode::default()));
        let tree_id = self
            .dynamic_data
            .as_ref()
            .expect("CreateAttribute requires the dynamic data block")
            .tree_id
            .clone();
        transient.borrow_mut().set_tree_id(tree_id);
        self.transient = Some(Rc::clone(&transient));
        transient
    }

    /// Returns the transient attribute (if created).
    pub fn transient(&self) -> Option<Rc<RefCell<TransientTreeNode>>> {
        self.transient.clone()
    }

    /// Returns the next sibling reference.
    pub fn next(&self) -> Option<NodeRef> {
        self.next.clone()
    }

    /// Returns the first child reference (while the dynamic block exists).
    pub fn first_child(&self) -> Option<NodeRef> {
        self.dynamic_data.as_ref().and_then(|d| d.first.clone())
    }

    /// True while the dynamic block has not been released by import.
    pub fn has_dynamic_data(&self) -> bool {
        self.dynamic_data.is_some()
    }

    /// Imports the transient attribute: walks the persistent child chain,
    /// appends each child's transient to this node's transient, releases the
    /// chain references and the dynamic block. Mirrors ImportAttribute.
    pub fn import_attribute(&mut self) {
        if let Some(dyn_data) = self.dynamic_data.take() {
            let my_transient = self
                .transient
                .as_ref()
                .expect("ImportAttribute requires the transient attribute")
                .clone();
            let mut child = dyn_data.first;
            while let Some(curr) = child {
                {
                    let curr_ref = curr.borrow();
                    if let Some(child_transient) = &curr_ref.transient {
                        my_transient.borrow_mut().append(Rc::clone(child_transient));
                    }
                }
                // Advance and release the no longer needed sibling reference.
                let next = curr.borrow_mut().next.take();
                child = next;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_node() -> NodeRef {
        Rc::new(RefCell::new(StdLPersistentTreeNode::new()))
    }

    /// Builds a parent with two children linked as a sibling chain, all
    /// populated through read().
    fn build_family() -> (NodeRef, NodeRef, NodeRef) {
        let parent = new_node();
        let child1 = new_node();
        let child2 = new_node();

        // child2: no first child, no next, own tree id
        child2.borrow_mut().read(&mut ReadData::new(vec![
            StreamItem::Ref(None),
            StreamItem::Ref(None),
            StreamItem::Guid(Guid("tree-A".into())),
        ]));
        // child1: no first child, next = child2
        child1.borrow_mut().read(&mut ReadData::new(vec![
            StreamItem::Ref(None),
            StreamItem::Ref(Some(Rc::clone(&child2))),
            StreamItem::Guid(Guid("tree-A".into())),
        ]));
        // parent: first = child1, no next
        parent.borrow_mut().read(&mut ReadData::new(vec![
            StreamItem::Ref(Some(Rc::clone(&child1))),
            StreamItem::Ref(None),
            StreamItem::Guid(Guid("tree-A".into())),
        ]));
        (parent, child1, child2)
    }

    #[test]
    fn test_pname() {
        assert_eq!(StdLPersistentTreeNode::new().p_name(), "PDataStd_TreeNode");
    }

    #[test]
    fn test_read_stores_fields() {
        let (parent, child1, _child2) = build_family();
        let p = parent.borrow();
        assert!(p.has_dynamic_data());
        assert!(p.next().is_none());
        assert!(Rc::ptr_eq(&p.first_child().unwrap(), &child1));
    }

    #[test]
    fn test_write_roundtrip() {
        let (parent, child1, _child2) = build_family();
        let mut wd = WriteData::new();
        parent.borrow().write(&mut wd);
        let items = wd.items();
        assert_eq!(items.len(), 3);
        match &items[0] {
            StreamItem::Ref(Some(r)) => assert!(Rc::ptr_eq(r, &child1)),
            other => panic!("expected first-child ref, got {:?}", other),
        }
        match &items[1] {
            StreamItem::Ref(None) => {}
            other => panic!("expected null next ref, got {:?}", other),
        }
        match &items[2] {
            StreamItem::Guid(g) => assert_eq!(g, &Guid("tree-A".into())),
            other => panic!("expected GUID, got {:?}", other),
        }
    }

    #[test]
    fn test_p_children() {
        let (parent, child1, child2) = build_family();
        // Parent: [next (null), first (child1)]
        let pc = parent.borrow().p_children();
        assert_eq!(pc.len(), 2);
        assert!(pc[0].is_none());
        assert!(Rc::ptr_eq(pc[1].as_ref().unwrap(), &child1));
        // child1: [next (child2), first (null)]
        let cc = child1.borrow().p_children();
        assert_eq!(cc.len(), 2);
        assert!(Rc::ptr_eq(cc[0].as_ref().unwrap(), &child2));
        assert!(cc[1].is_none());
    }

    #[test]
    fn test_create_attribute_sets_tree_id() {
        let (parent, _c1, _c2) = build_family();
        let transient = parent.borrow_mut().create_attribute();
        assert_eq!(transient.borrow().tree_id(), &Guid("tree-A".into()));
        assert!(parent.borrow().transient().is_some());
    }

    #[test]
    fn test_import_attribute_builds_child_list() {
        let (parent, child1, child2) = build_family();
        let pt = parent.borrow_mut().create_attribute();
        let c1t = child1.borrow_mut().create_attribute();
        let c2t = child2.borrow_mut().create_attribute();

        parent.borrow_mut().import_attribute();

        // Transient children appended in chain order.
        assert_eq!(pt.borrow().nb_children(), 2);
        assert!(Rc::ptr_eq(&pt.borrow().child(0), &c1t));
        assert!(Rc::ptr_eq(&pt.borrow().child(1), &c2t));

        // The chain references and dynamic block are released.
        assert!(child1.borrow().next().is_none());
        assert!(!parent.borrow().has_dynamic_data());
    }

    #[test]
    fn test_import_attribute_skips_children_without_transient() {
        let (parent, _child1, child2) = build_family();
        let pt = parent.borrow_mut().create_attribute();
        // Only child2 gets a transient; child1 is skipped during import.
        let c2t = child2.borrow_mut().create_attribute();

        parent.borrow_mut().import_attribute();
        assert_eq!(pt.borrow().nb_children(), 1);
        assert!(Rc::ptr_eq(&pt.borrow().child(0), &c2t));
    }

    #[test]
    fn test_import_attribute_without_dynamic_is_noop() {
        let node = new_node();
        // No read() -> no dynamic block; import must not panic.
        node.borrow_mut().import_attribute();
        assert!(!node.borrow().has_dynamic_data());
    }
}
