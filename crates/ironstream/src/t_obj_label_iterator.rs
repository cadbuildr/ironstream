// FILE: t_obj_label_iterator.rs
// occt: TObj_LabelIterator

//! This class is a basis for OCAF based iterators.
//! Faithful port of `TObj_LabelIterator` (.hxx + .cxx): the base keeps
//! the current node label, the current object and a TDF_ChildIterator;
//! `More()` is "current node is not null", `Next()` nullifies the node
//! and object then calls the subclass `MakeStep()`, `Value()`/
//! `LabelValue()` expose the current object/label.
//!
//! The OCAF label tree and child iterator are modeled locally: a label
//! is a node with an entry, optional attached object and children; the
//! child iterator supports recursive and non-recursive traversal plus
//! the NextBrother step used by subclasses.

use std::rc::Rc;

/// Local stand-in for `TObj_Object`.
#[derive(Debug)]
pub struct TObjObjectRecLi {
    pub name: String,
}

pub type HandleTObjObjectLi = Rc<TObjObjectRecLi>;

/// Local stand-in for a TDF label node.
#[derive(Debug)]
pub struct OcafLabelNodeLi {
    pub entry: String,
    pub object: Option<HandleTObjObjectLi>,
    pub children: Vec<Rc<OcafLabelNodeLi>>,
}

impl OcafLabelNodeLi {
    pub fn leaf(entry: &str, object: Option<HandleTObjObjectLi>) -> Rc<Self> {
        Rc::new(OcafLabelNodeLi { entry: entry.to_string(), object, children: Vec::new() })
    }

    pub fn node(
        entry: &str,
        object: Option<HandleTObjObjectLi>,
        children: Vec<Rc<OcafLabelNodeLi>>,
    ) -> Rc<Self> {
        Rc::new(OcafLabelNodeLi { entry: entry.to_string(), object, children })
    }
}

/// Local model of `TDF_ChildIterator`: pre-order walk over the children
/// of a root label (the root itself is NOT visited), with optional
/// recursion and the NextBrother step (skip the current subtree).
pub struct TdfChildIteratorLi {
    /// Flattened pre-order visit list, paired with each node's depth.
    visit: Vec<(Rc<OcafLabelNodeLi>, usize)>,
    cursor: usize,
    recursive: bool,
}

impl TdfChildIteratorLi {
    pub fn new(root: &Rc<OcafLabelNodeLi>, recursive: bool) -> Self {
        let mut visit = Vec::new();
        fn collect(
            node: &Rc<OcafLabelNodeLi>,
            depth: usize,
            out: &mut Vec<(Rc<OcafLabelNodeLi>, usize)>,
        ) {
            for child in &node.children {
                out.push((child.clone(), depth));
                collect(child, depth + 1, out);
            }
        }
        collect(root, 0, &mut visit);
        if !recursive {
            visit.retain(|(_, d)| *d == 0);
        }
        TdfChildIteratorLi { visit, cursor: 0, recursive }
    }

    pub fn more(&self) -> bool {
        self.cursor < self.visit.len()
    }

    pub fn value(&self) -> Rc<OcafLabelNodeLi> {
        self.visit[self.cursor].0.clone()
    }

    /// TDF_ChildIterator::Next — next label in pre-order.
    pub fn next(&mut self) {
        self.cursor += 1;
    }

    /// TDF_ChildIterator::NextBrother — skip the current subtree.
    pub fn next_brother(&mut self) {
        if !self.more() {
            return;
        }
        if !self.recursive {
            self.cursor += 1;
            return;
        }
        let depth = self.visit[self.cursor].1;
        self.cursor += 1;
        while self.cursor < self.visit.len() && self.visit[self.cursor].1 > depth {
            self.cursor += 1;
        }
    }
}

/// The state fields of TObj_LabelIterator (protected in C++).
pub struct TObjLabelIteratorCore {
    /// Current node (None models a null TDF_Label).
    pub node: Option<Rc<OcafLabelNodeLi>>,
    /// Current object.
    pub object: Option<HandleTObjObjectLi>,
    /// OCAF child iterator.
    pub iterator: TdfChildIteratorLi,
}

impl TObjLabelIteratorCore {
    /// Protected constructor: Init by label + recursion flag.
    pub fn init(root: &Rc<OcafLabelNodeLi>, recursive: bool) -> Self {
        TObjLabelIteratorCore {
            node: None,
            object: None,
            iterator: TdfChildIteratorLi::new(root, recursive),
        }
    }

    /// TObj_LabelIterator::More.
    pub fn more(&self) -> bool {
        self.node.is_some()
    }

    /// TObj_LabelIterator::Value.
    pub fn value(&self) -> Option<HandleTObjObjectLi> {
        self.object.clone()
    }

