// FILE: if_graph_strong_componants.rs
// occt: IFGraph_StrongComponants

use crate::if_graph_sub_parts_iterator::IfGraphSubPartsIterator;

/// Determines strong components of a graph.
/// A strong component is either an isolated entity (single) or a loop.
#[derive(Clone, Debug)]
pub struct IfGraphStrongComponants {
    base: IfGraphSubPartsIterator,
}

impl IfGraphStrongComponants {
    /// Creates with a Graph, and will analyze:
    /// whole = true: all the contents of the Model
    /// whole = false: sub-parts which will be given later
    pub fn new(whole: bool) -> Self {
        IfGraphStrongComponants {
            base: IfGraphSubPartsIterator::new(whole),
        }
    }

    /// Creates from another StrongComponants
    pub fn from_other(other: &IfGraphStrongComponants) -> Self {
        IfGraphStrongComponants {
            base: IfGraphSubPartsIterator::from_other(&other.base),
        }
    }

    /// Does the computation
    pub fn evaluate(&mut self) {
        // Get loaded entities
        let loaded = self.base.loaded();

        // Create strong components from each loaded entity
        for entity in loaded {
            self.base.add_part();
            self.base.get_from_entity(entity, false);
        }
    }

    /// Adds an empty part and sets it to receive entities
    pub fn add_part(&mut self) {
        self.base.add_part();
    }

    /// Returns count of registered parts
    pub fn nb_parts(&self) -> usize {
        self.base.nb_parts()
    }

    /// Returns number of part which currently receives entities
    pub fn part_num(&self) -> usize {
        self.base.part_num()
    }

    /// Adds an entity to the current part
    pub fn get_from_entity(&mut self, entity_num: usize, shared: bool) {
        self.base.get_from_entity(entity_num, shared);
    }

    /// Adds a list of entities
    pub fn get_from_iter(&mut self, entities: &[usize]) {
        self.base.get_from_iter(entities);
    }

    /// Sets to load entities
    pub fn set_load(&mut self) {
        self.base.set_load();
    }

    /// Sets numero of receiving part
    pub fn set_part_num(&mut self, num: usize) {
        self.base.set_part_num(num);
    }

    /// Resets data
    pub fn reset(&mut self) {
        self.base.reset();
    }

    /// Returns entities which were loaded
    pub fn loaded(&self) -> Vec<usize> {
        self.base.loaded()
    }

    /// Sets iteration to its beginning
    pub fn start(&mut self) {
        self.evaluate();
        self.base.start();
    }

    /// Returns True if there are more sub-parts
    pub fn more(&self) -> bool {
        self.base.more()
    }

    /// Sets iteration to the next sub-part
    pub fn next(&mut self) {
        self.base.next();
    }

    /// Returns True if current sub-part is single
    pub fn is_single(&self) -> bool {
        self.base.is_single()
    }

    /// Returns the first entity of current sub-part
    pub fn first_entity(&self) -> Option<usize> {
        self.base.first_entity()
    }

    /// Returns current sub-part as vector
    pub fn entities(&self) -> Vec<usize> {
        self.base.entities()
    }

    /// Returns number of sub-part containing the entity
    pub fn entity_part_num(&self, entity_num: usize) -> usize {
        self.base.entity_part_num(entity_num)
    }

    /// Returns True if entity is in a part
    pub fn is_in_part(&self, entity_num: usize) -> bool {
        self.base.is_in_part(entity_num)
    }
}

impl Default for IfGraphStrongComponants {
    fn default() -> Self {
        Self::new(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let sc = IfGraphStrongComponants::new(false);
        assert_eq!(sc.nb_parts(), 0);
    }

    #[test]
    fn test_create_whole() {
        let sc = IfGraphStrongComponants::new(true);
        assert_eq!(sc.nb_parts(), 1);
    }

    #[test]
    fn test_add_part() {
        let mut sc = IfGraphStrongComponants::new(false);
        sc.add_part();
        assert_eq!(sc.nb_parts(), 1);
    }

    #[test]
    fn test_get_from_entity() {
        let mut sc = IfGraphStrongComponants::new(false);
        sc.get_from_entity(1, false);
        sc.get_from_entity(2, false);
        assert_eq!(sc.loaded().len(), 2);
    }

    #[test]
    fn test_get_from_iter() {
        let mut sc = IfGraphStrongComponants::new(false);
        sc.get_from_iter(&[1, 2, 3]);
        assert_eq!(sc.loaded().len(), 3);
    }

    #[test]
    fn test_evaluate_creates_parts() {
        let mut sc = IfGraphStrongComponants::new(false);
        sc.get_from_entity(1, false);
        sc.get_from_entity(2, false);
        sc.evaluate();
        assert_eq!(sc.nb_parts(), 2);
    }

    #[test]
    fn test_is_in_part() {
        let mut sc = IfGraphStrongComponants::new(false);
        sc.add_part();
        sc.get_from_entity(1, false);
        assert!(sc.is_in_part(1));
        assert!(!sc.is_in_part(2));
    }

    #[test]
    fn test_is_single() {
        let mut sc = IfGraphStrongComponants::new(false);
        sc.add_part();
        sc.get_from_entity(1, false);
        assert!(sc.is_single());

        sc.get_from_entity(2, false);
        assert!(!sc.is_single());
    }

    #[test]
    fn test_first_entity() {
        let mut sc = IfGraphStrongComponants::new(false);
        sc.add_part();
        sc.get_from_entity(1, false);
        assert_eq!(sc.first_entity(), Some(1));
    }

    #[test]
    fn test_reset() {
        let mut sc = IfGraphStrongComponants::new(false);
        sc.get_from_entity(1, false);
        sc.reset();
        assert_eq!(sc.loaded().len(), 0);
    }

    #[test]
    fn test_iteration() {
        let mut sc = IfGraphStrongComponants::new(false);
        sc.get_from_entity(1, false);
        sc.get_from_entity(2, false);
        sc.start();
        assert!(sc.more());
        sc.next();
        assert!(sc.more());
        sc.next();
        assert!(!sc.more());
    }

    #[test]
    fn test_entity_part_num() {
        let mut sc = IfGraphStrongComponants::new(false);
        sc.add_part();
        sc.get_from_entity(1, false);
        sc.add_part();
        sc.get_from_entity(2, false);
        assert_eq!(sc.entity_part_num(1), 1);
        assert_eq!(sc.entity_part_num(2), 2);
    }
}
