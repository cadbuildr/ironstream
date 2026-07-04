// FILE: step_ap214_externally_defined_class.rs
// occt: StepAP214_ExternallyDefinedClass

/// Representation of STEP AP214 ExternallyDefinedClass entity.
#[derive(Clone, Debug)]
pub struct ExternallyDefinedClass {
    // Placeholder
}

impl ExternallyDefinedClass {
    pub fn new() -> Self {
        ExternallyDefinedClass {}
    }
}

impl Default for ExternallyDefinedClass {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _class = ExternallyDefinedClass::new();
    }
}
