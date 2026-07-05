// FILE: intf_seq_of_tangent_zone.rs
// occt: Intf_SeqOfTangentZone

/// Deprecated alias for NCollection_Sequence<Intf_TangentZone>.
/// Maintains backward compatibility. Use Vec directly in new code.
pub struct IntfSeqOfTangentZone {
    items: Vec<u32>, // Placeholder for Intf_TangentZone (opaque type)
}

impl IntfSeqOfTangentZone {
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

impl Default for IntfSeqOfTangentZone {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_append() {
        let mut seq = IntfSeqOfTangentZone::new();
        assert!(seq.is_empty());

        seq.append(10);
        seq.append(20);
        assert_eq!(seq.len(), 2);
        assert_eq!(seq.value_at(0), Some(10));
        assert_eq!(seq.value_at(1), Some(20));
    }

    #[test]
    fn test_sequence_prepend() {
        let mut seq = IntfSeqOfTangentZone::new();
        seq.append(20);
        seq.prepend(10);

        assert_eq!(seq.value_at(0), Some(10));
        assert_eq!(seq.value_at(1), Some(20));
    }

    #[test]
    fn test_sequence_insert_after() {
        let mut seq = IntfSeqOfTangentZone::new();
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
        let mut seq = IntfSeqOfTangentZone::new();
        seq.append(10);
        seq.append(20);
        seq.append(30);

        assert_eq!(seq.remove_at(1), Some(20));
        assert_eq!(seq.len(), 2);
        assert_eq!(seq.value_at(1), Some(30));
    }

    #[test]
    fn test_sequence_clear() {
        let mut seq = IntfSeqOfTangentZone::new();
        seq.append(10);
        seq.append(20);
        seq.clear();

        assert!(seq.is_empty());
        assert_eq!(seq.len(), 0);
    }

    #[test]
    fn test_default() {
        let seq = IntfSeqOfTangentZone::default();
        assert!(seq.is_empty());
    }
}
