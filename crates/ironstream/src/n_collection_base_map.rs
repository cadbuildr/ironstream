// FILE: n_collection_base_map.rs
// occt: NCollection_BaseMap

/// Base map container.
pub struct NCollectionBaseMap {
    size: usize,
    buckets: usize,
}

impl NCollectionBaseMap {
    pub fn new() -> Self {
        Self {
            size: 0,
            buckets: 16,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn buckets(&self) -> usize {
        self.buckets
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn clear(&mut self) {
        self.size = 0;
    }
}

impl Default for NCollectionBaseMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let map = NCollectionBaseMap::new();
        assert!(map.is_empty());
        assert_eq!(map.size(), 0);
        assert_eq!(map.buckets(), 16);
    }
}
