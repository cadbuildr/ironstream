// FILE: iges_dimen_tool_flag_note.rs
// occt-ref: IGESDimen_dimentoolflagnote

pub struct IGESDimen_dimentoolflagnote;

impl IGESDimen_dimentoolflagnote {
    pub fn new() -> Self {
        IGESDimen_dimentoolflagnote
    }
}

impl Default for IGESDimen_dimentoolflagnote {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_creation() {
        let _tool = IGESDimen_dimentoolflagnote::new();
    }
}
