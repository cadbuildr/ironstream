// FILE: iges_appli_tool_line_widening.rs
// occt: IGESAppli_ToolLineWidening

#[derive(Clone, Debug)]
pub struct IgesAppliToolLineWidening;

impl IgesAppliToolLineWidening {
    pub fn new() -> Self {
        Self
    }
}

impl Default for IgesAppliToolLineWidening {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = IgesAppliToolLineWidening::new();
    }
}
