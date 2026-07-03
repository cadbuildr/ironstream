// FILE: i_mesh_data_parameters_list.rs
// occt: IMeshData_ParametersList

/// Interface class representing list of parameters on curve.
pub struct ParametersList {
    parameters: Vec<f64>,
}

impl ParametersList {
    /// Constructor.
    pub fn new() -> Self {
        ParametersList {
            parameters: Vec::new(),
        }
    }

    /// Returns parameter with the given index.
    pub fn get_parameter(&self, index: usize) -> Option<f64> {
        self.parameters.get(index).copied()
    }

    /// Sets parameter at the given index.
    pub fn set_parameter(&mut self, index: usize, value: f64) {
        if index < self.parameters.len() {
            self.parameters[index] = value;
        }
    }

    /// Adds a parameter to the list.
    pub fn add_parameter(&mut self, value: f64) {
        self.parameters.push(value);
    }

    /// Returns number of parameters.
    pub fn parameters_nb(&self) -> usize {
        self.parameters.len()
    }

    /// Clears parameters list.
    pub fn clear(&mut self, _is_keep_endpoints: bool) {
        self.parameters.clear();
    }

    /// Removes parameter with the given index.
    pub fn remove_parameter(&mut self, index: usize) {
        if index < self.parameters.len() {
            self.parameters.remove(index);
        }
    }
}

impl Default for ParametersList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameters_list_new() {
        let list = ParametersList::new();
        assert_eq!(list.parameters_nb(), 0);
    }

    #[test]
    fn test_parameters_list_add_parameter() {
        let mut list = ParametersList::new();
        list.add_parameter(0.5);
        list.add_parameter(0.7);
        assert_eq!(list.parameters_nb(), 2);
        assert_eq!(list.get_parameter(0), Some(0.5));
        assert_eq!(list.get_parameter(1), Some(0.7));
    }

    #[test]
    fn test_parameters_list_set_parameter() {
        let mut list = ParametersList::new();
        list.add_parameter(0.5);
        list.set_parameter(0, 0.8);
        assert_eq!(list.get_parameter(0), Some(0.8));
    }

    #[test]
    fn test_parameters_list_remove_parameter() {
        let mut list = ParametersList::new();
        list.add_parameter(0.5);
        list.add_parameter(0.7);
        list.remove_parameter(0);
        assert_eq!(list.parameters_nb(), 1);
        assert_eq!(list.get_parameter(0), Some(0.7));
    }

    #[test]
    fn test_parameters_list_clear() {
        let mut list = ParametersList::new();
        list.add_parameter(0.5);
        list.clear(false);
        assert_eq!(list.parameters_nb(), 0);
    }

    #[test]
    fn test_parameters_list_default() {
        let list = ParametersList::default();
        assert_eq!(list.parameters_nb(), 0);
    }
}
