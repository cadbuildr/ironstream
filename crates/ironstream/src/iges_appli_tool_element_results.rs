// FILE: iges_appli_tool_element_results.rs
// occt: IGESAppli_ToolElementResults

/// Tool for reading/writing ElementResults entities.
#[derive(Clone, Debug)]
pub struct IgesAppliToolElementResults;

impl IgesAppliToolElementResults {
    pub fn new() -> Self {
        Self
    }
}

impl Default for IgesAppliToolElementResults {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = IgesAppliToolElementResults::new();
    }
}
