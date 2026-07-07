// FILE: iges_graph_tool_line_font_def_template.rs
// occt: IGESGraph_ToolLineFontDefTemplate

pub struct IGESGraphToolLineFontDefTemplate;

impl IGESGraphToolLineFontDefTemplate {
    pub fn new() -> Self {
        IGESGraphToolLineFontDefTemplate
    }

    pub fn read_own_params(&self) {}
    pub fn write_own_params(&self) {}
    pub fn own_shared(&self) {}
    pub fn dir_checker(&self) {}
    pub fn own_check(&self) {}
    pub fn own_copy(&self) {}
    pub fn own_dump(&self) {}
}

impl Default for IGESGraphToolLineFontDefTemplate {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = IGESGraphToolLineFontDefTemplate::new();
    }
}
