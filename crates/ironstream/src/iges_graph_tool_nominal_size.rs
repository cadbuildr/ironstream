// FILE: iges_graph_tool_nominal_size.rs
// occt: IGESGraph_ToolNominalSize

pub struct IGESGraphToolNominalSize;

impl IGESGraphToolNominalSize {
    pub fn new() -> Self {
        IGESGraphToolNominalSize
    }

    pub fn read_own_params(&self) {}
    pub fn write_own_params(&self) {}
    pub fn own_shared(&self) {}
    pub fn dir_checker(&self) {}
    pub fn own_check(&self) {}
    pub fn own_copy(&self) {}
    pub fn own_dump(&self) {}
}

impl Default for IGESGraphToolNominalSize {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = IGESGraphToolNominalSize::new();
    }
}
