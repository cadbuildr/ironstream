// FILE: if_graph_sub_parts_iterator.rs
// occt: IFGraph_SubPartsIterator

/// Defines general form for graph classes whose result is
/// not a single iteration on entities, but a nested one.
/// External iteration works on sub-parts, identified by each class.
/// Internal iteration concerns entities of a sub-part.
/// Sub-parts are assumed to be disjoint.
#[derive(Clone, Debug)]
pub struct IfGraphSubPartsIterator {
    parts: Vec<Vec<usize>>,
    part_num: usize,
    current_part: usize,
    loaded: Vec<usize>,
    load_mode: bool,
}

impl IfGraphSubPartsIterator {
    /// Creates with a Graph, whole or parts of it.
    /// whole = true: works on the entire Model
    /// whole = false: empty, ready to be filled
    pub fn new(whole: bool) -> Self {
        let (parts, loaded) = if whole {
            (vec![vec![]], vec![])
        } else {
            (vec![], vec![])
        };

        IfGraphSubPartsIterator {
            parts,
            part_num: 0,
            current_part: 0,
            loaded,
            load_mode: true,
        }
    }

    /// Creates from another iterator and gets its data
    pub fn from_other(other: &IfGraphSubPartsIterator) -> Self {
        IfGraphSubPartsIterator {
            parts: other.parts.clone(),
            part_num: other.parts.len(),
            current_part: other.parts.len(),
            loaded: other.loaded.clone(),
            load_mode: other.load_mode,
        }
    }

    /// Gets parts from another iterator (in addition to existing ones)
    pub fn get_parts(&mut self, other: &IfGraphSubPartsIterator) {
        for part in &other.parts {
            if !part.is_empty() {
                self.parts.push(part.clone());
            }
        }
        self.part_num = self.parts.len();
    }

    /// Adds an empty part and sets it to receive entities
    /// (OCCT: AddPart sets thepart to the new part number, so following
    /// GetFromEntity/GetFromIter calls feed this part, not the load status)
    pub fn add_part(&mut self) {
        self.parts.push(vec![]);
        self.part_num += 1;
        self.current_part = self.part_num;
        self.load_mode = false;
    }

    /// Returns count of registered parts
    pub fn nb_parts(&self) -> usize {
        self.parts.len()
    }

    /// Returns number of part which currently receives entities (0 at load time)
    pub fn part_num(&self) -> usize {
        self.current_part
    }

    /// Sets to get entities into load status
    pub fn set_load(&mut self) {
        self.load_mode = true;
        self.current_part = 0;
    }

    /// Sets numero of receiving part
    pub fn set_part_num(&mut self, num: usize) {
        if num <= self.parts.len() && num > 0 {
            self.current_part = num;
            self.load_mode = false;
        }
    }

    /// Adds an entity: into load status if in load mode, to current part otherwise
    pub fn get_from_entity(&mut self, entity_num: usize, shared: bool) {
        if self.load_mode {
            if !self.loaded.contains(&entity_num) {
                self.loaded.push(entity_num);
            }
        } else if self.current_part > 0 && self.current_part <= self.parts.len() {
            let idx = self.current_part - 1;
            if !self.parts[idx].contains(&entity_num) {
                self.parts[idx].push(entity_num);
            }
        }
    }

    /// Adds a list of entities (into load mode or to a part)
    pub fn get_from_iter(&mut self, entities: &[usize]) {
        if self.load_mode {
            for &entity in entities {
                if !self.loaded.contains(&entity) {
                    self.loaded.push(entity);
                }
            }
        } else if self.current_part > 0 && self.current_part <= self.parts.len() {
            let idx = self.current_part - 1;
            for &entity in entities {
                if !self.parts[idx].contains(&entity) {
                    self.parts[idx].push(entity);
                }
            }
        }
    }

    /// Erases data
    pub fn reset(&mut self) {
        self.parts.clear();
        self.part_num = 0;
        self.current_part = 0;
        self.loaded.clear();
        self.load_mode = true;
    }

    /// Called before iteration; default is doing nothing
    pub fn evaluate(&mut self) {
        // Override in subclasses
    }

    /// Returns entities which were loaded
    pub fn loaded(&self) -> Vec<usize> {
        self.loaded.clone()
    }

    /// Returns True if an entity is loaded
    pub fn is_loaded(&self, entity_num: usize) -> bool {
        self.loaded.contains(&entity_num)
    }

    /// Returns True if an entity is in a sub-part
    pub fn is_in_part(&self, entity_num: usize) -> bool {
        self.parts.iter().any(|part| part.contains(&entity_num))
    }

    /// Returns number of sub-part containing the entity (0 if not in any part)
    pub fn entity_part_num(&self, entity_num: usize) -> usize {
        for (i, part) in self.parts.iter().enumerate() {
            if part.contains(&entity_num) {
                return i + 1;
            }
        }
        0
    }

    /// Sets iteration to its beginning
    pub fn start(&mut self) {
        self.evaluate();
        self.current_part = if self.parts.is_empty() { 0 } else { 1 };
    }

