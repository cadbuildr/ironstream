// FILE: iges_dimen_tool_linear_dimension.rs
// occt-ref: IGESDimen_dimentoollineardimension

pub struct IGESDimen_dimentoollineardimension;

impl IGESDimen_dimentoollineardimension {
    pub fn new() -> Self {
        IGESDimen_dimentoollineardimension
    }
}

impl Default for IGESDimen_dimentoollineardimension {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_creation() {
        let _tool = IGESDimen_dimentoollineardimension::new();
    }
}
