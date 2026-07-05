// FILE: iges_select_update_file_name.rs
// occt: IGESSelect_UpdateFileName

//! Modifier that updates the IGES file name in the header to match the actual output file name.
//!
//! If the new file name is unknown (e.g., during immediate execution), the original name is kept
//! and a warning is issued.

/// IGES Global Section containing header information
#[derive(Clone, Debug)]
pub struct IGESGlobalSection {
    file_name: String,
}

impl IGESGlobalSection {
    pub fn new() -> Self {
        IGESGlobalSection {
            file_name: String::new(),
        }
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn set_file_name(&mut self, name: &str) {
        self.file_name = name.to_string();
    }
}

/// IGES Model containing a global section
pub struct IGESModel {
    global_section: IGESGlobalSection,
}

impl IGESModel {
    pub fn new() -> Self {
        IGESModel {
            global_section: IGESGlobalSection::new(),
        }
    }

    pub fn global_section(&self) -> IGESGlobalSection {
        self.global_section.clone()
    }

    pub fn set_global_section(&mut self, gs: IGESGlobalSection) {
        self.global_section = gs;
    }

    pub fn verify_check(&self) -> Vec<String> {
        // Placeholder: check validity of IGES model
        Vec::new()
    }
}

/// Interface copy tool for model modification
pub struct InterfaceCopyTool;

/// Check object for collecting warnings and errors
#[derive(Clone, Debug)]
pub struct InterfaceCheck {
    warnings: Vec<String>,
    failures: Vec<String>,
}

impl InterfaceCheck {
    pub fn new() -> Self {
        InterfaceCheck {
            warnings: Vec::new(),
            failures: Vec::new(),
        }
    }

    pub fn add_warning(&mut self, message: &str) {
        self.warnings.push(message.to_string());
    }

    pub fn add_failure(&mut self, message: &str) {
        self.failures.push(message.to_string());
    }

    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }

    pub fn failures(&self) -> &[String] {
        &self.failures
    }
}

/// Context for modification operations
pub struct IFSelectContextModif {
    checks: Vec<InterfaceCheck>,
    file_name: Option<String>,
}

impl IFSelectContextModif {
    pub fn new() -> Self {
        IFSelectContextModif {
            checks: vec![InterfaceCheck::new()],
            file_name: None,
        }
    }

    pub fn has_file_name(&self) -> bool {
        self.file_name.is_some()
    }

    pub fn file_name(&self) -> Option<&str> {
        self.file_name.as_deref()
    }

    pub fn set_file_name(&mut self, name: String) {
        self.file_name = Some(name);
    }

    /// Returns the check at index 0 (for adding warnings/failures)
    pub fn check_mut(&mut self, index: usize) -> Option<&mut InterfaceCheck> {
        self.checks.get_mut(index)
    }

    pub fn add_check(&mut self, check: InterfaceCheck) {
        self.checks.push(check);
    }

    pub fn checks(&self) -> &[InterfaceCheck] {
        &self.checks
    }
}

/// Model modifier for updating IGES file name
pub struct IGESSelectUpdateFileName;

impl IGESSelectUpdateFileName {
    /// Creates an UpdateFileName modifier
    pub fn new() -> Self {
        IGESSelectUpdateFileName
    }

    /// Performs the modification: updates the file name in the IGES header
    pub fn performing(
        &self,
        ctx: &mut IFSelectContextModif,
        target: &mut IGESModel,
        _copy_tool: &InterfaceCopyTool,
    ) {
        if !ctx.has_file_name() {
            if let Some(check) = ctx.check_mut(0) {
                check.add_warning("New File Name unknown, former one is kept");
            }
            return;
        }

        if let Some(file_name) = ctx.file_name() {
            let mut gs = target.global_section();
            gs.set_file_name(file_name);
            target.set_global_section(gs);
        }

        let checks = target.verify_check();
        let mut new_check = InterfaceCheck::new();
        for check_msg in checks {
            new_check.add_failure(&check_msg);
        }
        ctx.add_check(new_check);
    }

    /// Returns a label describing this modifier
    pub fn label(&self) -> String {
        "Updates IGES File Name to new current one".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iges_global_section_file_name() {
        let mut gs = IGESGlobalSection::new();
        assert_eq!(gs.file_name(), "");

        gs.set_file_name("myfile.iges");
        assert_eq!(gs.file_name(), "myfile.iges");
    }

    #[test]
    fn test_iges_model_file_name() {
        let mut model = IGESModel::new();
        let mut gs = model.global_section();
        gs.set_file_name("test.iges");
        model.set_global_section(gs);

        assert_eq!(model.global_section().file_name(), "test.iges");
    }

    #[test]
    fn test_context_modif_creation() {
        let ctx = IFSelectContextModif::new();
        assert!(!ctx.has_file_name());
        assert!(ctx.file_name().is_none());
    }

    #[test]
    fn test_context_modif_set_file_name() {
        let mut ctx = IFSelectContextModif::new();
        ctx.set_file_name("output.iges".to_string());

        assert!(ctx.has_file_name());
        assert_eq!(ctx.file_name(), Some("output.iges"));
    }

    #[test]
    fn test_modifier_creation() {
        let modifier = IGESSelectUpdateFileName::new();
        let label = modifier.label();
        assert_eq!(label, "Updates IGES File Name to new current one");
    }

    #[test]
    fn test_performing_with_file_name() {
        let modifier = IGESSelectUpdateFileName::new();
        let mut ctx = IFSelectContextModif::new();
        ctx.set_file_name("newfile.iges".to_string());

        let mut model = IGESModel::new();
        let copy_tool = InterfaceCopyTool;

        modifier.performing(&mut ctx, &mut model, &copy_tool);

        assert_eq!(model.global_section().file_name(), "newfile.iges");
    }

    #[test]
    fn test_performing_without_file_name() {
        let modifier = IGESSelectUpdateFileName::new();
        let mut ctx = IFSelectContextModif::new();

        let mut model = IGESModel::new();
        let mut gs = model.global_section();
        gs.set_file_name("original.iges");
        model.set_global_section(gs);

        let copy_tool = InterfaceCopyTool;

        modifier.performing(&mut ctx, &mut model, &copy_tool);

        // Original file name should be kept
        assert_eq!(model.global_section().file_name(), "original.iges");
        // Warning should be added
        assert!(!ctx.checks()[0].warnings().is_empty());
        assert_eq!(
            ctx.checks()[0].warnings()[0],
            "New File Name unknown, former one is kept"
        );
    }

    #[test]
    fn test_interface_check_warnings() {
        let mut check = InterfaceCheck::new();
        check.add_warning("Warning 1");
        check.add_warning("Warning 2");

        assert_eq!(check.warnings().len(), 2);
        assert_eq!(check.warnings()[0], "Warning 1");
    }

    #[test]
    fn test_interface_check_failures() {
        let mut check = InterfaceCheck::new();
        check.add_failure("Error 1");
        check.add_failure("Error 2");

        assert_eq!(check.failures().len(), 2);
        assert_eq!(check.failures()[0], "Error 1");
    }

    #[test]
    fn test_context_modif_checks() {
        let mut ctx = IFSelectContextModif::new();
        assert_eq!(ctx.checks().len(), 1); // Initialized with one check at index 0

        let new_check = InterfaceCheck::new();
        ctx.add_check(new_check);
        assert_eq!(ctx.checks().len(), 2);
    }
}
