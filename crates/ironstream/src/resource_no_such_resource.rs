// FILE: resource_no_such_resource.rs
// occt: Resource_NoSuchResource

/// Exception class for resource not found errors
#[derive(Debug, Clone)]
pub struct ResourceNoSuchResource {
    message: String,
}

impl ResourceNoSuchResource {
    /// Creates a new ResourceNoSuchResource exception with a message
    pub fn new(message: impl Into<String>) -> Self {
        ResourceNoSuchResource {
            message: message.into(),
        }
    }

    /// Gets the error message
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for ResourceNoSuchResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Resource_NoSuchResource: {}", self.message)
    }
}

impl std::error::Error for ResourceNoSuchResource {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_no_such_resource() {
        let err = ResourceNoSuchResource::new("resource not found");
        assert_eq!(err.message(), "resource not found");
    }
}
