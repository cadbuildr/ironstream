// FILE: iges_graph_tool_intercharacter_spacing.rs
// occt: IGESGraph_ToolIntercharacterSpacing

pub struct IGESGraphToolIntercharacterSpacing;

impl IGESGraphToolIntercharacterSpacing {
    pub fn new() -> Self {
        IGESGraphToolIntercharacterSpacing
    }

    pub fn read_own_params(&self) {}
    pub fn write_own_params(&self) {}
    pub fn own_shared(&self) {}
    pub fn dir_checker(&self) {}
    pub fn own_check(&self) {}
    pub fn own_copy(&self) {}
    pub fn own_dump(&self) {}
}

impl Default for IGESGraphToolIntercharacterSpacing {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = IGESGraphToolIntercharacterSpacing::new();
    }
}
