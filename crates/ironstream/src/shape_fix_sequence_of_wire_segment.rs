// FILE: shape_fix_sequence_of_wire_segment.rs
// occt: ShapeFix_SequenceOfWireSegment

pub struct ShapeFixSequenceOfWireSegment {
    data: Vec<Option<String>>,
}

impl ShapeFixSequenceOfWireSegment {
    pub fn new() -> Self {
        ShapeFixSequenceOfWireSegment {
            data: vec![None],
        }
    }

    pub fn append(&mut self, value: String) {
        self.data.push(Some(value));
    }

    pub fn prepend(&mut self, value: String) {
        self.data.insert(1, Some(value));
    }

    pub fn value(&self, index: usize) -> Option<&String> {
        if index > 0 && index < self.data.len() {
            self.data[index].as_ref()
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        if self.data.is_empty() {
            0
        } else {
            self.data.len() - 1
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn remove(&mut self, index: usize) -> Option<String> {
        if index > 0 && index < self.data.len() {
            // Slots at indices >= 1 always hold Some(value); Vec::remove
            // returns the Option stored at that slot.
            self.data.remove(index)
        } else {
            None
        }
    }
}

impl Default for ShapeFixSequenceOfWireSegment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut seq = ShapeFixSequenceOfWireSegment::new();
        assert!(seq.is_empty());
        seq.append("seg1".to_string());
        assert_eq!(seq.len(), 1);
        assert!(!seq.is_empty());
        assert_eq!(seq.value(1), Some(&"seg1".to_string()));
        // 1-based indexing: index 0 is invalid.
        assert_eq!(seq.value(0), None);
        assert_eq!(seq.value(2), None);
    }

    #[test]
    fn test_prepend_and_order() {
        let mut seq = ShapeFixSequenceOfWireSegment::new();
        seq.append("b".to_string());
        seq.prepend("a".to_string());
        seq.append("c".to_string());
        assert_eq!(seq.len(), 3);
        assert_eq!(seq.value(1), Some(&"a".to_string()));
        assert_eq!(seq.value(2), Some(&"b".to_string()));
        assert_eq!(seq.value(3), Some(&"c".to_string()));
    }

    #[test]
    fn test_remove() {
        let mut seq = ShapeFixSequenceOfWireSegment::new();
        seq.append("a".to_string());
        seq.append("b".to_string());
        seq.append("c".to_string());
        assert_eq!(seq.remove(2), Some("b".to_string()));
        assert_eq!(seq.len(), 2);
        assert_eq!(seq.value(2), Some(&"c".to_string()));
        // Out-of-range removals return None and change nothing.
        assert_eq!(seq.remove(0), None);
        assert_eq!(seq.remove(5), None);
        assert_eq!(seq.len(), 2);
    }
}
