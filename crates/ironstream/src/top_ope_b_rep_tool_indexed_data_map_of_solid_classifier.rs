// FILE: top_ope_b_rep_tool_indexed_data_map_of_solid_classifier.rs
// occt: TopOpeBRepTool_IndexedDataMapOfSolidClassifier

/// SolidKey: Solid identifier.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SolidKey {
    id: usize,
}

impl SolidKey {
    pub fn new(id: usize) -> Self {
        SolidKey { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

/// SolidClassifier: Classifier for solid topology.
#[derive(Clone, Debug)]
pub struct SolidClassifier {
    state: i32,
    classified: bool,
}

impl SolidClassifier {
    pub fn new() -> Self {
        SolidClassifier {
            state: 0,
            classified: false,
        }
    }

    pub fn with_state(state: i32) -> Self {
        SolidClassifier {
            state,
            classified: true,
        }
    }

    pub fn state(&self) -> i32 {
        self.state
    }

    pub fn set_state(&mut self, state: i32) {
        self.state = state;
        self.classified = true;
    }

    pub fn is_classified(&self) -> bool {
        self.classified
    }
}

impl Default for SolidClassifier {
    fn default() -> Self {
        Self::new()
    }
}

/// IndexedDataMapOfSolidClassifier: 1-based indexed map from Solid to SolidClassifier.
#[derive(Clone, Debug)]
pub struct IndexedDataMapOfSolidClassifier {
    entries: Vec<(SolidKey, SolidClassifier)>,
}

impl IndexedDataMapOfSolidClassifier {
    pub fn new() -> Self {
        IndexedDataMapOfSolidClassifier {
            entries: Vec::new(),
        }
    }

    pub fn add(&mut self, solid: SolidKey, classifier: SolidClassifier) -> usize {
        if let Some(pos) = self.entries.iter().position(|(k, _)| k == &solid) {
            self.entries[pos] = (solid, classifier);
            pos + 1
        } else {
            self.entries.push((solid, classifier));
            self.entries.len()
        }
    }

    pub fn bind(&mut self, solid: SolidKey, classifier: SolidClassifier) -> bool {
        if let Some(entry) = self.entries.iter_mut().find(|(k, _)| k == &solid) {
            entry.1 = classifier;
            false
        } else {
            self.entries.push((solid, classifier));
            true
        }
    }

    pub fn contains(&self, solid: &SolidKey) -> bool {
        self.entries.iter().any(|(k, _)| k == solid)
    }

    pub fn find(&self, solid: &SolidKey) -> Option<&SolidClassifier> {
        self.entries.iter().find(|(k, _)| k == solid).map(|(_, v)| v)
    }

    pub fn find_mut(&mut self, solid: &SolidKey) -> Option<&mut SolidClassifier> {
        self.entries
            .iter_mut()
            .find(|(k, _)| k == solid)
            .map(|(_, v)| v)
    }

    pub fn value_at(&self, index_1based: usize) -> Option<&SolidClassifier> {
        if index_1based == 0 {
            None
        } else {
            self.entries.get(index_1based - 1).map(|(_, v)| v)
        }
    }

    pub fn remove(&mut self, solid: &SolidKey) -> bool {
        if let Some(pos) = self.entries.iter().position(|(k, _)| k == solid) {
            self.entries.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn size(&self) -> usize {
        self.entries.len()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn lower(&self) -> usize {
        1
    }

    pub fn upper(&self) -> usize {
        self.entries.len()
    }
}

impl Default for IndexedDataMapOfSolidClassifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solid_classifier_new() {
        let clf = SolidClassifier::new();
        assert_eq!(clf.state(), 0);
        assert!(!clf.is_classified());
    }

    #[test]
    fn test_solid_classifier_with_state() {
        let clf = SolidClassifier::with_state(1);
        assert_eq!(clf.state(), 1);
        assert!(clf.is_classified());
    }

    #[test]
    fn test_solid_classifier_set_state() {
        let mut clf = SolidClassifier::new();
        clf.set_state(2);
        assert_eq!(clf.state(), 2);
        assert!(clf.is_classified());
    }

    #[test]
    fn test_indexed_map_add() {
        let mut map = IndexedDataMapOfSolidClassifier::new();
        let solid = SolidKey::new(1);
        let clf = SolidClassifier::with_state(1);
        let idx = map.add(solid, clf);
        assert_eq!(idx, 1);
    }

    #[test]
    fn test_indexed_map_find() {
        let mut map = IndexedDataMapOfSolidClassifier::new();
        let solid = SolidKey::new(3);
        let clf = SolidClassifier::with_state(2);
        map.bind(solid.clone(), clf);

        let found = map.find(&solid).unwrap();
        assert_eq!(found.state(), 2);
    }

    #[test]
    fn test_indexed_map_value_at() {
        let mut map = IndexedDataMapOfSolidClassifier::new();
        map.add(SolidKey::new(1), SolidClassifier::with_state(0));

        assert!(map.value_at(0).is_none());
        assert!(map.value_at(1).is_some());
    }
}
