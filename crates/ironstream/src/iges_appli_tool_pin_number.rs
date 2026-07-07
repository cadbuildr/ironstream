// FILE: iges_appli_tool_pin_number.rs
// occt: IGESAppli_ToolPinNumber

#[derive(Clone, Debug)]
pub struct IgesAppliToolPinNumber;

impl IgesAppliToolPinNumber {
    pub fn new() -> Self {
        Self
    }
}

impl Default for IgesAppliToolPinNumber {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = IgesAppliToolPinNumber::new();
    }
}
