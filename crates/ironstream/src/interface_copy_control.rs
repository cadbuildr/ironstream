// FILE: interface_copy_control.rs
// occt: Interface_CopyControl

/// Controls copying of entities during duplication.
#[derive(Clone, Debug)]
pub struct InterfaceCopyControl {
    mappings: Vec<(usize, usize)>, // (source_id, target_id)
}

impl InterfaceCopyControl {
    /// Creates a CopyControl
    pub fn new() -> Self {
        Self {
            mappings: Vec::new(),
        }
    }

    /// Records a mapping from source to target entity
    pub fn record(&mut self, source_id: usize, target_id: usize) {
        self.mappings.push((source_id, target_id));
    }

    /// Looks up the target for a source entity
    pub fn lookup(&self, source_id: usize) -> Option<usize> {
        self.mappings
            .iter()
            .find(|&&(src, _)| src == source_id)
            .map(|&(_, tgt)| tgt)
    }

    /// Returns the number of mappings
    pub fn count(&self) -> usize {
        self.mappings.len()
    }
}

impl Default for InterfaceCopyControl {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let ctrl = InterfaceCopyControl::new();
        assert_eq!(ctrl.count(), 0);
    }

    #[test]
    fn test_record_and_lookup() {
        let mut ctrl = InterfaceCopyControl::new();
        ctrl.record(10, 20);
        assert_eq!(ctrl.lookup(10), Some(20));
        assert_eq!(ctrl.lookup(99), None);
    }

    #[test]
    fn test_multiple_mappings() {
        let mut ctrl = InterfaceCopyControl::new();
        ctrl.record(1, 100);
        ctrl.record(2, 200);
        ctrl.record(3, 300);
        assert_eq!(ctrl.count(), 3);
        assert_eq!(ctrl.lookup(2), Some(200));
    }
}
