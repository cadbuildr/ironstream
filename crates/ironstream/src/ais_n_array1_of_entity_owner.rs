// FILE: ais_n_array1_of_entity_owner.rs
// occt: AIS_NArray1OfEntityOwner

//! Deprecated NCollection alias: NArray1<EntityOwner>
//! Modeled as a vector with 1-based indexing.

/// Entity owner (stub).
#[derive(Clone, Debug)]
pub struct EntityOwner {
    pub id: u32,
}

/// Array with 1-based indexing.
pub struct AisNArray1OfEntityOwner {
    data: Vec<EntityOwner>,
    lower: usize,
}

impl AisNArray1OfEntityOwner {
    /// Create array with given range [lower, upper].
    pub fn new(lower: usize, upper: usize) -> Self {
        let size = upper - lower + 1;
        Self {
            data: vec![EntityOwner { id: 0 }; size],
            lower,
        }
    }

    /// Get value at index (1-based).
    pub fn get(&self, idx: usize) -> Option<&EntityOwner> {
        self.data.get(idx - self.lower)
    }

    /// Set value at index (1-based).
    pub fn set(&mut self, idx: usize, value: EntityOwner) {
        if let Some(elem) = self.data.get_mut(idx - self.lower) {
            *elem = value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_create() {
        let arr = AisNArray1OfEntityOwner::new(1, 5);
        assert_eq!(arr.data.len(), 5);
    }

    #[test]
    fn test_array_set_get() {
        let mut arr = AisNArray1OfEntityOwner::new(1, 3);
        arr.set(1, EntityOwner { id: 100 });
        assert_eq!(arr.get(1).map(|e| e.id), Some(100));
    }
}
