// FILE: topo_ds_to_step_make_faceted_brep.rs
// occt: TopoDSToStep_MakeFacetedBrep

pub struct MakeFacetedBrep {
    faceted_brep: Option<FacetedBrep>,
    tessellated_item: Option<TessellatedItem>,
}

pub struct FacetedBrep;
pub struct TessellatedItem;

impl MakeFacetedBrep {
    pub fn new() -> Self {
        MakeFacetedBrep {
            faceted_brep: None,
            tessellated_item: None,
        }
    }

    pub fn value(&self) -> Option<&FacetedBrep> {
        self.faceted_brep.as_ref()
    }

    pub fn tessellated_value(&self) -> Option<&TessellatedItem> {
        self.tessellated_item.as_ref()
    }
}

impl Default for MakeFacetedBrep {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let maker = MakeFacetedBrep::new();
        assert!(maker.value().is_none());
    }
}
