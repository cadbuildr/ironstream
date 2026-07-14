// FILE: iges_dimen_tool_dimension_tolerance.rs
// occt-ref: IGESDimen_dimentooldimensiontolerance

pub struct IGESDimen_dimentooldimensiontolerance;

impl IGESDimen_dimentooldimensiontolerance {
    pub fn new() -> Self {
        IGESDimen_dimentooldimensiontolerance
    }
}

impl Default for IGESDimen_dimentooldimensiontolerance {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_creation() {
        let _tool = IGESDimen_dimentooldimensiontolerance::new();
    }
}
