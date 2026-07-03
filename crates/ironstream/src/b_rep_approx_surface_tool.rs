// FILE: b_rep_approx_surface_tool.rs
// occt: BRepApprox_SurfaceTool

pub struct BrepapproxSurfacetool;

impl BrepapproxSurfacetool {
    pub fn new() -> Self {
        BrepapproxSurfacetool
    }
}

impl Default for BrepapproxSurfacetool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BrepapproxSurfacetool::new();
    }
}
