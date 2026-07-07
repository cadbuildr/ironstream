// FILE: iges_graph_tool_high_light.rs
// occt: IGESGraph_ToolHighLight

pub struct IGESGraphToolHighLight;

impl IGESGraphToolHighLight {
    pub fn new() -> Self {
        IGESGraphToolHighLight
    }

    pub fn read_own_params(&self) {}
    pub fn write_own_params(&self) {}
    pub fn own_shared(&self) {}
    pub fn dir_checker(&self) {}
    pub fn own_check(&self) {}
    pub fn own_copy(&self) {}
    pub fn own_dump(&self) {}
}

impl Default for IGESGraphToolHighLight {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = IGESGraphToolHighLight::new();
    }
}
