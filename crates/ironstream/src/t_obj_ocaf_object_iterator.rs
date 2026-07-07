// FILE: t_obj_ocaf_object_iterator.rs
// occt: TObj_OcafObjectIterator

//! This class provides an iterator by objects in a partition
//! (implements TObj_ObjectIterator interface over OCAF labels).
//! Faithful port of `TObj_OcafObjectIterator` (.hxx + .cxx): walks the
//! child labels, picks labels carrying a TObj object, optionally filters
//! by type, and — the class's distinctive behavior — after finding an
//! object either steps to the next brother (children of found objects
//! are NOT iterated) or, when `theAllSubChildren` is set, continues into
//! the whole subtree. Labels without objects are descended into.
//!
//! Label tree, child iterator and typed objects are local models.

use std::rc::Rc;

/// Local stand-in for `TObj_Object` with a dynamic type for IsKind.
#[derive(Debug)]
pub struct TObjObjectRecOoi {
    pub name: String,
    pub type_name: String,
}

impl TObjObjectRecOoi {
    /// Standard_Transient::IsKind by type name.
    pub fn is_kind(&self, type_name: &str) -> bool {
        self.type_name == type_name
    }
}

pub type HandleTObjObjectOoi = Rc<TObjObjectRecOoi>;

/// Local stand-in for a TDF label node.
#[derive(Debug)]
pub struct OcafLabelNodeOoi {
    pub entry: String,
    pub object: Option<HandleTObjObjectOoi>,
    pub children: Vec<Rc<OcafLabelNodeOoi>>,
}

impl OcafLabelNodeOoi {
    pub fn leaf(entry: &str, object: Option<HandleTObjObjectOoi>) -> Rc<Self> {
        Rc::new(OcafLabelNodeOoi { entry: entry.to_string(), object, children: Vec::new() })
    }

    pub fn node(
        entry: &str,
        object: Option<HandleTObjObjectOoi>,
        children: Vec<Rc<OcafLabelNodeOoi>>,
    ) -> Rc<Self> {
        Rc::new(OcafLabelNodeOoi { entry: entry.to_string(), object, children })
    }
}

/// TDF_ChildIterator model with Next / NextBrother over a pre-order list.
struct TdfChildIteratorOoi {
    visit: Vec<(Rc<OcafLabelNodeOoi>, usize)>,
    cursor: usize,
    recursive: bool,
}

