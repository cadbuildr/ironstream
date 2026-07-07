// FILE: topo_ds_to_step_make_manifold_solid_brep.rs
// occt: TopoDSToStep_MakeManifoldSolidBrep

pub struct MakeManifoldSolidBrep {
    manifold_solid_brep: Option<ManifoldSolidBrep>,
    tessellated_item: Option<TessellatedItem>,
}

pub struct ManifoldSolidBrep;
pub struct TessellatedItem;

impl MakeManifoldSolidBrep {
    pub fn new() -> Self {
        MakeManifoldSolidBrep {
            manifold_solid_brep: None,
            tessellated_item: None,
        }
    }

    pub fn value(&self) -> Option<&ManifoldSolidBrep> {
        self.manifold_solid_brep.as_ref()
    }

    pub fn tessellated_value(&self) -> Option<&TessellatedItem> {
        self.tessellated_item.as_ref()
    }
}

impl Default for MakeManifoldSolidBrep {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let maker = MakeManifoldSolidBrep::new();
        assert!(maker.value().is_none());
    }
}
