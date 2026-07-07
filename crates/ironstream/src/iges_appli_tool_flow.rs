// FILE: iges_appli_tool_flow.rs
// occt: IGESAppli_ToolFlow

#[derive(Clone, Debug)]
pub struct IgesAppliToolFlow;

impl IgesAppliToolFlow {
    pub fn new() -> Self {
        Self
    }
}

impl Default for IgesAppliToolFlow {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = IgesAppliToolFlow::new();
    }
}
