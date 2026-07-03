// FILE: select_mgr_triangular_frustum_set.rs
// occt: SelectMgr_TriangularFrustumSet

/// A set of triangular frustums for complex selection volumes.
#[derive(Clone, Debug)]
pub struct TriangularFrustumSet {
    pub set_id: u32,
    pub frustum_count: usize,
}

impl TriangularFrustumSet {
    pub fn new(set_id: u32) -> Self {
        Self {
            set_id,
            frustum_count: 0,
        }
    }

    pub fn add_frustum(&mut self) {
        self.frustum_count += 1;
    }

    pub fn remove_frustum(&mut self) {
        if self.frustum_count > 0 {
            self.frustum_count -= 1;
        }
    }

    pub fn nb_frustums(&self) -> usize {
        self.frustum_count
    }

    pub fn is_empty(&self) -> bool {
        self.frustum_count == 0
    }

    pub fn clear(&mut self) {
        self.frustum_count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_set() {
        let set = TriangularFrustumSet::new(1);
        assert_eq!(set.set_id, 1);
        assert!(set.is_empty());
    }

    #[test]
    fn add_frustums() {
        let mut set = TriangularFrustumSet::new(1);
        set.add_frustum();
        set.add_frustum();
        set.add_frustum();
        assert_eq!(set.nb_frustums(), 3);
    }

    #[test]
    fn remove_frustum() {
        let mut set = TriangularFrustumSet::new(1);
        set.add_frustum();
        set.add_frustum();
        set.remove_frustum();
        assert_eq!(set.nb_frustums(), 1);
    }

    #[test]
    fn clear() {
        let mut set = TriangularFrustumSet::new(1);
        set.add_frustum();
        set.add_frustum();
        set.clear();
        assert!(set.is_empty());
    }
}
