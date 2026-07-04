// FILE: step_ap214_class.rs
// occt: StepAP214_Class

/// Representation of STEP AP214 Class entity.
#[derive(Clone, Debug)]
pub struct Class {
    // Placeholder
}

impl Class {
    pub fn new() -> Self {
        Class {}
    }
}

impl Default for Class {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _class = Class::new();
    }
}
