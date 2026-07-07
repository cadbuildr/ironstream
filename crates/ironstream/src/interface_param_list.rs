// FILE: interface_param_list.rs
// occt: Interface_ParamList

/// Defines a list of file parameters
pub struct InterfaceParamList {
    myparams: Vec<InterfaceFileParameter>,
}

/// File parameter structure
#[derive(Clone, Debug)]
pub struct InterfaceFileParameter {
    value: String,
    param_type: i32,
}

impl InterfaceParamList {
    /// Creates a ParamList with initial capacity
    pub fn new(increment: usize) -> Self {
        InterfaceParamList {
            myparams: Vec::with_capacity(increment),
        }
    }

    /// Returns the number of elements
    pub fn length(&self) -> usize {
        self.myparams.len()
    }

    /// Returns the lower bound (1 for OCCT compatibility)
    pub fn lower(&self) -> i32 {
        1
    }

    /// Returns the upper bound
    pub fn upper(&self) -> i32 {
        self.myparams.len() as i32
    }

    /// Sets the value at index
    pub fn set_value(&mut self, index: usize, value: InterfaceFileParameter) {
        if index > 0 && index <= self.myparams.len() {
            self.myparams[index - 1] = value;
        }
    }

    /// Gets the value at index
    pub fn value(&self, index: usize) -> Option<&InterfaceFileParameter> {
        if index > 0 && index <= self.myparams.len() {
            Some(&self.myparams[index - 1])
        } else {
            None
        }
    }

    /// Gets mutable value at index
    pub fn change_value(&mut self, index: usize) -> Option<&mut InterfaceFileParameter> {
        if index > 0 && index <= self.myparams.len() {
            Some(&mut self.myparams[index - 1])
        } else {
            None
        }
    }

    /// Appends a parameter
    pub fn append(&mut self, param: InterfaceFileParameter) {
        self.myparams.push(param);
    }

    /// Clears all parameters
    pub fn clear(&mut self) {
        self.myparams.clear();
    }
}

impl Default for InterfaceParamList {
    fn default() -> Self {
        Self::new(256)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let list = InterfaceParamList::new(256);
        assert_eq!(list.length(), 0);
    }

    #[test]
    fn test_append() {
        let mut list = InterfaceParamList::new(10);
        let param = InterfaceFileParameter {
            value: "test".to_string(),
            param_type: 1,
        };
        list.append(param);
        assert_eq!(list.length(), 1);
    }

    #[test]
    fn test_bounds() {
        let list = InterfaceParamList::new(10);
        assert_eq!(list.lower(), 1);
        assert_eq!(list.upper(), 0);
    }

    #[test]
    fn test_clear() {
        let mut list = InterfaceParamList::new(10);
        let param = InterfaceFileParameter {
            value: "test".to_string(),
            param_type: 1,
        };
        list.append(param);
        list.clear();
        assert_eq!(list.length(), 0);
    }
}
