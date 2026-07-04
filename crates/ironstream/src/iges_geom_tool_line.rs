// FILE: iges_geom_tool_line.rs
// occt: IGESGeom_ToolLine

/// Tool to work on a Line. Called by various Modules.
pub struct ToolLine;

impl ToolLine {
    pub fn new() -> Self {
        ToolLine
    }
}

impl Default for ToolLine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = ToolLine::new();
    }
}
