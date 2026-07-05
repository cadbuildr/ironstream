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
            self.data.remove(index).flatten()
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
        seq.append("seg1".to_string());
        assert_eq!(seq.len(), 1);
    }
}