    /// Returns True if there are more sub-parts
    pub fn more(&self) -> bool {
        self.current_part > 0 && self.current_part <= self.parts.len()
    }

    /// Sets iteration to next sub-part
    pub fn next(&mut self) {
        if self.current_part < self.parts.len() {
            self.current_part += 1;
        } else {
            self.current_part = 0;
        }
    }

    /// Returns True if current sub-part is single
    pub fn is_single(&self) -> bool {
        if self.current_part > 0 && self.current_part <= self.parts.len() {
            self.parts[self.current_part - 1].len() == 1
        } else {
            false
        }
    }

    /// Returns the first entity of current sub-part
    pub fn first_entity(&self) -> Option<usize> {
        if self.current_part > 0 && self.current_part <= self.parts.len() {
            self.parts[self.current_part - 1].first().copied()
        } else {
            None
        }
    }

    /// Returns current sub-part as vector of entities
    pub fn entities(&self) -> Vec<usize> {
        if self.current_part > 0 && self.current_part <= self.parts.len() {
            self.parts[self.current_part - 1].clone()
        } else {
            vec![]
        }
    }
}

impl Default for IfGraphSubPartsIterator {
    fn default() -> Self {
        Self::new(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new() {
        let iter = IfGraphSubPartsIterator::new(false);
        assert_eq!(iter.nb_parts(), 0);
        assert!(iter.load_mode);
    }

    #[test]
    fn test_create_whole() {
        let iter = IfGraphSubPartsIterator::new(true);
        assert_eq!(iter.nb_parts(), 1);
    }

    #[test]
    fn test_add_part() {
        let mut iter = IfGraphSubPartsIterator::new(false);
        iter.add_part();
        assert_eq!(iter.nb_parts(), 1);
        assert_eq!(iter.part_num(), 1);
    }

    #[test]
    fn test_load_entities() {
        let mut iter = IfGraphSubPartsIterator::new(false);
        iter.set_load();
        iter.get_from_entity(1, false);
        iter.get_from_entity(2, false);
        assert!(iter.is_loaded(1));
        assert!(iter.is_loaded(2));
        assert_eq!(iter.loaded().len(), 2);
    }

    #[test]
    fn test_add_entity_to_part() {
        let mut iter = IfGraphSubPartsIterator::new(false);
        iter.add_part();
        iter.get_from_entity(1, false);
        iter.get_from_entity(2, false);
        assert!(iter.is_in_part(1));
        assert!(iter.is_in_part(2));
    }

    #[test]
    fn test_entity_part_num() {
        let mut iter = IfGraphSubPartsIterator::new(false);
        iter.add_part();
        iter.get_from_entity(1, false);
        iter.add_part();
        iter.get_from_entity(2, false);
        assert_eq!(iter.entity_part_num(1), 1);
        assert_eq!(iter.entity_part_num(2), 2);
    }

    #[test]
    fn test_is_single() {
        let mut iter = IfGraphSubPartsIterator::new(false);
        iter.add_part();
        iter.get_from_entity(1, false);
        assert!(iter.is_single());
        iter.get_from_entity(2, false);
        assert!(!iter.is_single());
    }

    #[test]
    fn test_first_entity() {
        let mut iter = IfGraphSubPartsIterator::new(false);
        iter.add_part();
        iter.get_from_entity(1, false);
        assert_eq!(iter.first_entity(), Some(1));
    }

    #[test]
    fn test_iteration() {
        let mut iter = IfGraphSubPartsIterator::new(false);
        iter.add_part();
        iter.get_from_entity(1, false);
        iter.add_part();
        iter.get_from_entity(2, false);

        iter.start();
        assert!(iter.more());
        assert_eq!(iter.first_entity(), Some(1));
        iter.next();
        assert!(iter.more());
        assert_eq!(iter.first_entity(), Some(2));
        iter.next();
        assert!(!iter.more());
    }

    #[test]
    fn test_reset() {
        let mut iter = IfGraphSubPartsIterator::new(false);
        iter.add_part();
        iter.get_from_entity(1, false);
        iter.reset();
        assert_eq!(iter.nb_parts(), 0);
        assert_eq!(iter.loaded().len(), 0);
    }

    #[test]
    fn test_set_part_num() {
        let mut iter = IfGraphSubPartsIterator::new(false);
        iter.add_part();
        iter.add_part();
        iter.set_part_num(1);
        assert_eq!(iter.part_num(), 1);
        iter.set_part_num(2);
        assert_eq!(iter.part_num(), 2);
    }

    #[test]
    fn test_get_from_iter() {
        let mut iter = IfGraphSubPartsIterator::new(false);
        iter.add_part();
        iter.get_from_iter(&[1, 2, 3]);
        assert_eq!(iter.entities().len(), 3);
    }

    #[test]
    fn test_from_other() {
        let mut iter1 = IfGraphSubPartsIterator::new(false);
        iter1.add_part();
        iter1.get_from_entity(1, false);

        let iter2 = IfGraphSubPartsIterator::from_other(&iter1);
        assert_eq!(iter2.nb_parts(), 1);
    }
}
