// FILE: bopds_pave.rs
// occt: BOPDS_Pave

/// The class BOPDS_Pave is to store information about vertex on an edge
#[derive(Clone, Copy, Debug)]
pub struct BodsPave {
    my_index: i32,
    my_parameter: f64,
}

impl BodsPave {
    /// Empty constructor
    pub fn new() -> Self {
        BodsPave {
            my_index: 0,
            my_parameter: 0.0,
        }
    }

    /// Constructor with index and parameter
    pub fn with_params(the_index: i32, the_parameter: f64) -> Self {
        BodsPave {
            my_index: the_index,
            my_parameter: the_parameter,
        }
    }

    /// Modifier: Sets the index of vertex
    pub fn set_index(&mut self, the_index: i32) {
        self.my_index = the_index;
    }

    /// Selector: Returns the index of vertex
    pub fn index(&self) -> i32 {
        self.my_index
    }

    /// Modifier: Sets the parameter of vertex
    pub fn set_parameter(&mut self, the_parameter: f64) {
        self.my_parameter = the_parameter;
    }

    /// Selector: Returns the parameter of vertex
    pub fn parameter(&self) -> f64 {
        self.my_parameter
    }

    /// Selector: Returns both index and parameter
    pub fn contents(&self) -> (i32, f64) {
        (self.my_index, self.my_parameter)
    }

    /// Query: Returns true if the parameter of this is less than the parameter of theOther
    pub fn is_less(&self, other: &BodsPave) -> bool {
        self.my_parameter < other.my_parameter
    }

    /// Query: Returns true if the parameter of this is equal to the parameter of theOther
    pub fn is_equal(&self, other: &BodsPave) -> bool {
        (self.my_parameter - other.my_parameter).abs() < 1e-15
    }

    /// Dump the pave information
    pub fn dump(&self) {
        println!("Pave: Index={}, Parameter={}", self.my_index, self.my_parameter);
    }
}

impl Default for BodsPave {
    fn default() -> Self {
        BodsPave::new()
    }
}

impl PartialEq for BodsPave {
    fn eq(&self, other: &BodsPave) -> bool {
        self.is_equal(other)
    }
}

impl PartialOrd for BodsPave {
    fn partial_cmp(&self, other: &BodsPave) -> Option<std::cmp::Ordering> {
        self.my_parameter.partial_cmp(&other.my_parameter)
    }
}

impl Eq for BodsPave {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pave_new() {
        let pave = BodsPave::new();
        assert_eq!(pave.index(), 0);
        assert_eq!(pave.parameter(), 0.0);
    }

    #[test]
    fn test_pave_with_params() {
        let pave = BodsPave::with_params(3, 0.5);
        assert_eq!(pave.index(), 3);
        assert_eq!(pave.parameter(), 0.5);
    }

    #[test]
    fn test_set_index() {
        let mut pave = BodsPave::new();
        pave.set_index(2);
        assert_eq!(pave.index(), 2);
    }

    #[test]
    fn test_set_parameter() {
        let mut pave = BodsPave::new();
        pave.set_parameter(0.75);
        assert_eq!(pave.parameter(), 0.75);
    }

    #[test]
    fn test_contents() {
        let pave = BodsPave::with_params(1, 0.25);
        let (idx, param) = pave.contents();
        assert_eq!(idx, 1);
        assert_eq!(param, 0.25);
    }

    #[test]
    fn test_is_less() {
        let pave1 = BodsPave::with_params(0, 0.3);
        let pave2 = BodsPave::with_params(1, 0.7);
        assert_eq!(pave1.is_less(&pave2), true);
        assert_eq!(pave2.is_less(&pave1), false);
    }

    #[test]
    fn test_is_equal() {
        let pave1 = BodsPave::with_params(0, 0.5);
        let pave2 = BodsPave::with_params(1, 0.5);
        assert_eq!(pave1.is_equal(&pave2), true);
    }

    #[test]
    fn test_is_equal_close() {
        let pave1 = BodsPave::with_params(0, 0.5);
        let pave2 = BodsPave::with_params(1, 0.5 + 1e-16);
        assert_eq!(pave1.is_equal(&pave2), true);
    }

    #[test]
    fn test_is_equal_different() {
        let pave1 = BodsPave::with_params(0, 0.5);
        let pave2 = BodsPave::with_params(1, 0.6);
        assert_eq!(pave1.is_equal(&pave2), false);
    }

    #[test]
    fn test_dump() {
        let pave = BodsPave::with_params(2, 0.4);
        pave.dump();
    }

    #[test]
    fn test_default() {
        let pave = BodsPave::default();
        assert_eq!(pave.index(), 0);
        assert_eq!(pave.parameter(), 0.0);
    }
}
