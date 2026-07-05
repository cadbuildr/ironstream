// FILE: mat_sequence_of_arc.rs
// occt: MAT_SequenceOfArc

pub struct MATSequenceOfArc {
    items: Vec<u32>,
}

impl MATSequenceOfArc {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    pub fn append(&mut self, item: u32) {
        self.items.push(item);
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn value_at(&self, index: usize) -> Option<u32> {
        self.items.get(index).copied()
    }
}

impl Default for MATSequenceOfArc {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence() {
        let mut seq = MATSequenceOfArc::new();
        seq.append(10);
        seq.append(20);
        assert_eq!(seq.len(), 2);
        assert_eq!(seq.value_at(0), Some(10));
    }
}
