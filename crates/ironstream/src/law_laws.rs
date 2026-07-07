// FILE: law_laws.rs
// occt: Law_Laws, Law_ListIteratorOfLaws

/// Deprecated alias for NCollection_List<opencascade::handle<Law_Function>>.
/// Maintains backward compatibility. Use Vec or collections directly in new code.
pub struct LawLaws {
    items: Vec<u32>, // Placeholder for Law_Function (opaque type)
}

impl LawLaws {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    pub fn append(&mut self, item: u32) {
        self.items.push(item);
    }

    pub fn remove_first(&mut self) -> Option<u32> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn iterator(&self) -> LawListIteratorOfLaws {
        LawListIteratorOfLaws {
            items: self.items.clone(),
            index: 0,
        }
    }
}

impl Default for LawLaws {
    fn default() -> Self {
        Self::new()
    }
}

/// Iterator for Law_Laws.
pub struct LawListIteratorOfLaws {
    items: Vec<u32>,
    index: usize,
}

impl LawListIteratorOfLaws {
    pub fn more(&self) -> bool {
        self.index < self.items.len()
    }

    pub fn next(&mut self) {
        if self.index < self.items.len() {
            self.index += 1;
        }
    }

    pub fn value(&self) -> Option<u32> {
        if self.index < self.items.len() {
            Some(self.items[self.index])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_append_and_remove() {
        let mut list = LawLaws::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);

        list.append(42);
        list.append(99);
        assert_eq!(list.len(), 2);
        assert!(!list.is_empty());

        assert_eq!(list.remove_first(), Some(42));
        assert_eq!(list.len(), 1);
        assert_eq!(list.remove_first(), Some(99));
        assert!(list.is_empty());
        assert_eq!(list.remove_first(), None);
    }

    #[test]
    fn test_list_clear() {
        let mut list = LawLaws::new();
        list.append(1);
        list.append(2);
        list.append(3);
        assert_eq!(list.len(), 3);

        list.clear();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_iterator() {
        let mut list = LawLaws::new();
        list.append(10);
        list.append(20);
        list.append(30);

        let mut iter = list.iterator();
        assert!(iter.more());
        assert_eq!(iter.value(), Some(10));
        iter.next();
        assert!(iter.more());
        assert_eq!(iter.value(), Some(20));
        iter.next();
        assert!(iter.more());
        assert_eq!(iter.value(), Some(30));
        iter.next();
        assert!(!iter.more());
        assert_eq!(iter.value(), None);
    }

    #[test]
    fn test_default() {
        let list = LawLaws::default();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }
}
