// FILE: iges_graph_tool_line_font_predefined.rs
// occt: IGESGraph_ToolLineFontPredefined

pub struct IGESGraphToolLineFontPredefined;

impl IGESGraphToolLineFontPredefined {
    pub fn new() -> Self {
        IGESGraphToolLineFontPredefined
    }

    pub fn read_own_params(&self) {}
    pub fn write_own_params(&self) {}
    pub fn own_shared(&self) {}
    pub fn dir_checker(&self) {}
    pub fn own_check(&self) {}
    pub fn own_copy(&self) {}
    pub fn own_dump(&self) {}
}

impl Default for IGESGraphToolLineFontPredefined {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = IGESGraphToolLineFontPredefined::new();
    }
}
