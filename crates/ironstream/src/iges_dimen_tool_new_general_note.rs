// FILE: iges_dimen_tool_new_general_note.rs
// occt-ref: IGESDimen_dimentoolnewgeneralnote

pub struct IGESDimen_dimentoolnewgeneralnote;

impl IGESDimen_dimentoolnewgeneralnote {
    pub fn new() -> Self {
        IGESDimen_dimentoolnewgeneralnote
    }
}

impl Default for IGESDimen_dimentoolnewgeneralnote {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_creation() {
        let _tool = IGESDimen_dimentoolnewgeneralnote::new();
    }
}
