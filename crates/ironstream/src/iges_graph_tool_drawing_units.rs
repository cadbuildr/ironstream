// FILE: iges_graph_tool_drawing_units.rs
// occt: IGESGraph_ToolDrawingUnits

pub struct IGESGraphToolDrawingUnits;

impl IGESGraphToolDrawingUnits {
    pub fn new() -> Self {
        IGESGraphToolDrawingUnits
    }

    pub fn read_own_params(&self) {}
    pub fn write_own_params(&self) {}
    pub fn own_shared(&self) {}
    pub fn dir_checker(&self) {}
    pub fn own_check(&self) {}
    pub fn own_copy(&self) {}
    pub fn own_dump(&self) {}
}

impl Default for IGESGraphToolDrawingUnits {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = IGESGraphToolDrawingUnits::new();
    }
}
