// FILE: if_graph_cycles.rs
// occt: IFGraph_Cycles

/// Determines strong components in a graph which are cycles.
/// A cycle is a strong component that is not single (has more than one entity).
#[derive(Clone, Debug)]
pub struct IfGraphCycles {
    parts: Vec<Vec<usize>>,
    part_num: usize,
    current_part: usize,
}

impl IfGraphCycles {
    /// Creates with a Graph, and will analyze:
    /// whole = true: all the contents of the Model
    /// whole = false: sub-parts which will be given later
    pub fn new(whole: bool) -> Self {
        let parts = if whole {
            vec![vec![]]
        } else {
            vec![]
        };

        IfGraphCycles {
            parts,
            part_num: 0,
            current_part: 0,
        }
    }

    /// Creates from a StrongComponants which was already computed
    pub fn from_strong_components() -> Self {
        IfGraphCycles {
            parts: vec![],
            part_num: 0,
            current_part: 0,
        }
    }

    /// Does the computation. Cycles are StrongComponants which are not Single.
    pub fn evaluate(&mut self) {
        // This is called to analyze and filter parts.
        // Cycles are parts with more than one entity.
        self.parts.retain(|part| part.len() > 1);
    }

    /// Adds an empty part and sets it to receive entities
    pub fn add_part(&mut self) {
        self.parts.push(vec![]);
        self.part_num += 1;
        self.current_part = self.part_num;
    }

    /// Returns count of registered parts
    pub fn nb_parts(&self) -> usize {
        self.parts.len()
    }

    /// Returns numero of part which currently receives entities (0 at load time)
    pub fn part_num(&self) -> usize {
        self.current_part
    }

    /// Sets numero of receiving part to a new value
    pub fn set_part_num(&mut self, num: usize) {
        if num <= self.parts.len() && num > 0 {
            self.current_part = num;
        }
    }

    /// Adds an entity to the current part
    pub fn add_entity(&mut self, entity_num: usize) {
        if self.current_part > 0 && self.current_part <= self.parts.len() {
            let idx = self.current_part - 1;
            if !self.parts[idx].contains(&entity_num) {
                self.parts[idx].push(entity_num);
            }
        }
    }

    /// Adds a list of entities to the current part
    pub fn add_entities(&mut self, entities: &[usize]) {
        for &entity in entities {
            self.add_entity(entity);
        }
    }

    /// Returns the current part as a vector of entity numbers
    pub fn current_part_entities(&self) -> Vec<usize> {
        if self.current_part > 0 && self.current_part <= self.parts.len() {
            self.parts[self.current_part - 1].clone()
        } else {
            vec![]
        }
    }

    /// Returns True if current part is single (has only one entity)
    pub fn is_single(&self) -> bool {
        if self.current_part > 0 && self.current_part <= self.parts.len() {
            self.parts[self.current_part - 1].len() == 1
        } else {
            false
        }
    }

    /// Resets data
    pub fn reset(&mut self) {
        self.parts.clear();
        self.part_num = 0;
        self.current_part = 0;
    }

    /// Iterator support
    pub fn start(&mut self) {
        self.current_part = if self.parts.is_empty() { 0 } else { 1 };
    }

    pub fn more(&self) -> bool {
        self.current_part > 0 && self.current_part <= self.parts.len()
    }

    pub fn next(&mut self) {
        if self.current_part < self.parts.len() {
            self.current_part += 1;
        } else {
            self.current_part = 0;
        }
    }
}

impl Default for IfGraphCycles {
    fn default() -> Self {
        Self::new(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let cycles = IfGraphCycles::new(false);
        assert_eq!(cycles.nb_parts(), 0);
    }

    #[test]
    fn test_create_whole() {
        let cycles = IfGraphCycles::new(true);
        assert_eq!(cycles.nb_parts(), 1);
    }

    #[test]
    fn test_add_part() {
        let mut cycles = IfGraphCycles::new(false);
        cycles.add_part();
        assert_eq!(cycles.nb_parts(), 1);
        assert_eq!(cycles.part_num(), 1);
    }

    #[test]
    fn test_add_entity_to_part() {
        let mut cycles = IfGraphCycles::new(false);
        cycles.add_part();
        cycles.add_entity(1);
        cycles.add_entity(2);
        let entities = cycles.current_part_entities();
        assert_eq!(entities.len(), 2);
        assert!(entities.contains(&1));
        assert!(entities.contains(&2));
    }

    #[test]
    fn test_is_single() {
        let mut cycles = IfGraphCycles::new(false);
        cycles.add_part();
        cycles.add_entity(1);
        assert!(cycles.is_single());

        cycles.add_entity(2);
        assert!(!cycles.is_single());
    }

    #[test]
    fn test_evaluate_filters_singles() {
        let mut cycles = IfGraphCycles::new(false);
        cycles.add_part();
        cycles.add_entity(1);
        cycles.add_entity(2);

        cycles.add_part();
        cycles.add_entity(3);

        cycles.evaluate();
        // Only the first part should remain (it has 2 entities, so it's a cycle)
        // The second part should be removed (it's single)
        assert_eq!(cycles.nb_parts(), 1);
    }

    #[test]
    fn test_iteration() {
        let mut cycles = IfGraphCycles::new(false);
        cycles.add_part();
        cycles.add_entity(1);
        cycles.add_entity(2);
        cycles.add_part();
        cycles.add_entity(3);
        cycles.add_entity(4);

        cycles.start();
        assert!(cycles.more());
        cycles.next();
        assert!(cycles.more());
        cycles.next();
        assert!(!cycles.more());
    }

    #[test]
    fn test_set_part_num() {
        let mut cycles = IfGraphCycles::new(false);
        cycles.add_part();
        cycles.add_part();
        cycles.set_part_num(1);
        assert_eq!(cycles.part_num(), 1);
        cycles.set_part_num(2);
        assert_eq!(cycles.part_num(), 2);
    }

    #[test]
    fn test_reset() {
        let mut cycles = IfGraphCycles::new(false);
        cycles.add_part();
        cycles.add_entity(1);
        cycles.reset();
        assert_eq!(cycles.nb_parts(), 0);
        assert_eq!(cycles.part_num(), 0);
    }
}
