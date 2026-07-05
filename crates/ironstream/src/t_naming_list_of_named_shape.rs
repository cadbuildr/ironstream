// FILE: t_naming_list_of_named_shape.rs
// occt: TNaming_ListOfNamedShape

//! Deprecated NCollection alias (deprecated since OCCT 8.0.0):
//! `typedef NCollection_List<opencascade::handle<TNaming_NamedShape>>
//!    TNaming_ListOfNamedShape;`
//! plus `TNaming_ListIteratorOfListOfNamedShape`.
//!
//! `TNaming_NamedShape` is the OCAF attribute recording an evolution of
//! shapes at a label; modeled locally with its evolution enum, version
//! and label entry.

use std::rc::Rc;

/// `TNaming_Evolution` values (faithful set).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TNamingEvolutionLns {
    Primitive,
    Generated,
    Modify,
    Delete,
    Replace,
    Selected,
}

/// Local stand-in for `TNaming_NamedShape`.
#[derive(Debug)]
pub struct TNamingNamedShapeRecLns {
    /// Entry of the label carrying the attribute.
    pub label_entry: String,
    pub evolution: TNamingEvolutionLns,
    pub version: i32,
}

impl TNamingNamedShapeRecLns {
    pub fn new(label_entry: &str, evolution: TNamingEvolutionLns) -> Self {
        TNamingNamedShapeRecLns {
            label_entry: label_entry.to_string(),
            evolution,
            version: 0,
        }
    }
}

pub type HandleNamedShapeLns = Rc<TNamingNamedShapeRecLns>;

/// `TNaming_ListOfNamedShape` (NCollection_List semantics).
#[derive(Default)]
pub struct TNamingListOfNamedShape {
    items: Vec<HandleNamedShapeLns>,
}

impl TNamingListOfNamedShape {
    pub fn new() -> Self {
        TNamingListOfNamedShape { items: Vec::new() }
    }

    pub fn extent(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn append(&mut self, ns: HandleNamedShapeLns) {
        self.items.push(ns);
    }

    pub fn prepend(&mut self, ns: HandleNamedShapeLns) {
        self.items.insert(0, ns);
    }

    pub fn first(&self) -> &HandleNamedShapeLns {
        assert!(!self.items.is_empty(), "List: First on empty list");
        &self.items[0]
    }

    pub fn last(&self) -> &HandleNamedShapeLns {
        assert!(!self.items.is_empty(), "List: Last on empty list");
        self.items.last().unwrap()
    }

    pub fn remove_first(&mut self) {
        assert!(!self.items.is_empty(), "List: RemoveFirst on empty list");
        self.items.remove(0);
    }

    /// NCollection_List::Contains (handle identity, as OCCT compares handles).
    pub fn contains(&self, ns: &HandleNamedShapeLns) -> bool {
        self.items.iter().any(|it| Rc::ptr_eq(it, ns))
    }

    /// NCollection_List::Remove(item) — removes first identity match,
    /// returns true if found.
    pub fn remove_item(&mut self, ns: &HandleNamedShapeLns) -> bool {
        if let Some(pos) = self.items.iter().position(|it| Rc::ptr_eq(it, ns)) {
            self.items.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// `TNaming_ListIteratorOfListOfNamedShape`.
    pub fn iter(&self) -> impl Iterator<Item = &HandleNamedShapeLns> {
        self.items.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_order_and_ends() {
        let mut list = TNamingListOfNamedShape::new();
        list.append(Rc::new(TNamingNamedShapeRecLns::new("0:1:1", TNamingEvolutionLns::Primitive)));
        list.append(Rc::new(TNamingNamedShapeRecLns::new("0:1:2", TNamingEvolutionLns::Generated)));
        list.prepend(Rc::new(TNamingNamedShapeRecLns::new("0:1:0", TNamingEvolutionLns::Modify)));
        assert_eq!(list.extent(), 3);
        assert_eq!(list.first().label_entry, "0:1:0");
        assert_eq!(list.last().evolution, TNamingEvolutionLns::Generated);
    }

    #[test]
    fn contains_and_remove_by_handle_identity() {
        let mut list = TNamingListOfNamedShape::new();
        let ns = Rc::new(TNamingNamedShapeRecLns::new("0:2", TNamingEvolutionLns::Selected));
        let twin = Rc::new(TNamingNamedShapeRecLns::new("0:2", TNamingEvolutionLns::Selected));
        list.append(ns.clone());
        assert!(list.contains(&ns));
        assert!(!list.contains(&twin), "handle comparison is identity, not value");
        assert!(!list.remove_item(&twin));
        assert!(list.remove_item(&ns));
        assert!(list.is_empty());
    }

    #[test]
    fn iterator_walks_in_order() {
        let mut list = TNamingListOfNamedShape::new();
        for i in 0..3 {
            list.append(Rc::new(TNamingNamedShapeRecLns::new(
                &format!("0:{i}"),
                TNamingEvolutionLns::Primitive,
            )));
        }
        let entries: Vec<&str> = list.iter().map(|n| n.label_entry.as_str()).collect();
        assert_eq!(entries, vec!["0:0", "0:1", "0:2"]);
    }
}
