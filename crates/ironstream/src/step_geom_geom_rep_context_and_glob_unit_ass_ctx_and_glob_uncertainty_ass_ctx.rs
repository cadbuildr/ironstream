// FILE: step_geom_geom_rep_context_and_glob_unit_ass_ctx_and_glob_uncertainty_ass_ctx.rs
// occt: StepGeom_GeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx

use std::sync::Arc;

#[derive(Clone)]
pub struct GeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx {
    name: Arc<String>,
}

impl GeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx {
    pub fn new() -> Self {
        Self {
            name: Arc::new(String::new()),
        }
    }

    pub fn init(&mut self, name: String) {
        self.name = Arc::new(name);
    }

    pub fn name(&self) -> String {
        self.name.as_ref().clone()
    }
}

impl Default for GeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let ctx = GeomRepContextAndGlobUnitAssCtxAndGlobUncertaintyAssCtx::new();
        assert_eq!(ctx.name(), "");
    }
}
