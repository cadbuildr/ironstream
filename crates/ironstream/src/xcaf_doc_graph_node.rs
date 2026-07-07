// FILE: xcaf_doc_graph_node.rs
// occt: XCAFDoc_GraphNode
//
// A graph-node attribute: each node keeps sequences of father and child
// graph nodes, allowing a multi-relation graph between labels.
// TDF plumbing (label / attribute storage) is modeled locally.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Default GraphNode GUID (from OCCT GetDefaultGraphID).
pub const DEFAULT_GRAPH_ID: &str = "efd212f5-6dfd-11d4-b9c8-0060b0ee281b";

pub type GraphNodeHandle = Rc<RefCell<XCAFDocGraphNode>>;

/// Local model of a TDF_Label: attributes keyed by GUID.
#[derive(Default, Clone)]
pub struct TdfLabel {
    attrs: Rc<RefCell<HashMap<String, GraphNodeHandle>>>,
}

impl TdfLabel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn find_attribute(&self, guid: &str) -> Option<GraphNodeHandle> {
        self.attrs.borrow().get(guid).cloned()
    }

    pub fn add_attribute(&self, attr: GraphNodeHandle) {
        let guid = attr.borrow().id().to_string();
        self.attrs.borrow_mut().insert(guid, attr);
    }
}

/// XCAFDoc_GraphNode: node of a label graph with father/child links.
pub struct XCAFDocGraphNode {
    graph_id: String,
    fathers: Vec<GraphNodeHandle>,
    children: Vec<GraphNodeHandle>,
}

impl XCAFDocGraphNode {
    /// OCCT default ctor (graph ID defaults to the class GUID).
    pub fn new() -> GraphNodeHandle {
        Rc::new(RefCell::new(XCAFDocGraphNode {
            graph_id: DEFAULT_GRAPH_ID.to_string(),
            fathers: Vec::new(),
            children: Vec::new(),
        }))
    }

    /// OCCT GetDefaultGraphID.
    pub fn get_default_graph_id() -> &'static str {
        DEFAULT_GRAPH_ID
    }

    /// OCCT static Find.
    pub fn find(label: &TdfLabel) -> Option<GraphNodeHandle> {
        label.find_attribute(DEFAULT_GRAPH_ID)
    }

    /// OCCT static Set(L): finds or creates a GraphNode with default ID.
    pub fn set(label: &TdfLabel) -> GraphNodeHandle {
        Self::set_with_id(label, DEFAULT_GRAPH_ID)
    }

    /// OCCT static Set(L, explicitID): finds or creates with explicit ID.
    pub fn set_with_id(label: &TdfLabel, explicit_id: &str) -> GraphNodeHandle {
        if let Some(existing) = label.find_attribute(explicit_id) {
            return existing;
        }
        let gn = Self::new();
        gn.borrow_mut().set_graph_id(explicit_id);
        label.add_attribute(gn.clone());
        gn
    }

    /// OCCT SetGraphID.
    pub fn set_graph_id(&mut self, explicit_id: &str) {
        self.graph_id = explicit_id.to_string();
    }

    /// OCCT ID: the graph ID of this node.
    pub fn id(&self) -> &str {
        &self.graph_id
    }

    /// OCCT NbFathers.
    pub fn nb_fathers(&self) -> usize {
        self.fathers.len()
    }

    /// OCCT NbChildren.
    pub fn nb_children(&self) -> usize {
        self.children.len()
    }

    /// OCCT GetFather (1-based index).
    pub fn get_father(&self, findex: usize) -> GraphNodeHandle {
        self.fathers[findex - 1].clone()
    }

    /// OCCT GetChild (1-based index).
    pub fn get_child(&self, chindex: usize) -> GraphNodeHandle {
        self.children[chindex - 1].clone()
    }

    /// OCCT FatherIndex: 1-based index of F in fathers, 0 if absent.
    pub fn father_index(&self, f: &GraphNodeHandle) -> usize {
        for (i, fa) in self.fathers.iter().enumerate() {
            if Rc::ptr_eq(fa, f) {
                return i + 1;
            }
        }
        0
    }

    /// OCCT ChildIndex: 1-based index of Ch in children, 0 if absent.
    pub fn child_index(&self, ch: &GraphNodeHandle) -> usize {
        for (i, c) in self.children.iter().enumerate() {
            if Rc::ptr_eq(c, ch) {
                return i + 1;
            }
        }
        0
    }

    /// OCCT IsFather: true if Ch is a child of this node.
    pub fn is_father(&self, ch: &GraphNodeHandle) -> bool {
        self.child_index(ch) != 0
    }

    /// OCCT IsChild: true if F is a father of this node.
    pub fn is_child(&self, f: &GraphNodeHandle) -> bool {
        self.father_index(f) != 0
    }
}

/// OCCT SetFather: appends F to the fathers sequence, returns 1-based index.
pub fn set_father(node: &GraphNodeHandle, f: &GraphNodeHandle) -> usize {
    let mut n = node.borrow_mut();
    n.fathers.push(f.clone());
    n.fathers.len()
}

