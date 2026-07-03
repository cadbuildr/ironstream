// FILE: bopds_interf.rs
// occt: BOPDS_Interf

/// Stores information about interference between two shapes
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BodsInterf {
    index1: i32,
    index2: i32,
    type_id: i32,
}

impl BodsInterf {
    pub fn new() -> Self {
        BodsInterf {
            index1: 0,
            index2: 0,
            type_id: 0,
        }
    }

    pub fn with_indices(i1: i32, i2: i32) -> Self {
        BodsInterf {
            index1: i1,
            index2: i2,
            type_id: 0,
        }
    }

    pub fn set_indices(&mut self, i1: i32, i2: i32) {
        self.index1 = i1;
        self.index2 = i2;
    }

    pub fn indices(&self) -> (i32, i32) {
        (self.index1, self.index2)
    }

    pub fn set_type(&mut self, t: i32) {
        self.type_id = t;
    }

    pub fn type_id(&self) -> i32 {
        self.type_id
    }
}

impl Default for BodsInterf {
    fn default() -> Self {
        BodsInterf::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interf_new() {
        let interf = BodsInterf::new();
        assert_eq!(interf.indices(), (0, 0));
    }

    #[test]
    fn test_interf_with_indices() {
        let interf = BodsInterf::with_indices(2, 3);
        assert_eq!(interf.indices(), (2, 3));
    }

    #[test]
    fn test_set_indices() {
        let mut interf = BodsInterf::new();
        interf.set_indices(5, 7);
        assert_eq!(interf.indices(), (5, 7));
    }

    #[test]
    fn test_type_id() {
        let mut interf = BodsInterf::new();
        interf.set_type(1);
        assert_eq!(interf.type_id(), 1);
    }
}