impl TdfChildIteratorOoi {
    fn new(root: &Rc<OcafLabelNodeOoi>, recursive: bool) -> Self {
        let mut visit = Vec::new();
        fn collect(
            node: &Rc<OcafLabelNodeOoi>,
            depth: usize,
            out: &mut Vec<(Rc<OcafLabelNodeOoi>, usize)>,
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
        TdfChildIteratorOoi { visit, cursor: 0, recursive }
    }

    fn more(&self) -> bool {
        self.cursor < self.visit.len()
    }

    fn value(&self) -> Rc<OcafLabelNodeOoi> {
        self.visit[self.cursor].0.clone()
    }

    fn next(&mut self) {
        self.cursor += 1;
    }

    fn next_brother(&mut self) {
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

/// Iterator by objects in a partition.
pub struct TObjOcafObjectIterator {
    node: Option<Rc<OcafLabelNodeOoi>>,
    object: Option<HandleTObjObjectOoi>,
    iterator: TdfChildIteratorOoi,
    /// Optional type filter (None = any type).
    my_type: Option<String>,
    /// Iterate all sub-children instead of skipping found subtrees.
    all_sub_children: bool,
}

impl TObjOcafObjectIterator {
    /// Constructor: initializes and performs the first MakeStep.
    pub fn new(
        root: &Rc<OcafLabelNodeOoi>,
        the_type: Option<&str>,
        recursive: bool,
        all_sub_children: bool,
    ) -> Self {
        let mut it = TObjOcafObjectIterator {
            node: None,
            object: None,
            iterator: TdfChildIteratorOoi::new(root, recursive),
            my_type: the_type.map(|s| s.to_string()),
            all_sub_children,
        };
        it.make_step();
        it
    }

    /// TObj_LabelIterator::More.
    pub fn more(&self) -> bool {
        self.node.is_some()
    }

    /// TObj_LabelIterator::Next.
    pub fn next(&mut self) {
        self.object = None;
        self.node = None;
        self.make_step();
    }

    /// TObj_LabelIterator::Value.
    pub fn value(&self) -> Option<HandleTObjObjectOoi> {
        self.object.clone()
    }

    /// TObj_LabelIterator::LabelValue.
    pub fn label_value(&self) -> Option<Rc<OcafLabelNodeOoi>> {
        self.node.clone()
    }

    /// TObj_OcafObjectIterator::MakeStep.
    fn make_step(&mut self) {
        while self.iterator.more() && self.node.is_none() {
            let label = self.iterator.value();
            if let Some(obj) = &label.object {
                let type_ok = match &self.my_type {
                    None => true,
                    Some(t) => obj.is_kind(t),
                };
                if type_ok {
                    self.object = Some(obj.clone());
                    self.node = Some(label.clone());
                }
                if self.all_sub_children {
                    self.iterator.next();
                } else {
                    self.iterator.next_brother();
                }
            } else {
                self.iterator.next();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn obj(name: &str, ty: &str) -> HandleTObjObjectOoi {
        Rc::new(TObjObjectRecOoi { name: name.into(), type_name: ty.into() })
    }

    /// Partition tree:
    /// 0:1
    ///   0:1:1 [part A (TObj_Partition)]
    ///     0:1:1:1 [sub S (TObj_Object)]
    ///   0:1:2 (empty)
    ///     0:1:2:1 [nested N (TObj_Object)]
    ///   0:1:3 [obj B (TObj_Object)]
    fn sample_tree() -> Rc<OcafLabelNodeOoi> {
        OcafLabelNodeOoi::node(
            "0:1",
            None,
            vec![
                OcafLabelNodeOoi::node(
                    "0:1:1",
                    Some(obj("A", "TObj_Partition")),
                    vec![OcafLabelNodeOoi::leaf("0:1:1:1", Some(obj("S", "TObj_Object")))],
                ),
                OcafLabelNodeOoi::node(
                    "0:1:2",
                    None,
                    vec![OcafLabelNodeOoi::leaf("0:1:2:1", Some(obj("N", "TObj_Object")))],
                ),
                OcafLabelNodeOoi::leaf("0:1:3", Some(obj("B", "TObj_Object"))),
            ],
        )
    }

    fn collect(mut it: TObjOcafObjectIterator) -> Vec<String> {
        let mut names = Vec::new();
        while it.more() {
            names.push(it.value().unwrap().name.clone());
            it.next();
        }
        names
    }

    #[test]
    fn found_object_subtrees_are_skipped() {
        // Recursive, no type filter, allSubChildren = false:
        // A found -> NextBrother skips S; empty 0:1:2 descended -> N; B.
        let names = collect(TObjOcafObjectIterator::new(&sample_tree(), None, true, false));
        assert_eq!(names, vec!["A", "N", "B"]);
    }

    #[test]
    fn all_sub_children_mode_visits_everything() {
        let names = collect(TObjOcafObjectIterator::new(&sample_tree(), None, true, true));
        assert_eq!(names, vec!["A", "S", "N", "B"]);
    }

    #[test]
    fn type_filter_selects_kind() {
        let names = collect(TObjOcafObjectIterator::new(
            &sample_tree(),
            Some("TObj_Partition"),
            true,
            false,
        ));
        assert_eq!(names, vec!["A"]);
        let names2 = collect(TObjOcafObjectIterator::new(
            &sample_tree(),
            Some("TObj_Object"),
            true,
            true,
        ));
        assert_eq!(names2, vec!["S", "N", "B"]);
    }

    #[test]
    fn non_recursive_direct_children() {
        let names = collect(TObjOcafObjectIterator::new(&sample_tree(), None, false, false));
        assert_eq!(names, vec!["A", "B"], "0:1:2 has no object and no descent happens");
    }

    #[test]
    fn label_value_tracks_entry() {
        let it = TObjOcafObjectIterator::new(&sample_tree(), None, true, false);
        assert_eq!(it.label_value().unwrap().entry, "0:1:1");
    }
}
