// FILE: if_select_work_library.rs
// occt: IFSelect_WorkLibrary

/// Library of work utilities for data exchange operations.
#[derive(Clone, Debug)]
pub struct IFSelectWorkLibrary;

impl IFSelectWorkLibrary {
    /// Creates a WorkLibrary
    pub fn new() -> Self {
        Self
    }

    /// Returns the name of this library
    pub fn name(&self) -> &'static str {
        "WorkLibrary"
    }
}

impl Default for IFSelectWorkLibrary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let lib = IFSelectWorkLibrary::new();
        assert_eq!(lib.name(), "WorkLibrary");
    }

    #[test]
    fn test_default() {
        let lib = IFSelectWorkLibrary::default();
        assert_eq!(lib.name(), "WorkLibrary");
    }
}
