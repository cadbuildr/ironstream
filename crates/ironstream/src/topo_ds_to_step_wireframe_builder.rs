// FILE: topo_ds_to_step_wireframe_builder.rs
// occt: TopoDSToStep_WireframeBuilder

use super::topo_ds_to_step::BuilderError;

/// Builder for creating wireframe representations.
pub struct WireframeBuilder {
    result: Vec<TransientItem>,
    error: BuilderError,
}

pub struct TransientItem;

impl WireframeBuilder {
    pub fn new() -> Self {
        WireframeBuilder {
            result: Vec::new(),
            error: BuilderError::BuilderDone,
        }
    }

    pub fn value(&self) -> &[TransientItem] {
        &self.result
    }

    pub fn error(&self) -> BuilderError {
        self.error
    }
}

impl Default for WireframeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let builder = WireframeBuilder::new();
        assert_eq!(builder.value().len(), 0);
        assert_eq!(builder.error(), BuilderError::BuilderDone);
    }
}
