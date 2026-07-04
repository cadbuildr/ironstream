// FILE: iges_dimen_tool_diameter_dimension.rs
// occt: IGESDimen_dimentooldiameterdimension

pub struct IGESDimen_dimentooldiameterdimension;

impl IGESDimen_dimentooldiameterdimension {
    pub fn new() -> Self {
        IGESDimen_dimentooldiameterdimension
    }
}

impl Default for IGESDimen_dimentooldiameterdimension {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_creation() {
        let _tool = IGESDimen_dimentooldiameterdimension::new();
    }
}
