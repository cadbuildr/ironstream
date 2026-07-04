// FILE: iges_graph_tool_definition_level.rs
// occt: IGESGraph_ToolDefinitionLevel

pub struct IGESGraphToolDefinitionLevel;

impl IGESGraphToolDefinitionLevel {
    pub fn new() -> Self {
        IGESGraphToolDefinitionLevel
    }

    pub fn read_own_params(&self) {}
    pub fn write_own_params(&self) {}
    pub fn own_shared(&self) {}
    pub fn dir_checker(&self) {}
    pub fn own_check(&self) {}
    pub fn own_copy(&self) {}
    pub fn own_dump(&self) {}
}

impl Default for IGESGraphToolDefinitionLevel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = IGESGraphToolDefinitionLevel::new();
    }
}
