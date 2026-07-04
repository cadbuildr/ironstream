// FILE: topo_ds_to_step_make_brep_with_voids.rs
// occt: TopoDSToStep_MakeBrepWithVoids

/// Class implementing the mapping between TopoDS Solid and STEP BrepWithVoids.
pub struct MakeBrepWithVoids {
    brep_with_voids: Option<BrepWithVoids>,
    tessellated_item: Option<TessellatedItem>,
}

pub struct BrepWithVoids;
pub struct TessellatedItem;

impl MakeBrepWithVoids {
    pub fn new() -> Self {
        MakeBrepWithVoids {
            brep_with_voids: None,
            tessellated_item: None,
        }
    }

    pub fn value(&self) -> Option<&BrepWithVoids> {
        self.brep_with_voids.as_ref()
    }

    pub fn tessellated_value(&self) -> Option<&TessellatedItem> {
        self.tessellated_item.as_ref()
    }
}

impl Default for MakeBrepWithVoids {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let maker = MakeBrepWithVoids::new();
        assert!(maker.value().is_none());
        assert!(maker.tessellated_value().is_none());
    }
}
