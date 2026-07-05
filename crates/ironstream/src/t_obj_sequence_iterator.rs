// FILE: t_obj_sequence_iterator.rs
// occt: TObj_SequenceIterator

//! This class is an iterator on sequence.
//! Faithful port of `TObj_SequenceIterator` (.hxx + .cxx): a 1-based
//! index over an HSequence of TObj_Object handles with an optional type
//! filter applied lazily inside `More()` — when the current item fails
//! the type test, More() advances (mutating the index through the const
//! cast in C++) and recurses. Null items terminate the iteration.

use std::cell::Cell;
use std::rc::Rc;

/// Local stand-in for `TObj_Object` with dynamic type.
#[derive(Debug)]
pub struct TObjObjectRecSi {
    pub name: String,
    pub type_name: String,
}

impl TObjObjectRecSi {
    pub fn is_kind(&self, type_name: &str) -> bool {
        self.type_name == type_name
    }
}

pub type HandleTObjObjectSi = Rc<TObjObjectRecSi>;

/// `TObj_HSequenceOfObject` slots: None models a null object handle.
pub type TObjHSequenceOfObjectSi = Vec<Option<HandleTObjObjectSi>>;

/// Iterator on a sequence of TObj objects.
pub struct TObjSequenceIterator {
    /// Current 1-based index (Cell: More() advances it, const in C++).
    index: Cell<i32>,
    my_type: Option<String>,
    objects: Option<Rc<TObjHSequenceOfObjectSi>>,
}

impl TObjSequenceIterator {
    /// Protected empty constructor (index starts at 1, no sequence).
    pub fn new_empty() -> Self {
        TObjSequenceIterator { index: Cell::new(1), my_type: None, objects: None }
    }

    /// Public constructor by sequence + optional type filter.
    pub fn new(objects: Rc<TObjHSequenceOfObjectSi>, the_type: Option<&str>) -> Self {
        TObjSequenceIterator {
            index: Cell::new(1),
            my_type: the_type.map(|s| s.to_string()),
            objects: Some(objects),
        }
    }

    /// TObj_SequenceIterator::More — advances past filtered-out items.
    pub fn more(&self) -> bool {
        let objects = match &self.objects {
            Some(o) => o,
            None => return false,
        };
        let idx = self.index.get();
        let is_more = idx > 0
            && idx <= objects.len() as i32
            && objects[(idx - 1) as usize].is_some();
        if is_more {
            if let Some(filter) = &self.my_type {
                let current = objects[(idx - 1) as usize].as_ref().unwrap();
                if !current.is_kind(filter) {
                    self.index.set(idx + 1); // me->Next()
                    return self.more(); // recurse as in C++
                }
            }
        }
        is_more
    }

    /// TObj_SequenceIterator::Next.
    pub fn next(&mut self) {
        self.index.set(self.index.get() + 1);
    }

    /// TObj_SequenceIterator::Value — panics when the index is invalid
    /// (Sequence::Value raises Standard_OutOfRange in C++).
    pub fn value(&self) -> HandleTObjObjectSi {
        let objects = self.objects.as_ref().expect("TObj_SequenceIterator: null sequence");
        let idx = self.index.get();
        assert!(
            idx >= 1 && idx <= objects.len() as i32,
            "TObj_SequenceIterator: index out of range"
        );
        objects[(idx - 1) as usize]
            .clone()
            .expect("TObj_SequenceIterator: null object at index")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn obj(name: &str, ty: &str) -> Option<HandleTObjObjectSi> {
        Some(Rc::new(TObjObjectRecSi { name: name.into(), type_name: ty.into() }))
    }

    #[test]
    fn iterates_whole_sequence() {
        let seq = Rc::new(vec![obj("a", "T1"), obj("b", "T2"), obj("c", "T1")]);
        let mut it = TObjSequenceIterator::new(seq, None);
        let mut names = Vec::new();
        while it.more() {
            names.push(it.value().name.clone());
            it.next();
        }
        assert_eq!(names, vec!["a", "b", "c"]);
    }

    #[test]
    fn type_filter_advances_inside_more() {
        let seq = Rc::new(vec![obj("a", "T1"), obj("b", "T2"), obj("c", "T1"), obj("d", "T2")]);
        let mut it = TObjSequenceIterator::new(seq, Some("T2"));
        let mut names = Vec::new();
        while it.more() {
            names.push(it.value().name.clone());
            it.next();
        }
        assert_eq!(names, vec!["b", "d"]);
    }

    #[test]
    fn null_item_stops_iteration() {
        let seq = Rc::new(vec![obj("a", "T1"), None, obj("c", "T1")]);
        let mut it = TObjSequenceIterator::new(seq, None);
        let mut names = Vec::new();
        while it.more() {
            names.push(it.value().name.clone());
            it.next();
        }
        assert_eq!(names, vec!["a"], "null handle terminates More()");
    }

    #[test]
    fn empty_iterator_has_nothing() {
        let it = TObjSequenceIterator::new_empty();
        assert!(!it.more());
        let empty_seq = Rc::new(Vec::new());
        let it2 = TObjSequenceIterator::new(empty_seq, None);
        assert!(!it2.more());
    }

    #[test]
    fn filter_that_matches_nothing() {
        let seq = Rc::new(vec![obj("a", "T1"), obj("b", "T1")]);
        let it = TObjSequenceIterator::new(seq, Some("T9"));
        assert!(!it.more());
    }
}
