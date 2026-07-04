// FILE: interface_file_parameter.rs
// occt: Interface_FileParameter

/// Represents a parameter from an interface file.
#[derive(Clone, Debug)]
pub struct InterfaceFileParameter {
    value: String,
    entity_num: usize,
}

impl InterfaceFileParameter {
    /// Creates a file parameter
    pub fn new(value: String, entity_num: usize) -> Self {
        Self { value, entity_num }
    }

    /// Returns the parameter value
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the entity number
    pub fn entity_num(&self) -> usize {
        self.entity_num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let param = InterfaceFileParameter::new("value1".to_string(), 5);
        assert_eq!(param.value(), "value1");
        assert_eq!(param.entity_num(), 5);
    }
}
