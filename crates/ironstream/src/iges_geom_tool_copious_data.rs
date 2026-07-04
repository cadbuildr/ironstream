// FILE: iges_geom_tool_copious_data.rs
// occt: IGESGeom_ToolCopiousData

pub struct ToolCopiousData;

impl ToolCopiousData {
    pub fn new() -> Self {
        ToolCopiousData
    }
}

impl Default for ToolCopiousData {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = ToolCopiousData::new();
    }
}
