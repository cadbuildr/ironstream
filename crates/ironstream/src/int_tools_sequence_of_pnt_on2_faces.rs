// FILE: int_tools_sequence_of_pnt_on2_faces.rs
// occt: IntTools_SequenceOfPntOn2Faces

/// Deprecated alias for NCollection_Sequence<IntTools_PntOn2Faces>.
/// Maintains backward compatibility. Use Vec directly in new code.
pub struct IntToolsSequenceOfPntOn2Faces {
    items: Vec<u32>, // Placeholder for IntTools_PntOn2Faces (opaque type)
}

impl IntToolsSequenceOfPntOn2Faces {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    pub fn append(&mut self, item: u32) {
        self.items.push(item);
    }

    pub fn prepend(&mut self, item: u32) {
        self.items.insert(0, item);
    }

    pub fn insert_after(&mut self, index: usize, item: u32) {
        if index < self.items.len() {
            self.items.insert(index + 1, item);
        } else if index == self.items.len() {
            self.items.push(item);
        }
    }

    pub fn remove_at(&mut self, index: usize) -> Option<u32> {
        if index < self.items.len() {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    pub fn value_at(&self, index: usize) -> Option<u32> {
        if index < self.items.len() {
            Some(self.items[index])
        } else {
            None
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
}

impl Default for IntToolsSequenceOfPntOn2Faces {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_append() {
        let mut seq = IntToolsSequenceOfPntOn2Faces::new();
        assert!(seq.is_empty());

        seq.append(10);
        seq.append(20);
        assert_eq!(seq.len(), 2);
        assert_eq!(seq.value_at(0), Some(10));
        assert_eq!(seq.value_at(1), Some(20));
    }

    #[test]
    fn test_sequence_prepend() {
        let mut seq = IntToolsSequenceOfPntOn2Faces::new();
        seq.append(20);
        seq.prepend(10);

        assert_eq!(seq.value_at(0), Some(10));
        assert_eq!(seq.value_at(1), Some(20));
    }

    #[test]
    fn test_sequence_insert_after() {
        let mut seq = IntToolsSequenceOfPntOn2Faces::new();
        seq.append(10);
        seq.append(30);
        seq.insert_after(0, 20);

        assert_eq!(seq.len(), 3);
        assert_eq!(seq.value_at(0), Some(10));
        assert_eq!(seq.value_at(1), Some(20));
        assert_eq!(seq.value_at(2), Some(30));
    }

    #[test]
    fn test_sequence_remove_at() {
        let mut seq = IntToolsSequenceOfPntOn2Faces::new();
        seq.append(10);
        seq.append(20);
        seq.append(30);

        assert_eq!(seq.remove_at(1), Some(20));
        assert_eq!(seq.len(), 2);
        assert_eq!(seq.value_at(1), Some(30));
    }

    #[test]
    fn test_sequence_clear() {
        let mut seq = IntToolsSequenceOfPntOn2Faces::new();
        seq.append(10);
        seq.append(20);
        seq.clear();

        assert!(seq.is_empty());
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_default() {
        let seq = IntToolsSequenceOfPntOn2Faces::default();
        assert!(seq.is_empty());
    }
}
