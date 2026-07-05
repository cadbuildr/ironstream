// FILE: iges_dimen_tool_general_symbol_o.rs
// occt: IGESDimen_ToolGeneralSymbol

//! Tool to work on a GeneralSymbol. Called by various Modules
//! (ReadWriteModule, GeneralModule, SpecificModule)

/// Tool for working with IGES GeneralSymbol entities
pub struct IGESDimenToolGeneralSymbol;

impl IGESDimenToolGeneralSymbol {
    /// Returns a ToolGeneralSymbol, ready to work
    pub fn new() -> Self {
        IGESDimenToolGeneralSymbol
    }
}

impl Default for IGESDimenToolGeneralSymbol {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construction() {
        let tool = IGESDimenToolGeneralSymbol::new();
        // Tool is successfully constructed and ready to work
    }

    #[test]
    fn test_default() {
        let tool = IGESDimenToolGeneralSymbol::default();
        // Tool can be created via default trait
    }
}
