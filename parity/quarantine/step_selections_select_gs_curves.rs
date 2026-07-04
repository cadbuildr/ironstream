// FILE: step_selections_select_gs_curves.rs
// occt: STEPSelections_SelectGSCurves

/// Selector for geometric and structural curves in STEP
pub struct STEPSelections_SelectGSCurves;

impl STEPSelections_SelectGSCurves {
    pub fn new() -> Self {
        STEPSelections_SelectGSCurves
    }
}

impl Default for STEPSelections_SelectGSCurves {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _selector = STEPSelections_SelectGSCurves::new();
    }
}
