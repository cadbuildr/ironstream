// FILE: if_graph_strong_componants.rs
// occt: IFGraph_StrongComponants

/// Simplified port of IFGraph_SubPartsIterator (base class of
/// IFGraph_StrongComponants), inlined here so the module is self-contained.
///
/// In OCCT each entity of the graph carries a status: 0 means "load" status,
/// N > 0 means the entity belongs to sub-part number N. `thepart` is the
/// number of the part currently receiving entities (0 = load status) and is
/// set by `AddPart` / `SetPartNum` / `SetLoad`. This port models the same
/// behaviour with entity numbers instead of transient handles.
#[derive(Clone, Debug)]
struct SubPartsIterator {
    /// Entities per part (part N is `parts[N - 1]`).
    parts: Vec<Vec<usize>>,
    /// Currently receiving part (0 = load status), also used as the
    /// iteration cursor once `start` has been called.
    current_part: usize,
    /// Entities in load status (status 0).
    loaded: Vec<usize>,
}

impl SubPartsIterator {
    /// Creates the iterator. In OCCT `whole = true` loads the entire model
    /// into load status; this standalone port has no attached model, so the
    /// parts list starts empty either way (as in OCCT, where the constructor
    /// never creates parts).
    fn new(_whole: bool) -> Self {
        SubPartsIterator {
            parts: Vec::new(),
            current_part: 0,
            loaded: Vec::new(),
        }
    }

    /// Creates from another iterator and gets its data.
    fn from_other(other: &SubPartsIterator) -> Self {
        other.clone()
    }

    /// Adds an empty part and sets it to receive entities
    /// (OCCT: `theparts->Append(0); thepart = theparts->Length();`).
    fn add_part(&mut self) {
        self.parts.push(Vec::new());
        self.current_part = self.parts.len();
    }

    /// Returns count of registered parts.
    fn nb_parts(&self) -> usize {
        self.parts.len()
    }

    /// Returns number of part which currently receives entities (0 during load).
    fn part_num(&self) -> usize {
        self.current_part
    }

    /// Sets to get entities into load status (OCCT: `thepart = 0`).
    fn set_load(&mut self) {
        self.current_part = 0;
    }

    /// Sets numero of receiving part.
    fn set_part_num(&mut self, num: usize) {
        if num > 0 && num <= self.parts.len() {
            self.current_part = num;
        }
    }

    /// Adds an entity with the current status: load status if no part is
    /// receiving, otherwise into the current part (overwriting any previous
    /// status, as a status is unique per entity in OCCT).
    fn get_from_entity(&mut self, entity_num: usize, _shared: bool) {
        if self.current_part == 0 {
            if !self.loaded.contains(&entity_num) {
                self.loaded.push(entity_num);
            }
        } else {
            let idx = self.current_part - 1;
            self.loaded.retain(|&e| e != entity_num);
            for (i, part) in self.parts.iter_mut().enumerate() {
                if i != idx {
                    part.retain(|&e| e != entity_num);
                }
            }
            if !self.parts[idx].contains(&entity_num) {
                self.parts[idx].push(entity_num);
            }
        }
    }

    /// Adds a list of entities (into load status or the current part).
    fn get_from_iter(&mut self, entities: &[usize]) {
        for &entity in entities {
            self.get_from_entity(entity, false);
        }
    }

    /// Erases data.
    fn reset(&mut self) {
        self.parts.clear();
        self.current_part = 0;
        self.loaded.clear();
    }

    /// Returns entities which are in load status.
    fn loaded(&self) -> Vec<usize> {
        self.loaded.clone()
    }

    /// Sets iteration to its beginning.
    fn start(&mut self) {
        self.current_part = if self.parts.is_empty() { 0 } else { 1 };
    }

    /// Returns True if there are more sub-parts.
    fn more(&self) -> bool {
        self.current_part > 0 && self.current_part <= self.parts.len()
    }

    /// Sets iteration to the next sub-part.
    fn next(&mut self) {
        if self.current_part < self.parts.len() {
            self.current_part += 1;
        } else {
            self.current_part = 0;
        }
    }

    /// Returns True if current sub-part is single.
    fn is_single(&self) -> bool {
        if self.current_part > 0 && self.current_part <= self.parts.len() {
            self.parts[self.current_part - 1].len() == 1
        } else {
            false
        }
    }

    /// Returns the first entity of current sub-part.
    fn first_entity(&self) -> Option<usize> {
        if self.current_part > 0 && self.current_part <= self.parts.len() {
            self.parts[self.current_part - 1].first().copied()
        } else {
            None
        }
    }

    /// Returns current sub-part as vector.
    fn entities(&self) -> Vec<usize> {
        if self.current_part > 0 && self.current_part <= self.parts.len() {
            self.parts[self.current_part - 1].clone()
        } else {
            Vec::new()
        }
    }

    /// Returns number of sub-part containing the entity (0 if in none).
    fn entity_part_num(&self, entity_num: usize) -> usize {
        for (i, part) in self.parts.iter().enumerate() {
            if part.contains(&entity_num) {
                return i + 1;
            }
        }
        0
    }

    /// Returns True if entity is in a part (status != 0).
    fn is_in_part(&self, entity_num: usize) -> bool {
        self.parts.iter().any(|part| part.contains(&entity_num))
    }
}

/// Determines strong components of a graph.
/// A strong component is either an isolated entity (single) or a loop.
///
/// In the absence of shared-reference information in this standalone port,
/// `evaluate` follows OCCT's IFGraph_StrongComponants::Evaluate structure:
/// every loaded entity yields its own part (AddPart + GetFromEntity).
#[derive(Clone, Debug)]
pub struct IfGraphStrongComponants {
    base: SubPartsIterator,
}

impl IfGraphStrongComponants {
    /// Creates with a Graph, and will analyze:
    /// whole = true: all the contents of the Model
    /// whole = false: sub-parts which will be given later
    pub fn new(whole: bool) -> Self {
        IfGraphStrongComponants {
            base: SubPartsIterator::new(whole),
        }
    }

    /// Creates from another StrongComponants
    pub fn from_other(other: &IfGraphStrongComponants) -> Self {
        IfGraphStrongComponants {
            base: SubPartsIterator::from_other(&other.base),
        }
    }

    /// Does the computation: each loaded entity becomes its own part
    /// (OCCT: `AddPart(); GetFromEntity(G.Entity(i), false);` per entity).
    pub fn evaluate(&mut self) {
        let loaded = self.base.loaded();
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

    /// Adds an entity to the current part (or to load status if none)
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

    /// Sets iteration to its beginning (evaluates first, as OCCT Start does)
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
        // OCCT: the constructor never creates parts; `whole` only loads the
        // model entities into load status. Parts appear via AddPart/Evaluate.
        let sc = IfGraphStrongComponants::new(true);
        assert_eq!(sc.nb_parts(), 0);
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
