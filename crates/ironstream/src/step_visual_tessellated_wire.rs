// FILE: step_visual_tessellated_wire.rs
// occt: StepVisual_TessellatedWire

/// Represents a STEP TessellatedWire entity.
pub struct TessellatedWire {
    name: String,
    items: Vec<TessellatedEdgeOrVertex>,
    geometric_model_link: Option<PathOrCompositeCurve>,
    has_geometric_model_link: bool,
}

/// Placeholder for TessellatedEdgeOrVertex
pub struct TessellatedEdgeOrVertex;

/// Placeholder for PathOrCompositeCurve
pub struct PathOrCompositeCurve;

impl TessellatedWire {
    /// Creates a new tessellated wire.
    pub fn new() -> Self {
        TessellatedWire {
            name: String::new(),
            items: Vec::new(),
            geometric_model_link: None,
            has_geometric_model_link: false,
        }
    }

    /// Initializes all fields.
    pub fn init(
        &mut self,
        name: String,
        items: Vec<TessellatedEdgeOrVertex>,
        has_geometric_model_link: bool,
        geometric_model_link: Option<PathOrCompositeCurve>,
    ) {
        self.name = name;
        self.items = items;
        self.has_geometric_model_link = has_geometric_model_link;
        self.geometric_model_link = geometric_model_link;
    }

    /// Returns the items.
    pub fn items(&self) -> &[TessellatedEdgeOrVertex] {
        &self.items
    }

    /// Sets the items.
    pub fn set_items(&mut self, items: Vec<TessellatedEdgeOrVertex>) {
        self.items = items;
    }

    /// Returns the number of items.
    pub fn nb_items(&self) -> usize {
        self.items.len()
    }

    /// Returns the item at the given index.
    pub fn items_value(&self, idx: usize) -> Option<&TessellatedEdgeOrVertex> {
        self.items.get(idx)
    }

    /// Returns the geometric model link.
    pub fn geometric_model_link(&self) -> Option<&PathOrCompositeCurve> {
        self.geometric_model_link.as_ref()
    }

    /// Sets the geometric model link.
    pub fn set_geometric_model_link(&mut self, link: PathOrCompositeCurve) {
        self.geometric_model_link = Some(link);
        self.has_geometric_model_link = true;
    }

    /// Returns true if geometric model link is defined.
    pub fn has_geometric_model_link(&self) -> bool {
        self.has_geometric_model_link
    }
}

impl Default for TessellatedWire {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tw = TessellatedWire::new();
        assert_eq!(tw.nb_items(), 0);
        assert!(!tw.has_geometric_model_link());
    }

    #[test]
    fn test_items() {
        let mut tw = TessellatedWire::new();
        let items = vec![];
        tw.set_items(items);
        assert_eq!(tw.nb_items(), 0);
    }

    #[test]
    fn test_geometric_model_link() {
        let mut tw = TessellatedWire::new();
        let link = PathOrCompositeCurve;
        tw.set_geometric_model_link(link);
        assert!(tw.has_geometric_model_link());
        assert!(tw.geometric_model_link().is_some());
    }
}
