// FILE: iges_solid_tool_loop.rs
// occt: IGESSolid_ToolLoop

pub struct IGESSolidToolLoop;

impl IGESSolidToolLoop {
    pub fn new() -> Self {
        IGESSolidToolLoop
    }

    pub fn label(&self) -> &str {
        "Loop"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _t = IGESSolidToolLoop::new();
    }

    #[test]
    fn test_label() {
        assert_eq!(IGESSolidToolLoop::new().label(), "Loop");
    }
}
