// FILE: iges_appli_tool_finite_element.rs
// occt: IGESAppli_ToolFiniteElement

#[derive(Clone, Debug)]
pub struct IgesAppliToolFiniteElement;

impl IgesAppliToolFiniteElement {
    pub fn new() -> Self {
        Self
    }
}

impl Default for IgesAppliToolFiniteElement {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = IgesAppliToolFiniteElement::new();
    }
}
