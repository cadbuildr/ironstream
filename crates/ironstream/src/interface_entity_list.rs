// FILE: interface_entity_list.rs
// occt: Interface_EntityList

/// A list of entities for managing collections.
#[derive(Clone, Debug)]
pub struct InterfaceEntityList {
    entities: Vec<usize>,
}

impl InterfaceEntityList {
    /// Creates an empty entity list
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
        }
    }

    /// Appends an entity to the list
    pub fn append(&mut self, entity_id: usize) {
        self.entities.push(entity_id);
    }

    /// Returns the count of entities
    pub fn count(&self) -> usize {
        self.entities.len()
    }

    /// Gets an entity by 1-indexed position
    pub fn value(&self, num: usize) -> Option<usize> {
        if num >= 1 && num <= self.entities.len() {
            Some(self.entities[num - 1])
        } else {
            None
        }
    }
}

impl Default for InterfaceEntityList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let list = InterfaceEntityList::new();
        assert_eq!(list.count(), 0);
    }

    #[test]
    fn test_append() {
        let mut list = InterfaceEntityList::new();
        list.append(10);
        list.append(20);
        assert_eq!(list.count(), 2);
        assert_eq!(list.value(1), Some(10));
        assert_eq!(list.value(2), Some(20));
    }
}
