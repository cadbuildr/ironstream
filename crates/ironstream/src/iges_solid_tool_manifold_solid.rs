// FILE: iges_solid_tool_manifold_solid.rs
// occt: IGESSolid_ToolManifoldSolid

pub struct IGESSolidToolManifoldSolid;

impl IGESSolidToolManifoldSolid {
    pub fn new() -> Self {
        IGESSolidToolManifoldSolid
    }

    pub fn label(&self) -> &str {
        "ManifoldSolid"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let _t = IGESSolidToolManifoldSolid::new();
    }

    #[test]
    fn test_label() {
        assert_eq!(IGESSolidToolManifoldSolid::new().label(), "ManifoldSolid");
    }
}