/// OCCT SetChild: appends Ch to the children sequence, returns 1-based index.
pub fn set_child(node: &GraphNodeHandle, ch: &GraphNodeHandle) -> usize {
    let mut n = node.borrow_mut();
    n.children.push(ch.clone());
    n.children.len()
}

/// OCCT UnSetFatherlink: one-way removal of F from node's fathers.
pub fn unset_fatherlink(node: &GraphNodeHandle, f: &GraphNodeHandle) {
    let idx = node.borrow().father_index(f);
    if idx != 0 {
        node.borrow_mut().fathers.remove(idx - 1);
    }
}

/// OCCT UnSetChildlink: one-way removal of Ch from node's children.
pub fn unset_childlink(node: &GraphNodeHandle, ch: &GraphNodeHandle) {
    let idx = node.borrow().child_index(ch);
    if idx != 0 {
        node.borrow_mut().children.remove(idx - 1);
    }
}

/// OCCT UnSetFather: two-way removal of the father/child link.
pub fn unset_father(node: &GraphNodeHandle, f: &GraphNodeHandle) {
    if node.borrow().father_index(f) != 0 {
        unset_childlink(f, node);
        unset_fatherlink(node, f);
    }
}

/// OCCT UnSetChild: two-way removal of the child/father link.
pub fn unset_child(node: &GraphNodeHandle, ch: &GraphNodeHandle) {
    if node.borrow().child_index(ch) != 0 {
        unset_fatherlink(ch, node);
        unset_childlink(node, ch);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Creates a two-way father->child link, as XCAFDoc callers do.
    fn link(father: &GraphNodeHandle, child: &GraphNodeHandle) {
        set_child(father, child);
        set_father(child, father);
    }

    #[test]
    fn test_default_graph_id() {
        assert_eq!(
            XCAFDocGraphNode::get_default_graph_id(),
            "efd212f5-6dfd-11d4-b9c8-0060b0ee281b"
        );
        let gn = XCAFDocGraphNode::new();
        assert_eq!(gn.borrow().id(), DEFAULT_GRAPH_ID);
    }

    #[test]
    fn test_set_on_label_finds_or_creates() {
        let label = TdfLabel::new();
        assert!(XCAFDocGraphNode::find(&label).is_none());
        let gn1 = XCAFDocGraphNode::set(&label);
        let gn2 = XCAFDocGraphNode::set(&label);
        assert!(Rc::ptr_eq(&gn1, &gn2));
        assert!(Rc::ptr_eq(
            &XCAFDocGraphNode::find(&label).unwrap(),
            &gn1
        ));
    }

    #[test]
    fn test_set_with_explicit_id() {
        let label = TdfLabel::new();
        let gn = XCAFDocGraphNode::set_with_id(&label, "my-guid");
        assert_eq!(gn.borrow().id(), "my-guid");
        // Default-ID lookup must not see it.
        assert!(XCAFDocGraphNode::find(&label).is_none());
        assert!(label.find_attribute("my-guid").is_some());
    }

    #[test]
    fn test_father_child_links_and_indices() {
        let f = XCAFDocGraphNode::new();
        let c1 = XCAFDocGraphNode::new();
        let c2 = XCAFDocGraphNode::new();
        link(&f, &c1);
        link(&f, &c2);

        assert_eq!(f.borrow().nb_children(), 2);
        assert_eq!(c1.borrow().nb_fathers(), 1);
        assert_eq!(f.borrow().child_index(&c1), 1);
        assert_eq!(f.borrow().child_index(&c2), 2);
        assert_eq!(c2.borrow().father_index(&f), 1);

        assert!(f.borrow().is_father(&c1));
        assert!(c1.borrow().is_child(&f));
        assert!(!c1.borrow().is_father(&f));

        assert!(Rc::ptr_eq(&f.borrow().get_child(2), &c2));
        assert!(Rc::ptr_eq(&c1.borrow().get_father(1), &f));
    }

    #[test]
    fn test_unset_child_removes_both_links() {
        let f = XCAFDocGraphNode::new();
        let c = XCAFDocGraphNode::new();
        link(&f, &c);
        assert!(f.borrow().is_father(&c));

        unset_child(&f, &c);
        assert_eq!(f.borrow().nb_children(), 0);
        assert_eq!(c.borrow().nb_fathers(), 0);
    }

    #[test]
    fn test_unset_father_removes_both_links() {
        let f = XCAFDocGraphNode::new();
        let c = XCAFDocGraphNode::new();
        link(&f, &c);

        unset_father(&c, &f);
        assert_eq!(f.borrow().nb_children(), 0);
        assert_eq!(c.borrow().nb_fathers(), 0);
    }

    #[test]
    fn test_unset_missing_link_is_noop() {
        let f = XCAFDocGraphNode::new();
        let c = XCAFDocGraphNode::new();
        // No link established: unset must not panic or alter anything.
        unset_child(&f, &c);
        unset_father(&c, &f);
        assert_eq!(f.borrow().nb_children(), 0);
        assert_eq!(c.borrow().nb_fathers(), 0);
        assert_eq!(f.borrow().child_index(&c), 0);
    }
}
