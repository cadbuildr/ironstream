// FILE: db_rep_list_of_face.rs
// occt: DBRep_ListOfFace

/// Stub for DBRep_Face (topology face)
#[derive(Debug, Clone, PartialEq)]
pub struct Face {
    pub id: u64,
    pub normal_x: f64,
    pub normal_y: f64,
    pub normal_z: f64,
}

impl Face {
    pub fn new(id: u64) -> Self {
        Face {
            id,
            normal_x: 0.0,
            normal_y: 0.0,
            normal_z: 1.0,
        }
    }
}

/// Type alias for a list of faces.
/// This is a deprecated typedef from OCCT that has been replaced with generic list usage.
/// In Rust, we use a Vec<Face> as the equivalent.
pub type ListOfFace = Vec<Face>;

/// Iterator-like interface for DBRep_ListOfFace
pub struct ListIteratorOfListOfFace {
    items: Vec<Face>,
    index: usize,
}

impl ListIteratorOfListOfFace {
    /// Create a new iterator from a list.
    pub fn new(list: &ListOfFace) -> Self {
        ListIteratorOfListOfFace {
            items: list.clone(),
            index: 0,
        }
    }

    /// Check if there are more items.
    pub fn more(&self) -> bool {
        self.index < self.items.len()
    }

    /// Move to the next item.
    pub fn next(&mut self) {
        if self.index < self.items.len() {
            self.index += 1;
        }
    }

    /// Get the current value.
    pub fn value(&self) -> Option<&Face> {
        self.items.get(self.index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_face_new() {
        let face = Face::new(42);
        assert_eq!(face.id, 42);
        assert_eq!(face.normal_z, 1.0);
    }

    #[test]
    fn test_list_operations() {
        let mut list: ListOfFace = Vec::new();
        list.push(Face::new(1));
        list.push(Face::new(2));
        list.push(Face::new(3));

        assert_eq!(list.len(), 3);
        assert_eq!(list[0].id, 1);
        assert_eq!(list[1].id, 2);
        assert_eq!(list[2].id, 3);
    }

    #[test]
    fn test_iterator() {
        let mut list: ListOfFace = Vec::new();
        list.push(Face::new(10));
        list.push(Face::new(20));
        list.push(Face::new(30));

        let mut it = ListIteratorOfListOfFace::new(&list);

        assert!(it.more());
        assert_eq!(it.value().unwrap().id, 10);
        it.next();

        assert!(it.more());
        assert_eq!(it.value().unwrap().id, 20);
        it.next();

        assert!(it.more());
        assert_eq!(it.value().unwrap().id, 30);
        it.next();

        assert!(!it.more());
    }

    #[test]
    fn test_empty_list() {
        let list: ListOfFace = Vec::new();
        let it = ListIteratorOfListOfFace::new(&list);
        assert!(!it.more());
    }
}
