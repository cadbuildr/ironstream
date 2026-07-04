// FILE: topo_ds_to_step_builder.rs
// occt: TopoDSToStep_Builder

use super::topo_ds_to_step::{BuilderError, BuilderError::*};

/// Builder for converting TopoDS shapes to STEP representation.
pub struct Builder {
    error: BuilderError,
}

impl Builder {
    /// Creates a new builder.
    pub fn new() -> Self {
        Builder {
            error: BuilderDone,
        }
    }

    /// Returns the error status.
    pub fn error(&self) -> BuilderError {
        self.error
    }

    /// Sets the error status.
    pub fn set_error(&mut self, err: BuilderError) {
        self.error = err;
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let builder = Builder::new();
        assert_eq!(builder.error(), BuilderDone);
    }

    #[test]
    fn test_error() {
        let mut builder = Builder::new();
        builder.set_error(NoFaceMapped);
        assert_eq!(builder.error(), NoFaceMapped);
    }
}
