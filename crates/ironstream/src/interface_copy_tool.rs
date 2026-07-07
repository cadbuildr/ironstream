// FILE: interface_copy_tool.rs
// occt: Interface_CopyTool

/// Tool for copying entities between models.
#[derive(Clone, Debug)]
pub struct InterfaceCopyTool {
    source_model: usize,
    target_model: usize,
}

impl InterfaceCopyTool {
    /// Creates a CopyTool
    pub fn new(source: usize, target: usize) -> Self {
        Self {
            source_model: source,
            target_model: target,
        }
    }

    /// Returns the source model
    pub fn source_model(&self) -> usize {
        self.source_model
    }

    /// Returns the target model
    pub fn target_model(&self) -> usize {
        self.target_model
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let tool = InterfaceCopyTool::new(1, 2);
        assert_eq!(tool.source_model(), 1);
        assert_eq!(tool.target_model(), 2);
    }
}
