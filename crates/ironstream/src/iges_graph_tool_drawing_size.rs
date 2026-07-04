// FILE: iges_graph_tool_drawing_size.rs
// occt: IGESGraph_ToolDrawingSize

pub struct IGESGraphToolDrawingSize;

impl IGESGraphToolDrawingSize {
    pub fn new() -> Self {
        IGESGraphToolDrawingSize
    }

    pub fn read_own_params(&self) {}
    pub fn write_own_params(&self) {}
    pub fn own_shared(&self) {}
    pub fn dir_checker(&self) {}
    pub fn own_check(&self) {}
    pub fn own_copy(&self) {}
    pub fn own_dump(&self) {}
}

impl Default for IGESGraphToolDrawingSize {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _tool = IGESGraphToolDrawingSize::new();
    }
}
