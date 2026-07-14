// FILE: iges_dimen_tool_general_note.rs
// occt-ref: IGESDimen_dimentoolgeneralnote

pub struct IGESDimen_dimentoolgeneralnote;

impl IGESDimen_dimentoolgeneralnote {
    pub fn new() -> Self {
        IGESDimen_dimentoolgeneralnote
    }
}

impl Default for IGESDimen_dimentoolgeneralnote {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_creation() {
        let _tool = IGESDimen_dimentoolgeneralnote::new();
    }
}
