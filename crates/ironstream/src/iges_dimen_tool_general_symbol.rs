// FILE: iges_dimen_tool_general_symbol.rs
// occt-ref: IGESDimen_dimentoolgeneralsymbol

pub struct IGESDimen_dimentoolgeneralsymbol;

impl IGESDimen_dimentoolgeneralsymbol {
    pub fn new() -> Self {
        IGESDimen_dimentoolgeneralsymbol
    }
}

impl Default for IGESDimen_dimentoolgeneralsymbol {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_creation() {
        let _tool = IGESDimen_dimentoolgeneralsymbol::new();
    }
}
