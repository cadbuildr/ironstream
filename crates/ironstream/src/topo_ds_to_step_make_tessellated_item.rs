// FILE: topo_ds_to_step_make_tessellated_item.rs
// occt: TopoDSToStep_MakeTessellatedItem

pub struct MakeTessellatedItem {
    tessellated_item: Option<TessellatedItem>,
}

pub struct TessellatedItem;

impl MakeTessellatedItem {
    pub fn new() -> Self {
        MakeTessellatedItem {
            tessellated_item: None,
        }
    }

    pub fn value(&self) -> Option<&TessellatedItem> {
        self.tessellated_item.as_ref()
    }
}

impl Default for MakeTessellatedItem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let maker = MakeTessellatedItem::new();
        assert!(maker.value().is_none());
    }
}
