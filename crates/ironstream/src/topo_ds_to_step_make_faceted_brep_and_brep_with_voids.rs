// FILE: topo_ds_to_step_make_faceted_brep_and_brep_with_voids.rs
// occt: TopoDSToStep_MakeFacetedBrepAndBrepWithVoids

pub struct MakeFacetedBrepAndBrepWithVoids {
    faceted_brep_and_brep_with_voids: Option<FacetedBrepAndBrepWithVoids>,
    tessellated_item: Option<TessellatedItem>,
}

pub struct FacetedBrepAndBrepWithVoids;
pub struct TessellatedItem;

impl MakeFacetedBrepAndBrepWithVoids {
    pub fn new() -> Self {
        MakeFacetedBrepAndBrepWithVoids {
            faceted_brep_and_brep_with_voids: None,
            tessellated_item: None,
        }
    }

    pub fn value(&self) -> Option<&FacetedBrepAndBrepWithVoids> {
        self.faceted_brep_and_brep_with_voids.as_ref()
    }

    pub fn tessellated_value(&self) -> Option<&TessellatedItem> {
        self.tessellated_item.as_ref()
    }
}

impl Default for MakeFacetedBrepAndBrepWithVoids {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let maker = MakeFacetedBrepAndBrepWithVoids::new();
        assert!(maker.value().is_none());
    }
}
