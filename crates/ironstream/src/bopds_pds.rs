// FILE: bopds_pds.rs
// occt: BOPDS_PDS

/// Parallel Data Structure for Boolean Operations
pub struct BodsPds {
    data: Vec<i32>,
}

impl BodsPds {
    pub fn new() -> Self {
        BodsPds { data: Vec::new() }
    }

    pub fn append(&mut self, val: i32) {
        self.data.push(val);
    }

    pub fn get(&self, index: usize) -> Option<i32> {
        self.data.get(index).copied()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl Default for BodsPds {
    fn default() -> Self {
        BodsPds::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pds = BodsPds::new();
        assert_eq!(pds.len(), 0);
    }

    #[test]
    fn test_append() {
        let mut pds = BodsPds::new();
        pds.append(1);
        pds.append(2);
        assert_eq!(pds.len(), 2);
    }

    #[test]
    fn test_get() {
        let mut pds = BodsPds::new();
        pds.append(10);
        assert_eq!(pds.get(0), Some(10));
        assert_eq!(pds.get(1), None);
    }

    #[test]
    fn test_clear() {
        let mut pds = BodsPds::new();
        pds.append(1);
        pds.clear();
        assert_eq!(pds.len(), 0);
    }
}
