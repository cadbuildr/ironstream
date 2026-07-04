// FILE: iges_appli_tool_nodal_constraint.rs
// occt: IGESAppli_ToolNodalConstraint

#[derive(Clone, Debug)]
pub struct IgesAppliToolNodalConstraint;

impl IgesAppliToolNodalConstraint {
    pub fn new() -> Self {
        Self
    }
}

impl Default for IgesAppliToolNodalConstraint {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = IgesAppliToolNodalConstraint::new();
    }
}
