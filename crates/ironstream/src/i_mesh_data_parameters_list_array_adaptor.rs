// FILE: i_mesh_data_parameters_list_array_adaptor.rs
// occt: IMeshData_ParametersListArrayAdaptor

/// Trait for parameter list types
pub trait IMeshDataParametersList {
    /// Get the number of parameters
    fn parameters_nb(&self) -> i32;

    /// Get a parameter at the given index
    fn get_parameter(&self, index: i32) -> f64;
}

/// A simple concrete implementation of parameter list
#[derive(Clone, Debug)]
pub struct SimpleParametersList {
    parameters: Vec<f64>,
}

impl SimpleParametersList {
    pub fn new() -> Self {
        SimpleParametersList {
            parameters: Vec::new(),
        }
    }

    pub fn add_parameter(&mut self, param: f64) {
        self.parameters.push(param);
    }

    pub fn parameters(&self) -> &[f64] {
        &self.parameters
    }
}

impl Default for SimpleParametersList {
    fn default() -> Self {
        Self::new()
    }
}

impl IMeshDataParametersList for SimpleParametersList {
    fn parameters_nb(&self) -> i32 {
        self.parameters.len() as i32
    }

    fn get_parameter(&self, index: i32) -> f64 {
        if index < 0 || index >= self.parameters.len() as i32 {
            0.0
        } else {
            self.parameters[index as usize]
        }
    }
}

/// Auxiliary tool representing adaptor interface for parameter lists
/// to be used in tools working on array-like structures.
pub struct IMeshDataParametersListArrayAdaptor<T: IMeshDataParametersList> {
    parameters: T,
}

impl<T: IMeshDataParametersList> IMeshDataParametersListArrayAdaptor<T> {
    /// Constructor. Initializes tool by the given parameters.
    pub fn new(parameters: T) -> Self {
        IMeshDataParametersListArrayAdaptor { parameters }
    }

    /// Returns lower index in parameters array.
    pub fn lower(&self) -> i32 {
        0
    }

    /// Returns upper index in parameters array.
    pub fn upper(&self) -> i32 {
        self.parameters.parameters_nb() - 1
    }

    /// Returns value at the given index.
    pub fn value(&self, index: i32) -> f64 {
        if index < 0 || index > self.upper() {
            0.0
        } else {
            self.parameters.get_parameter(index)
        }
    }

    /// Returns the number of parameters
    pub fn length(&self) -> i32 {
        self.parameters.parameters_nb()
    }

    /// Get reference to parameters
    pub fn parameters(&self) -> &T {
        &self.parameters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_parameter_list() {
        let list = SimpleParametersList::new();
        assert_eq!(list.parameters_nb(), 0);
    }

    #[test]
    fn test_add_parameter() {
        let mut list = SimpleParametersList::new();
        list.add_parameter(1.0);
        list.add_parameter(2.5);
        list.add_parameter(3.7);
        assert_eq!(list.parameters_nb(), 3);
    }

    #[test]
    fn test_get_parameter() {
        let mut list = SimpleParametersList::new();
        list.add_parameter(1.5);
        list.add_parameter(2.5);
        assert_eq!(list.get_parameter(0), 1.5);
        assert_eq!(list.get_parameter(1), 2.5);
    }

    #[test]
    fn test_create_adaptor() {
        let mut list = SimpleParametersList::new();
        list.add_parameter(1.0);
        list.add_parameter(2.0);
        let adaptor = IMeshDataParametersListArrayAdaptor::new(list);
        assert_eq!(adaptor.lower(), 0);
        assert_eq!(adaptor.upper(), 1);
    }

    #[test]
    fn test_adaptor_value() {
        let mut list = SimpleParametersList::new();
        list.add_parameter(10.0);
        list.add_parameter(20.0);
        list.add_parameter(30.0);
        let adaptor = IMeshDataParametersListArrayAdaptor::new(list);
        assert_eq!(adaptor.value(0), 10.0);
        assert_eq!(adaptor.value(1), 20.0);
        assert_eq!(adaptor.value(2), 30.0);
    }

    #[test]
    fn test_adaptor_out_of_bounds() {
        let mut list = SimpleParametersList::new();
        list.add_parameter(1.0);
        let adaptor = IMeshDataParametersListArrayAdaptor::new(list);
        assert_eq!(adaptor.value(-1), 0.0);
        assert_eq!(adaptor.value(10), 0.0);
    }

    #[test]
    fn test_adaptor_length() {
        let mut list = SimpleParametersList::new();
        list.add_parameter(1.0);
        list.add_parameter(2.0);
        list.add_parameter(3.0);
        list.add_parameter(4.0);
        let adaptor = IMeshDataParametersListArrayAdaptor::new(list);
        assert_eq!(adaptor.length(), 4);
    }

    #[test]
    fn test_adaptor_empty() {
        let list = SimpleParametersList::new();
        let adaptor = IMeshDataParametersListArrayAdaptor::new(list);
        assert_eq!(adaptor.length(), 0);
        assert_eq!(adaptor.upper(), -1);
    }
}
