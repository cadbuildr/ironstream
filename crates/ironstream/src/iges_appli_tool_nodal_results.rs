// FILE: iges_appli_tool_nodal_results.rs
// occt: IGESAppli_ToolNodalResults

#[derive(Clone, Debug)]
pub struct IgesAppliToolNodalResults;

impl IgesAppliToolNodalResults {
    pub fn new() -> Self {
        Self
    }
}

impl Default for IgesAppliToolNodalResults {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = IgesAppliToolNodalResults::new();
    }
}
