// FILE: iges_dimen_tool_flag_note_o.rs
// occt: IGESDimen_ToolFlagNote

//! Tool to work on a FlagNote. Called by various Modules
//! (ReadWriteModule, GeneralModule, SpecificModule)

/// Tool for working with IGES FlagNote entities
pub struct IGESDimenToolFlagNote;

impl IGESDimenToolFlagNote {
    /// Returns a ToolFlagNote, ready to work
    pub fn new() -> Self {
        IGESDimenToolFlagNote
    }
}

impl Default for IGESDimenToolFlagNote {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let tool = IGESDimenToolFlagNote::new();
        // Tool is successfully constructed and ready to work
    }

    #[test]
    fn test_default() {
        let tool = IGESDimenToolFlagNote::default();
        // Tool can be created via default trait
    }
}
