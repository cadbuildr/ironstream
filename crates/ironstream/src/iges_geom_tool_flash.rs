// FILE: iges_geom_tool_flash.rs
// occt: IGESGeom_ToolFlash

pub struct ToolFlash;

impl ToolFlash {
    pub fn new() -> Self {
        ToolFlash
    }
}

impl Default for ToolFlash {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = ToolFlash::new();
    }
}
