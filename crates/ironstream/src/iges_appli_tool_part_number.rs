// FILE: iges_appli_tool_part_number.rs
// occt: IGESAppli_ToolPartNumber

#[derive(Clone, Debug)]
pub struct IgesAppliToolPartNumber;

impl IgesAppliToolPartNumber {
    pub fn new() -> Self {
        Self
    }
}

impl Default for IgesAppliToolPartNumber {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = IgesAppliToolPartNumber::new();
    }
}
