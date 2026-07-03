// FILE: ais_graphic_tool.rs
// occt: AIS_GraphicTool

#[derive(Clone, Debug, Default)]
pub struct GraphicTool;

impl GraphicTool {
    pub fn new() -> Self { Self }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _t = GraphicTool::new();
    }
}
