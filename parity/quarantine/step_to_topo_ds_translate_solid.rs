// FILE: step_to_topo_ds_translate_solid.rs
// occt: StepToTopoDS_TranslateSolid

use crate::step_to_topo_ds_root::StepToTopoDS_Root;

/// Translate STEP solid to TopoDS_Solid
pub struct StepToTopoDS_TranslateSolid {
    root: StepToTopoDS_Root,
    solid: Option<String>,
}

impl StepToTopoDS_TranslateSolid {
    pub fn new() -> Self {
        StepToTopoDS_TranslateSolid {
            root: StepToTopoDS_Root::new(),
            solid: None,
        }
    }

    pub fn init(&mut self, solid_key: &str) -> bool {
        self.solid = Some(solid_key.to_string());
        self.root.set_done(true);
        true
    }

    pub fn value(&self) -> Option<&String> {
        self.solid.as_ref()
    }

    pub fn is_done(&self) -> bool {
        self.root.is_done()
    }
}

impl Default for StepToTopoDS_TranslateSolid {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ts = StepToTopoDS_TranslateSolid::new();
        assert!(!ts.is_done());
    }

    #[test]
    fn test_init() {
        let mut ts = StepToTopoDS_TranslateSolid::new();
        assert!(ts.init("solid1"));
        assert!(ts.is_done());
        assert_eq!(ts.value(), Some(&"solid1".to_string()));
    }
}
