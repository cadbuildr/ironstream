// FILE: n_collection_base_list.rs
// occt: NCollection_BaseList

/// Base list container.
pub struct NCollectionBaseList {
    size: usize,
}

impl NCollectionBaseList {
    pub fn new() -> Self {
        Self { size: 0 }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn clear(&mut self) {
        self.size = 0;
    }
}

impl Default for NCollectionBaseList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let list = NCollectionBaseList::new();
        assert!(list.is_empty());
        assert_eq!(list.size(), 0);
    }
}
