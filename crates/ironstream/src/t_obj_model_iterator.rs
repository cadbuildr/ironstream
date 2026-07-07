// FILE: t_obj_model_iterator.rs
// occt: TObj_ModelIterator

//! This class provides an iterator by all objects in the model
//! (implements TObj_ObjectIterator interface).
//! Faithful port of `TObj_ModelIterator` (.hxx + .cxx): starts from the
//! model root object, maintains a sequence of child iterators
//! (`myIterSeq`), and in `Next()` drains the LAST iterator in the
//! sequence, pushing a child iterator for every visited object — a
//! depth-first walk over the object tree. `More()` is "current object
//! is not null"; the root itself is the first value.
//!
//! The model, objects and their GetChildren iterators are local models.

use std::cell::RefCell;
use std::rc::Rc;

/// Local stand-in for `TObj_Object`: named node with children.
#[derive(Debug)]
pub struct TObjObjectRecMi {
    pub name: String,
    pub children: Vec<Rc<TObjObjectRecMi>>,
}

impl TObjObjectRecMi {
    pub fn leaf(name: &str) -> Rc<Self> {
        Rc::new(TObjObjectRecMi { name: name.to_string(), children: Vec::new() })
    }

    pub fn node(name: &str, children: Vec<Rc<TObjObjectRecMi>>) -> Rc<Self> {
        Rc::new(TObjObjectRecMi { name: name.to_string(), children })
    }

    /// TObj_Object::GetChildren — None when the object has no children
    /// (a null iterator handle in C++).
    pub fn get_children(self: &Rc<Self>) -> Option<TObjChildIteratorMi> {
        if self.children.is_empty() {
            None
        } else {
            Some(TObjChildIteratorMi {
                items: self.children.clone(),
                cursor: RefCell::new(0),
            })
        }
    }
}

pub type HandleTObjObjectMi = Rc<TObjObjectRecMi>;

/// Local stand-in for a child TObj_ObjectIterator.
pub struct TObjChildIteratorMi {
    items: Vec<HandleTObjObjectMi>,
    cursor: RefCell<usize>,
}

impl TObjChildIteratorMi {
    pub fn more(&self) -> bool {
        *self.cursor.borrow() < self.items.len()
    }

    pub fn value(&self) -> HandleTObjObjectMi {
        self.items[*self.cursor.borrow()].clone()
    }

    pub fn next(&self) {
        *self.cursor.borrow_mut() += 1;
    }
}

/// Local stand-in for `TObj_Model` with its root object.
#[derive(Debug)]
pub struct TObjModelRecMi {
    pub root: Option<HandleTObjObjectMi>,
}

/// Iterator by all objects in the model.
pub struct TObjModelIterator {
    object: Option<HandleTObjObjectMi>,
    iter_seq: Vec<TObjChildIteratorMi>,
}

impl TObjModelIterator {
    /// Constructor by model: the root becomes the current object and its
    /// children iterator is queued.
    pub fn new(model: &TObjModelRecMi) -> Self {
        let mut it = TObjModelIterator { object: model.root.clone(), iter_seq: Vec::new() };
        if let Some(root) = &it.object.clone() {
            it.add_iterator(root);
        }
        it
    }

    /// TObj_ModelIterator::addIterator.
    fn add_iterator(&mut self, obj: &HandleTObjObjectMi) {
        if let Some(children_iter) = obj.get_children() {
            self.iter_seq.push(children_iter);
        }
        // objects without children contribute no iterator
    }

    /// More — true while there is a current object.
    pub fn more(&self) -> bool {
        self.object.is_some()
    }

    /// Value — the current object.
    pub fn value(&self) -> Option<HandleTObjObjectMi> {
        self.object.clone()
    }

    /// Next — drain the last iterator of the sequence, descending into
    /// each visited object (depth-first traversal).
    pub fn next(&mut self) {
        self.object = None;
        while !self.iter_seq.is_empty() {
            let last = self.iter_seq.last().unwrap();
            if last.more() {
                let obj = last.value();
                last.next();
                self.object = Some(obj.clone());
                self.add_iterator(&obj);
                return;
            } else {
                self.iter_seq.pop(); // Remove(myIterSeq.Length())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// root
    ///   a
    ///     a1
    ///     a2
    ///   b
    fn sample_model() -> TObjModelRecMi {
        let a = TObjObjectRecMi::node(
            "a",
            vec![TObjObjectRecMi::leaf("a1"), TObjObjectRecMi::leaf("a2")],
        );
        let b = TObjObjectRecMi::leaf("b");
        TObjModelRecMi { root: Some(TObjObjectRecMi::node("root", vec![a, b])) }
    }

    fn collect(model: &TObjModelRecMi) -> Vec<String> {
        let mut it = TObjModelIterator::new(model);
        let mut names = Vec::new();
        while it.more() {
            names.push(it.value().unwrap().name.clone());
            it.next();
        }
        names
    }

    #[test]
    fn depth_first_over_object_tree() {
        assert_eq!(collect(&sample_model()), vec!["root", "a", "a1", "a2", "b"]);
    }

    #[test]
    fn root_only_model() {
        let model = TObjModelRecMi { root: Some(TObjObjectRecMi::leaf("solo")) };
        assert_eq!(collect(&model), vec!["solo"]);
    }

    #[test]
    fn null_root_yields_empty_iteration() {
        let model = TObjModelRecMi { root: None };
        let it = TObjModelIterator::new(&model);
        assert!(!it.more());
        assert!(it.value().is_none());
    }

    #[test]
    fn deep_nesting() {
        let model = TObjModelRecMi {
            root: Some(TObjObjectRecMi::node(
                "r",
                vec![TObjObjectRecMi::node(
                    "x",
                    vec![TObjObjectRecMi::node("y", vec![TObjObjectRecMi::leaf("z")])],
                )],
            )),
        };
        assert_eq!(collect(&model), vec!["r", "x", "y", "z"]);
    }
}
