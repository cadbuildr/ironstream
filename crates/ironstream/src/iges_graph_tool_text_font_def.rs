// FILE: iges_graph_tool_text_font_def.rs
// occt: IGESGraph_ToolTextFontDef

pub struct IGESGraphToolTextFontDef;

impl IGESGraphToolTextFontDef {
    pub fn new() -> Self {
        IGESGraphToolTextFontDef
    }

    pub fn read_own_params(&self) {}
    pub fn write_own_params(&self) {}
    pub fn own_shared(&self) {}
    pub fn dir_checker(&self) {}
    pub fn own_check(&self) {}
    pub fn own_copy(&self) {}
    pub fn own_dump(&self) {}
}

impl Default for IGESGraphToolTextFontDef {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = IGESGraphToolTextFontDef::new();
    }
}
