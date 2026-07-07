// FILE: step_ap214_externally_defined_general_property.rs
// occt: StepAP214_ExternallyDefinedGeneralProperty

/// Representation of STEP AP214 ExternallyDefinedGeneralProperty entity.
#[derive(Clone, Debug)]
pub struct ExternallyDefinedGeneralProperty {
    // Placeholder
}

impl ExternallyDefinedGeneralProperty {
    pub fn new() -> Self {
        ExternallyDefinedGeneralProperty {}
    }
}

impl Default for ExternallyDefinedGeneralProperty {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _property = ExternallyDefinedGeneralProperty::new();
    }
}
