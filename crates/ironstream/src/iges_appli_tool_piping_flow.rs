// FILE: iges_appli_tool_piping_flow.rs
// occt: IGESAppli_ToolPipingFlow

#[derive(Clone, Debug)]
pub struct IgesAppliToolPipingFlow;

impl IgesAppliToolPipingFlow {
    pub fn new() -> Self {
        Self
    }
}

impl Default for IgesAppliToolPipingFlow {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = IgesAppliToolPipingFlow::new();
    }
}
