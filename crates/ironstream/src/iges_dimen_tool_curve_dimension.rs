// FILE: iges_dimen_tool_curve_dimension.rs
// occt: IGESDimen_dimentoolcurvedimension

pub struct IGESDimen_dimentoolcurvedimension;

impl IGESDimen_dimentoolcurvedimension {
    pub fn new() -> Self {
        IGESDimen_dimentoolcurvedimension
    }
}

impl Default for IGESDimen_dimentoolcurvedimension {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_creation() {
        let _tool = IGESDimen_dimentoolcurvedimension::new();
    }
}
