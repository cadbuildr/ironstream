// FILE: osd_function.rs
// occt: OSD_Function

/// Function pointer wrapper.
pub struct Function {
    name: String,
}

impl Function {
    /// Create function from name.
    pub fn new(name: &str) -> Self {
        Function {
            name: name.to_string(),
        }
    }

    /// Get function name.
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_function() {
        let f = Function::new("malloc");
        assert_eq!(f.name(), "malloc");
    }
}
