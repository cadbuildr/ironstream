// FILE: iges_appli_tool_flow_line_spec.rs
// occt: IGESAppli_ToolFlowLineSpec

#[derive(Clone, Debug)]
pub struct IgesAppliToolFlowLineSpec;

impl IgesAppliToolFlowLineSpec {
    pub fn new() -> Self {
        Self
    }
}

impl Default for IgesAppliToolFlowLineSpec {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = IgesAppliToolFlowLineSpec::new();
    }
}
