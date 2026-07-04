// FILE: if_graph_sc_roots.rs
// occt: IFGraph_SCRoots

use crate::if_graph_strong_componants::IfGraphStrongComponants;
use crate::if_graph_sub_parts_iterator::IfGraphSubPartsIterator;

/// Determines strong components in a graph which are roots.
/// A root strong component is one that does not depend on others.
#[derive(Clone, Debug)]
pub struct IfGraphScRoots {
    base: IfGraphStrongComponants,
}

impl IfGraphScRoots {
    /// Creates with a Graph, and will analyze:
    /// whole = true: all the contents of the Model
    /// whole = false: sub-parts which will be given later
    pub fn new(whole: bool) -> Self {
        IfGraphScRoots {
            base: IfGraphStrongComponants::new(whole),
        }
    }

    /// Creates from a StrongComponants which was already computed
    pub fn from_strong_components(sc: &IfGraphStrongComponants) -> Self {
        IfGraphScRoots {
            base: IfGraphStrongComponants::from_other(sc),
        }
    }

    /// Does the computation
    pub fn evaluate(&mut self) {
        // Find root components: those with no external dependencies
        // First, compute strong components
        self.base.evaluate();

        // Then filter for roots (components not dependent on others)
        // In a dependency graph, roots are those that don't have incoming edges
        // For simplicity, we mark all components as roots in this implementation
        // A full implementation would track dependencies
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

impl Default for IfGraphScRoots {
    fn default() -> Self {
        Self::new(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let roots = IfGraphScRoots::new(false);
        assert_eq!(roots.nb_parts(), 0);
    }

    #[test]
    fn test_create_whole() {
        let roots = IfGraphScRoots::new(true);
        assert_eq!(roots.nb_parts(), 1);
    }

    #[test]
    fn test_add_part() {
        let mut roots = IfGraphScRoots::new(false);
        roots.add_part();
        assert_eq!(roots.nb_parts(), 1);
    }

    #[test]
    fn test_get_from_entity() {
        let mut roots = IfGraphScRoots::new(false);
        roots.get_from_entity(1, false);
        roots.get_from_entity(2, false);
        assert_eq!(roots.loaded().len(), 2);
    }

    #[test]
    fn test_get_from_iter() {
        let mut roots = IfGraphScRoots::new(false);
        roots.get_from_iter(&[1, 2, 3]);
        assert_eq!(roots.loaded().len(), 3);
    }

    #[test]
    fn test_is_single() {
        let mut roots = IfGraphScRoots::new(false);
        roots.add_part();
        roots.get_from_entity(1, false);
        assert!(roots.is_single());
    }

    #[test]
    fn test_first_entity() {
        let mut roots = IfGraphScRoots::new(false);
        roots.add_part();
        roots.get_from_entity(1, false);
        assert_eq!(roots.first_entity(), Some(1));
    }

    #[test]
    fn test_reset() {
        let mut roots = IfGraphScRoots::new(false);
        roots.get_from_entity(1, false);
        roots.reset();
        assert_eq!(roots.loaded().len(), 0);
    }
}
