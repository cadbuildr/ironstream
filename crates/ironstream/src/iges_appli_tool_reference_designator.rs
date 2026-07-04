// FILE: iges_appli_tool_reference_designator.rs
// occt: IGESAppli_ToolReferenceDesignator

#[derive(Clone, Debug)]
pub struct IgesAppliToolReferenceDesignator;

impl IgesAppliToolReferenceDesignator {
    pub fn new() -> Self {
        Self
    }
}

impl Default for IgesAppliToolReferenceDesignator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = IgesAppliToolReferenceDesignator::new();
    }
}
