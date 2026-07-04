// FILE: interface_param_set.rs
// occt: Interface_ParamSet

use std::sync::Arc;

/// Defines an ordered set of FileParameters
pub struct InterfaceParamSet {
    theval: Option<Vec<u8>>,
    thelnval: usize,
    thelnres: usize,
    thenbpar: usize,
    themxpar: usize,
    thelist: Option<Arc<InterfaceParamList>>,
    thenext: Option<Arc<InterfaceParamSet>>,
}

pub struct InterfaceParamList;
pub struct InterfaceFileParameter;

impl InterfaceParamSet {
    /// Creates an empty ParamSet
    pub fn new(nres: usize, nst: usize) -> Self {
        InterfaceParamSet {
            theval: None,
            thelnval: 0,
            thelnres: nres,
            thenbpar: nst,
            themxpar: nres,
            thelist: None,
            thenext: None,
        }
    }

    /// Adds a parameter (value, length, type, entity number)
    pub fn append(&mut self, _val: &str, _lnval: i32, _typ: i32, _nument: i32) -> usize {
        self.thenbpar += 1;
        self.thenbpar
    }

    /// Adds a complete FileParameter
    pub fn append_param(&mut self, _fp: &InterfaceFileParameter) -> usize {
        self.thenbpar += 1;
        self.thenbpar
    }

    /// Returns the total count of parameters
    pub fn nb_params(&self) -> usize {
        self.thenbpar
    }

    /// Returns a parameter by number
    pub fn param(&self, _num: usize) -> Option<&InterfaceFileParameter> {
        None
    }

    /// Returns mutable parameter
    pub fn change_param(&mut self, _num: usize) -> Option<&mut InterfaceFileParameter> {
        None
    }

    /// Sets a parameter
    pub fn set_param(&mut self, _num: usize, _fp: &InterfaceFileParameter) {
        // TODO: Implement
    }

    /// Builds and returns the sub-list of parameters
    pub fn params(&self, _num: usize, _nb: usize) -> Option<Arc<InterfaceParamList>> {
        self.thelist.clone()
    }

    /// Destructor
    pub fn destroy(&mut self) {
        self.theval = None;
        self.thelist = None;
        self.thenext = None;
    }
}

impl Drop for InterfaceParamSet {
    fn drop(&mut self) {
        self.destroy();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let set = InterfaceParamSet::new(100, 1);
        assert_eq!(set.nb_params(), 1);
    }

    #[test]
    fn test_append() {
        let mut set = InterfaceParamSet::new(100, 1);
        let count = set.append("test", 4, 0, 0);
        assert!(count > 1);
    }

    #[test]
    fn test_nb_params() {
        let set = InterfaceParamSet::new(50, 1);
        assert_eq!(set.nb_params(), 1);
    }
}
