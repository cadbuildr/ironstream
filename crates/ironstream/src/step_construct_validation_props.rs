// FILE: step_construct_validation_props.rs
// occt: STEPConstruct_ValidationProps

/// Tool for handling STEP validation properties
pub struct STEPConstruct_ValidationProps;

impl STEPConstruct_ValidationProps {
    pub fn new() -> Self {
        STEPConstruct_ValidationProps
    }
}

impl Default for STEPConstruct_ValidationProps {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _props = STEPConstruct_ValidationProps::new();
    }
}
