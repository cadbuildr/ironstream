// FILE: iges_dimen_tool_new_general_note_o.rs
// occt: IGESDimen_ToolNewGeneralNote

//! Tool to work on a NewGeneralNote. Called by various Modules
//! (ReadWriteModule, GeneralModule, SpecificModule)

/// Tool for working with IGES NewGeneralNote entities
pub struct IGESDimenToolNewGeneralNote;

impl IGESDimenToolNewGeneralNote {
    /// Returns a ToolNewGeneralNote, ready to work
    pub fn new() -> Self {
        IGESDimenToolNewGeneralNote
    }
}

impl Default for IGESDimenToolNewGeneralNote {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let tool = IGESDimenToolNewGeneralNote::new();
        // Tool is successfully constructed and ready to work
    }

    #[test]
    fn test_default() {
        let tool = IGESDimenToolNewGeneralNote::default();
        // Tool can be created via default trait
    }
}
