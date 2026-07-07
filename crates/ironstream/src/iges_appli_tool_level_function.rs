// FILE: iges_appli_tool_level_function.rs
// occt: IGESAppli_ToolLevelFunction

#[derive(Clone, Debug)]
pub struct IgesAppliToolLevelFunction;

impl IgesAppliToolLevelFunction {
    pub fn new() -> Self {
        Self
    }
}

impl Default for IgesAppliToolLevelFunction {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = IgesAppliToolLevelFunction::new();
    }
}
