// FILE: iges_dimen_tool_leader_arrow_o.rs
// occt: IGESDimen_ToolLeaderArrow

//! Tool to work on a LeaderArrow. Called by various Modules
//! (ReadWriteModule, GeneralModule, SpecificModule)

/// Tool for working with IGES LeaderArrow entities
pub struct IGESDimenToolLeaderArrow;

impl IGESDimenToolLeaderArrow {
    /// Returns a ToolLeaderArrow, ready to work
    pub fn new() -> Self {
        IGESDimenToolLeaderArrow
    }
}

impl Default for IGESDimenToolLeaderArrow {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let tool = IGESDimenToolLeaderArrow::new();
        // Tool is successfully constructed and ready to work
    }

    #[test]
    fn test_default() {
        let tool = IGESDimenToolLeaderArrow::default();
        // Tool can be created via default trait
    }
}
