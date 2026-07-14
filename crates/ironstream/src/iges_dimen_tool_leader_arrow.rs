// FILE: iges_dimen_tool_leader_arrow.rs
// occt-ref: IGESDimen_dimentoolleaderarrow

pub struct IGESDimen_dimentoolleaderarrow;

impl IGESDimen_dimentoolleaderarrow {
    pub fn new() -> Self {
        IGESDimen_dimentoolleaderarrow
    }
}

impl Default for IGESDimen_dimentoolleaderarrow {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_creation() {
        let _tool = IGESDimen_dimentoolleaderarrow::new();
    }
}
