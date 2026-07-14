// FILE: iges_dimen_tool_dimension_units.rs
// occt-ref: IGESDimen_dimentooldimensionunits

pub struct IGESDimen_dimentooldimensionunits;

impl IGESDimen_dimentooldimensionunits {
    pub fn new() -> Self {
        IGESDimen_dimentooldimensionunits
    }
}

impl Default for IGESDimen_dimentooldimensionunits {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_creation() {
        let _tool = IGESDimen_dimentooldimensionunits::new();
    }
}
