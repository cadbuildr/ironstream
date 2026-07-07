// FILE: if_select_transform_standard.rs
// occt: IFSelect_TransformStandard

/// Standard transformer for data exchange operations.
#[derive(Clone, Debug)]
pub struct IFSelectTransformStandard {
    name: String,
}

impl IFSelectTransformStandard {
    /// Creates a TransformStandard
    pub fn new(name: String) -> Self {
        Self { name }
    }

    /// Returns the name
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Default for IFSelectTransformStandard {
    fn default() -> Self {
        Self {
            name: "Standard Transform".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let transform = IFSelectTransformStandard::new("transform1".to_string());
        assert_eq!(transform.name(), "transform1");
    }

    #[test]
    fn test_default() {
        let transform = IFSelectTransformStandard::default();
        assert_eq!(transform.name(), "Standard Transform");
    }
}