    /// TObj_LabelIterator::LabelValue.
    pub fn label_value(&self) -> Option<Rc<OcafLabelNodeLi>> {
        self.node.clone()
    }
}

/// The abstract MakeStep of TObj_LabelIterator subclasses.
pub trait TObjLabelIteratorStepLi {
    fn core(&mut self) -> &mut TObjLabelIteratorCore;

    /// Shifts iterator to the next object.
    fn make_step(&mut self);

    /// TObj_LabelIterator::Next — nullify then step.
    fn next(&mut self) {
        {
            let core = self.core();
            core.object = None;
            core.node = None;
        }
        self.make_step();
    }
}

/// A concrete label iterator visiting labels carrying objects
/// (the simplest MakeStep, matching the base-class usage pattern).
pub struct ObjectBearingLabelIteratorLi {
    core: TObjLabelIteratorCore,
}

impl ObjectBearingLabelIteratorLi {
    pub fn new(root: &Rc<OcafLabelNodeLi>, recursive: bool) -> Self {
        let mut it = ObjectBearingLabelIteratorLi {
            core: TObjLabelIteratorCore::init(root, recursive),
        };
        it.make_step();
        it
    }

    pub fn more(&self) -> bool {
        self.core.more()
    }

    pub fn value(&self) -> Option<HandleTObjObjectLi> {
        self.core.value()
    }

    pub fn label_value(&self) -> Option<Rc<OcafLabelNodeLi>> {
        self.core.label_value()
    }
}

impl TObjLabelIteratorStepLi for ObjectBearingLabelIteratorLi {
    fn core(&mut self) -> &mut TObjLabelIteratorCore {
        &mut self.core
    }

    fn make_step(&mut self) {
        while self.core.iterator.more() && self.core.node.is_none() {
            let label = self.core.iterator.value();
            if let Some(obj) = &label.object {
                self.core.object = Some(obj.clone());
                self.core.node = Some(label.clone());
            }
            self.core.iterator.next();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn obj(name: &str) -> HandleTObjObjectLi {
        Rc::new(TObjObjectRecLi { name: name.into() })
    }

    fn sample_tree() -> Rc<OcafLabelNodeLi> {
        // 0:1 (root, not visited)
        //   0:1:1 [objA]
        //     0:1:1:1 [objB]
        //   0:1:2 (no object)
        //   0:1:3 [objC]
        OcafLabelNodeLi::node(
            "0:1",
            None,
            vec![
                OcafLabelNodeLi::node(
                    "0:1:1",
                    Some(obj("A")),
                    vec![OcafLabelNodeLi::leaf("0:1:1:1", Some(obj("B")))],
                ),
                OcafLabelNodeLi::leaf("0:1:2", None),
                OcafLabelNodeLi::leaf("0:1:3", Some(obj("C"))),
            ],
        )
    }

    #[test]
    fn non_recursive_visits_direct_children_only() {
        let root = sample_tree();
        let mut it = ObjectBearingLabelIteratorLi::new(&root, false);
        let mut seen = Vec::new();
        while it.more() {
            seen.push(it.value().unwrap().name.clone());
            it.next();
        }
        assert_eq!(seen, vec!["A", "C"], "0:1:1:1 not visited, 0:1:2 skipped");
    }

    #[test]
    fn recursive_visits_subtree() {
        let root = sample_tree();
        let mut it = ObjectBearingLabelIteratorLi::new(&root, true);
        let mut seen = Vec::new();
        while it.more() {
            seen.push(it.value().unwrap().name.clone());
            it.next();
        }
        assert_eq!(seen, vec!["A", "B", "C"]);
    }

    #[test]
    fn label_value_matches_object() {
        let root = sample_tree();
        let it = ObjectBearingLabelIteratorLi::new(&root, false);
        assert!(it.more());
        assert_eq!(it.label_value().unwrap().entry, "0:1:1");
        assert_eq!(it.value().unwrap().name, "A");
    }

    #[test]
    fn next_nullifies_before_step() {
        let root = OcafLabelNodeLi::node("0:2", None, vec![OcafLabelNodeLi::leaf("0:2:1", Some(obj("only")))]);
        let mut it = ObjectBearingLabelIteratorLi::new(&root, false);
        assert!(it.more());
        it.next();
        assert!(!it.more(), "exhausted after single item");
        assert!(it.value().is_none(), "object nullified by Next");
        assert!(it.label_value().is_none());
    }

    #[test]
    fn next_brother_skips_subtree() {
        let root = sample_tree();
        let mut child_it = TdfChildIteratorLi::new(&root, true);
        assert_eq!(child_it.value().entry, "0:1:1");
        child_it.next_brother();
        assert_eq!(child_it.value().entry, "0:1:2", "0:1:1:1 skipped");
    }
}
