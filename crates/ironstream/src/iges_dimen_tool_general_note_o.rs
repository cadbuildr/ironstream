// FILE: iges_dimen_tool_general_note_o.rs
// occt: IGESDimen_ToolGeneralNote

//! Tool to work on a GeneralNote. Called by various Modules
//! (ReadWriteModule, GeneralModule, SpecificModule)

/// Tool for working with IGES GeneralNote entities
pub struct IGESDimenToolGeneralNote;

impl IGESDimenToolGeneralNote {
    /// Returns a ToolGeneralNote, ready to work
    pub fn new() -> Self {
        IGESDimenToolGeneralNote
    }
}

impl Default for IGESDimenToolGeneralNote {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let tool = IGESDimenToolGeneralNote::new();
        // Tool is successfully constructed and ready to work
    }

    #[test]
    fn test_default() {
        let tool = IGESDimenToolGeneralNote::default();
        // Tool can be created via default trait
    }
}
