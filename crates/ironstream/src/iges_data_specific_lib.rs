// FILE: iges_data_specific_lib.rs
// occt: IGESData_SpecificLib

//! Library for specific IGES entity handlers.

#[derive(Clone, Debug)]
pub struct SpecificLib {
    handlers: Vec<String>,
}

impl SpecificLib {
    pub fn new() -> Self {
        SpecificLib {
            handlers: Vec::new(),
        }
    }

    pub fn add_handler(&mut self, name: &str) {
        self.handlers.push(name.to_string());
    }

    pub fn handler_count(&self) -> usize {
        self.handlers.len()
    }

    pub fn handlers(&self) -> &[String] {
        &self.handlers
    }
}

impl Default for SpecificLib {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let lib = SpecificLib::new();
        assert_eq!(lib.handler_count(), 0);
    }

    #[test]
    fn test_add_handler() {
        let mut lib = SpecificLib::new();
        lib.add_handler("handler1");
        lib.add_handler("handler2");
        assert_eq!(lib.handler_count(), 2);
    }

    #[test]
    fn test_handlers() {
        let mut lib = SpecificLib::new();
        lib.add_handler("a");
        lib.add_handler("b");
        let handlers = lib.handlers();
        assert_eq!(handlers.len(), 2);
        assert_eq!(handlers[0], "a");
        assert_eq!(handlers[1], "b");
    }
}
