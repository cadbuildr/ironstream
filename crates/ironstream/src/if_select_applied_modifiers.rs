// FILE: if_select_applied_modifiers.rs
// occt: IFSelect_AppliedModifiers

/// Memorizes and provides access to modifiers which are to be applied to a file.
/// Each modifier has an optional list of entity numbers it applies to.
#[derive(Clone, Debug)]
pub struct IfSelectAppliedModifiers {
    modifiers: Vec<usize>,
    numbers: Vec<Vec<usize>>,
    nb_max: usize,
    nb_entities: usize,
}

impl IfSelectAppliedModifiers {
    /// Creates an AppliedModifiers, ready to record up to nbmax modifiers
    /// on a model of nbent entities
    pub fn new(nbmax: usize, nbent: usize) -> Self {
        IfSelectAppliedModifiers {
            modifiers: Vec::new(),
            numbers: Vec::new(),
            nb_max: nbmax,
            nb_entities: nbent,
        }
    }

    /// Records a modifier. By default applies to all of a produced file.
    /// Returns true if done, false if too many modifiers are recorded
    pub fn add_modif(&mut self, modif_id: usize) -> bool {
        if self.modifiers.len() >= self.nb_max {
            return false;
        }
        self.modifiers.push(modif_id);
        self.numbers.push(vec![]);
        true
    }

    /// Adds a number of entity of the output file to be applied on
    pub fn add_num(&mut self, num: usize) -> bool {
        if self.modifiers.is_empty() {
            return false;
        }
        let last_idx = self.modifiers.len() - 1;
        if !self.numbers[last_idx].contains(&num) {
            self.numbers[last_idx].push(num);
        }
        true
    }

    /// Returns count of recorded modifiers
    pub fn count(&self) -> usize {
        self.modifiers.len()
    }

    /// Returns the modifier at index num (1-based)
    pub fn item(&self, num: usize) -> Option<usize> {
        if num > 0 && num <= self.modifiers.len() {
            Some(self.modifiers[num - 1])
        } else {
            None
        }
    }

    /// Returns the count of entity numbers for a modifier
    pub fn entity_count(&self, num: usize) -> usize {
        if num > 0 && num <= self.modifiers.len() {
            self.numbers[num - 1].len()
        } else {
            0
        }
    }

    /// Returns if modifier applies to all entities
    pub fn is_for_all(&self, num: usize) -> bool {
        if num > 0 && num <= self.modifiers.len() {
            self.numbers[num - 1].is_empty()
        } else {
            false
        }
    }

    /// Returns the entity number at position i for modifier num (1-based)
    pub fn item_num(&self, num: usize, i: usize) -> Option<usize> {
        if num > 0 && num <= self.modifiers.len() && i > 0 && i <= self.numbers[num - 1].len() {
            Some(self.numbers[num - 1][i - 1])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let mods = IfSelectAppliedModifiers::new(10, 100);
        assert_eq!(mods.count(), 0);
    }

    #[test]
    fn test_add_modif() {
        let mut mods = IfSelectAppliedModifiers::new(10, 100);
        assert!(mods.add_modif(1));
        assert_eq!(mods.count(), 1);
        assert_eq!(mods.item(1), Some(1));
    }

    #[test]
    fn test_add_modif_overflow() {
        let mut mods = IfSelectAppliedModifiers::new(1, 100);
        assert!(mods.add_modif(1));
        assert!(!mods.add_modif(2));
    }

    #[test]
    fn test_add_num() {
        let mut mods = IfSelectAppliedModifiers::new(10, 100);
        mods.add_modif(1);
        assert!(mods.add_num(1));
        assert!(mods.add_num(2));
        assert_eq!(mods.entity_count(1), 2);
    }

    #[test]
    fn test_is_for_all() {
        let mut mods = IfSelectAppliedModifiers::new(10, 100);
        mods.add_modif(1);
        assert!(mods.is_for_all(1));
        mods.add_num(1);
        assert!(!mods.is_for_all(1));
    }

    #[test]
    fn test_item_num() {
        let mut mods = IfSelectAppliedModifiers::new(10, 100);
        mods.add_modif(1);
        mods.add_num(5);
        mods.add_num(10);
        assert_eq!(mods.item_num(1, 1), Some(5));
        assert_eq!(mods.item_num(1, 2), Some(10));
    }
}
