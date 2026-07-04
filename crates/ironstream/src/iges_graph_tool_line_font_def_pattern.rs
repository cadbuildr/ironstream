// FILE: iges_graph_tool_line_font_def_pattern.rs
// occt: IGESGraph_ToolLineFontDefPattern

pub struct IGESGraphToolLineFontDefPattern;

impl IGESGraphToolLineFontDefPattern {
    pub fn new() -> Self {
        IGESGraphToolLineFontDefPattern
    }

    pub fn read_own_params(&self) {}
    pub fn write_own_params(&self) {}
    pub fn own_shared(&self) {}
    pub fn dir_checker(&self) {}
    pub fn own_check(&self) {}
    pub fn own_copy(&self) {}
    pub fn own_dump(&self) {}
}

impl Default for IGESGraphToolLineFontDefPattern {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = IGESGraphToolLineFontDefPattern::new();
    }
}
