// FILE: iges_graph_tool_text_display_template.rs
// occt: IGESGraph_ToolTextDisplayTemplate

pub struct IGESGraphToolTextDisplayTemplate;

impl IGESGraphToolTextDisplayTemplate {
    pub fn new() -> Self {
        IGESGraphToolTextDisplayTemplate
    }

    pub fn read_own_params(&self) {}
    pub fn write_own_params(&self) {}
    pub fn own_shared(&self) {}
    pub fn dir_checker(&self) {}
    pub fn own_check(&self) {}
    pub fn own_copy(&self) {}
    pub fn own_dump(&self) {}
}

impl Default for IGESGraphToolTextDisplayTemplate {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = IGESGraphToolTextDisplayTemplate::new();
    }
}
