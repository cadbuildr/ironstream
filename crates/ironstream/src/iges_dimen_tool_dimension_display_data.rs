// FILE: iges_dimen_tool_dimension_display_data.rs
// occt: IGESDimen_dimentooldimensiondisplaydata

pub struct IGESDimen_dimentooldimensiondisplaydata;

impl IGESDimen_dimentooldimensiondisplaydata {
    pub fn new() -> Self {
        IGESDimen_dimentooldimensiondisplaydata
    }
}

impl Default for IGESDimen_dimentooldimensiondisplaydata {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_creation() {
        let _tool = IGESDimen_dimentooldimensiondisplaydata::new();
    }
}
