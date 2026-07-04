// FILE: if_graph_external_sources.rs
// occt: IFGraph_ExternalSources

/// Gives entities which are sources of entities of a sub-part,
/// but are not contained by this sub-part.
/// External sources are entities that are referenced but not included.
#[derive(Clone, Debug)]
pub struct IfGraphExternalSources {
    entities: Vec<usize>,
    status_map: Vec<i32>,
}

impl IfGraphExternalSources {
    /// Creates empty ExternalSources, ready to work
    pub fn new() -> Self {
        IfGraphExternalSources {
            entities: vec![],
            status_map: vec![],
        }
    }

    /// Initializes with a specific size
    pub fn with_size(size: usize) -> Self {
        IfGraphExternalSources {
            entities: vec![],
            status_map: vec![0; size],
        }
    }

    /// Adds an entity and its shared ones to the list
    pub fn get_from_entity(&mut self, entity_num: usize) {
        if !self.entities.contains(&entity_num) {
            self.entities.push(entity_num);
            if entity_num - 1 < self.status_map.len() {
                self.status_map[entity_num - 1] = 0;
            }
        }
    }

    /// Adds a list of entities (as an iterator) with shared ones
    pub fn get_from_iter(&mut self, entities: &[usize]) {
        for &entity_num in entities {
            self.get_from_entity(entity_num);
        }
    }

    /// Allows to restart on a new data set
    pub fn reset_data(&mut self) {
        self.entities.clear();
        self.status_map.iter_mut().for_each(|s| *s = 0);
    }

    /// Evaluates external sources of a set of entities
    pub fn evaluate(&mut self) {
        // Mark all starting entities as status 0
        // External sources (sharings not in the set) become status 1
        for i in 0..self.status_map.len() {
            if !self.entities.contains(&(i + 1)) {
                self.status_map[i] = 1;
            } else {
                self.status_map[i] = 0;
            }
        }
    }

    /// Returns True if no external sources are found
    pub fn is_empty(&mut self) -> bool {
        self.evaluate();
        self.status_map.iter().all(|&s| s != 1)
    }

    /// Returns external sources (entities with status 1)
    pub fn external_sources(&self) -> Vec<usize> {
        let mut result = vec![];
        for i in 0..self.status_map.len() {
            if self.status_map[i] == 1 {
                result.push(i + 1);
            }
        }
        result
    }

    /// Returns all entities in this structure
    pub fn all_entities(&self) -> Vec<usize> {
        self.entities.clone()
    }
}

impl Default for IfGraphExternalSources {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let es = IfGraphExternalSources::new();
        assert_eq!(es.entities.len(), 0);
    }

    #[test]
    fn test_with_size() {
        let es = IfGraphExternalSources::with_size(10);
        assert_eq!(es.status_map.len(), 10);
    }

    #[test]
    fn test_get_from_entity() {
        let mut es = IfGraphExternalSources::with_size(5);
        es.get_from_entity(1);
        es.get_from_entity(2);
        assert_eq!(es.entities.len(), 2);
        assert!(es.entities.contains(&1));
        assert!(es.entities.contains(&2));
    }

    #[test]
    fn test_get_from_iter() {
        let mut es = IfGraphExternalSources::with_size(5);
        es.get_from_iter(&[1, 2, 3]);
        assert_eq!(es.entities.len(), 3);
    }

    #[test]
    fn test_evaluate() {
        let mut es = IfGraphExternalSources::with_size(5);
        es.get_from_entity(1);
        es.get_from_entity(2);
        es.evaluate();

        let external = es.external_sources();
        assert_eq!(external.len(), 3);
        assert!(external.contains(&3));
        assert!(external.contains(&4));
        assert!(external.contains(&5));
    }

    #[test]
    fn test_is_empty_with_all_entities() {
        let mut es = IfGraphExternalSources::with_size(3);
        es.get_from_entity(1);
        es.get_from_entity(2);
        es.get_from_entity(3);
        assert!(es.is_empty());
    }

    #[test]
    fn test_is_empty_with_missing_entities() {
        let mut es = IfGraphExternalSources::with_size(5);
        es.get_from_entity(1);
        es.get_from_entity(2);
        assert!(!es.is_empty());
    }

    #[test]
    fn test_reset_data() {
        let mut es = IfGraphExternalSources::with_size(5);
        es.get_from_entity(1);
        es.get_from_entity(2);
        es.reset_data();
        assert_eq!(es.entities.len(), 0);
    }
}
