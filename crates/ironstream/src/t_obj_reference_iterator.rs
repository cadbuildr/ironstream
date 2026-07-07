// FILE: t_obj_reference_iterator.rs
// occt: TObj_ReferenceIterator

//! This class provides an iterator by references of the object
//! (implements TObj_ObjectIterator interface).
//! Faithful port of `TObj_ReferenceIterator` (.hxx + .cxx): walks child
//! labels (optionally recursive), stops on labels carrying a
//! TObj_TReference attribute, resolves the referred object and filters
//! by type when a filter is given — labels whose referred object fails
//! the type test are skipped, but a reference to a NULL object is still
//! reported (per the C++ condition).
//!
//! Labels, reference attributes and typed objects are local models.

use std::rc::Rc;

/// Local stand-in for `TObj_Object` with dynamic type.
#[derive(Debug)]
pub struct TObjObjectRecRi {
    pub name: String,
    pub type_name: String,
}

impl TObjObjectRecRi {
    pub fn is_kind(&self, type_name: &str) -> bool {
        self.type_name == type_name
    }
}

pub type HandleTObjObjectRi = Rc<TObjObjectRecRi>;

/// Local stand-in for a TDF label node possibly carrying a
/// TObj_TReference attribute (the referred object, None = null object).
#[derive(Debug)]
pub struct OcafLabelNodeRi {
    pub entry: String,
    /// Some(...) when the label has a TObj_TReference attribute;
    /// the inner Option is the resolved referred object.
    pub reference: Option<Option<HandleTObjObjectRi>>,
    pub children: Vec<Rc<OcafLabelNodeRi>>,
}

impl OcafLabelNodeRi {
    pub fn leaf(entry: &str, reference: Option<Option<HandleTObjObjectRi>>) -> Rc<Self> {
        Rc::new(OcafLabelNodeRi { entry: entry.to_string(), reference, children: Vec::new() })
    }

    pub fn node(
        entry: &str,
        reference: Option<Option<HandleTObjObjectRi>>,
        children: Vec<Rc<OcafLabelNodeRi>>,
    ) -> Rc<Self> {
        Rc::new(OcafLabelNodeRi { entry: entry.to_string(), reference, children })
    }
}

/// Iterator by references of an object.
pub struct TObjReferenceIterator {
    node: Option<Rc<OcafLabelNodeRi>>,
    object: Option<HandleTObjObjectRi>,
    /// Pre-order visit list (children of the root label).
    visit: Vec<Rc<OcafLabelNodeRi>>,
    cursor: usize,
    my_type: Option<String>,
}

impl TObjReferenceIterator {
    /// Constructor: initializes by label + recursion, then MakeStep.
    pub fn new(root: &Rc<OcafLabelNodeRi>, the_type: Option<&str>, recursive: bool) -> Self {
        let mut visit = Vec::new();
        fn collect(node: &Rc<OcafLabelNodeRi>, recursive: bool, out: &mut Vec<Rc<OcafLabelNodeRi>>) {
            for child in &node.children {
                out.push(child.clone());
                if recursive {
                    collect(child, recursive, out);
                }
            }
        }
        collect(root, recursive, &mut visit);
        let mut it = TObjReferenceIterator {
            node: None,
            object: None,
            visit,
            cursor: 0,
            my_type: the_type.map(|s| s.to_string()),
        };
        it.make_step();
        it
    }

    pub fn more(&self) -> bool {
        self.node.is_some()
    }

    pub fn next(&mut self) {
        self.object = None;
        self.node = None;
        self.make_step();
    }

    /// The referred object of the current reference (None = null object).
    pub fn value(&self) -> Option<HandleTObjObjectRi> {
        self.object.clone()
    }

    pub fn label_value(&self) -> Option<Rc<OcafLabelNodeRi>> {
        self.node.clone()
    }

    /// TObj_ReferenceIterator::MakeStep.
    fn make_step(&mut self) {
        while self.cursor < self.visit.len() && self.node.is_none() {
            let label = self.visit[self.cursor].clone();
            self.cursor += 1; // for-loop increment (myIterator.Next())
            if let Some(referred) = &label.reference {
                self.object = referred.clone();
                if let (Some(filter), Some(obj)) = (&self.my_type, &self.object) {
                    if !obj.is_kind(filter) {
                        continue; // type mismatch: skip this reference
                    }
                }
                self.node = Some(label);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn obj(name: &str, ty: &str) -> HandleTObjObjectRi {
        Rc::new(TObjObjectRecRi { name: name.into(), type_name: ty.into() })
    }

    /// Object label with reference sub-labels:
    /// 0:5
    ///   0:5:1 -> wheel (TObj_Partition)
    ///   0:5:2 (no reference)
    ///   0:5:3 -> axle (TObj_Object)
    ///   0:5:4 -> NULL object reference
    fn sample_tree() -> Rc<OcafLabelNodeRi> {
        OcafLabelNodeRi::node(
            "0:5",
            None,
            vec![
                OcafLabelNodeRi::leaf("0:5:1", Some(Some(obj("wheel", "TObj_Partition")))),
                OcafLabelNodeRi::leaf("0:5:2", None),
                OcafLabelNodeRi::leaf("0:5:3", Some(Some(obj("axle", "TObj_Object")))),
                OcafLabelNodeRi::leaf("0:5:4", Some(None)),
            ],
        )
    }

    #[test]
    fn iterates_reference_labels_only() {
        let mut it = TObjReferenceIterator::new(&sample_tree(), None, false);
        let mut entries = Vec::new();
        while it.more() {
            entries.push(it.label_value().unwrap().entry.clone());
            it.next();
        }
        assert_eq!(entries, vec!["0:5:1", "0:5:3", "0:5:4"]);
    }

    #[test]
    fn type_filter_skips_mismatched_but_keeps_null() {
        let mut it = TObjReferenceIterator::new(&sample_tree(), Some("TObj_Partition"), false);
        let mut seen = Vec::new();
        while it.more() {
            seen.push((it.label_value().unwrap().entry.clone(), it.value().map(|o| o.name.clone())));
            it.next();
        }
        // axle (TObj_Object) is skipped; NULL-object reference is reported.
        assert_eq!(
            seen,
            vec![
                ("0:5:1".to_string(), Some("wheel".to_string())),
                ("0:5:4".to_string(), None)
            ]
        );
    }

    #[test]
    fn recursive_descends_into_children() {
        let tree = OcafLabelNodeRi::node(
            "0:6",
            None,
            vec![OcafLabelNodeRi::node(
                "0:6:1",
                None,
                vec![OcafLabelNodeRi::leaf("0:6:1:1", Some(Some(obj("deep", "T"))))],
            )],
        );
        let it_flat = TObjReferenceIterator::new(&tree, None, false);
        assert!(!it_flat.more(), "non-recursive finds nothing");
        let it_rec = TObjReferenceIterator::new(&tree, None, true);
        assert!(it_rec.more());
        assert_eq!(it_rec.value().unwrap().name, "deep");
    }

    #[test]
    fn empty_label_yields_exhausted_iterator() {
        let tree = OcafLabelNodeRi::node("0:7", None, vec![]);
        let it = TObjReferenceIterator::new(&tree, None, true);
        assert!(!it.more());
        assert!(it.value().is_none());
    }
}
