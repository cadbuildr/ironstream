// FILE: iges_appli_tool_nodal_displ_and_rot.rs
// occt: IGESAppli_ToolNodalDisplAndRot

#[derive(Clone, Debug)]
pub struct IgesAppliToolNodalDisplAndRot;

impl IgesAppliToolNodalDisplAndRot {
    pub fn new() -> Self {
        Self
    }
}

impl Default for IgesAppliToolNodalDisplAndRot {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = IgesAppliToolNodalDisplAndRot::new();
    }
}
