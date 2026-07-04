// FILE: step_selections_select_for_transfer.rs
// occt: STEPSelections_SelectForTransfer

/// Selector for entities to transfer in STEP
pub struct STEPSelections_SelectForTransfer;

impl STEPSelections_SelectForTransfer {
    pub fn new() -> Self {
        STEPSelections_SelectForTransfer
    }
}

impl Default for STEPSelections_SelectForTransfer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _selector = STEPSelections_SelectForTransfer::new();
    }
}
