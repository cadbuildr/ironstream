// FILE: iges_dimen_tool_general_label.rs
// occt-ref: IGESDimen_dimentoolgenerallabel

pub struct IGESDimen_dimentoolgenerallabel;

impl IGESDimen_dimentoolgenerallabel {
    pub fn new() -> Self {
        IGESDimen_dimentoolgenerallabel
    }
}

impl Default for IGESDimen_dimentoolgenerallabel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_creation() {
        let _tool = IGESDimen_dimentoolgenerallabel::new();
    }
}
