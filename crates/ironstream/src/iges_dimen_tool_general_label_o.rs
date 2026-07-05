// FILE: iges_dimen_tool_general_label_o.rs
// occt: IGESDimen_ToolGeneralLabel

//! Tool to work on a GeneralLabel. Called by various Modules
//! (ReadWriteModule, GeneralModule, SpecificModule)

/// Tool for working with IGES GeneralLabel entities
pub struct IGESDimenToolGeneralLabel;

impl IGESDimenToolGeneralLabel {
    /// Returns a ToolGeneralLabel, ready to work
    pub fn new() -> Self {
        IGESDimenToolGeneralLabel
    }
}

impl Default for IGESDimenToolGeneralLabel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let tool = IGESDimenToolGeneralLabel::new();
        // Tool is successfully constructed and ready to work
    }

    #[test]
    fn test_default() {
        let tool = IGESDimenToolGeneralLabel::default();
        // Tool can be created via default trait
    }
}
