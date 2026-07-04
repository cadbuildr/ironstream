// FILE: step_visual_tessellated_solid.rs
// occt: StepVisual_TessellatedSolid

/// Represents a STEP TessellatedSolid entity.
/// A solid tessellated as a collection of structured items.
pub struct TessellatedSolid {
    name: String,
    items: Vec<TessellatedStructuredItem>,
    geometric_link: Option<ManifoldSolidBrep>,
    has_geometric_link: bool,
}

/// Placeholder for TessellatedStructuredItem
pub struct TessellatedStructuredItem;

/// Placeholder for ManifoldSolidBrep
pub struct ManifoldSolidBrep;

impl TessellatedSolid {
    /// Creates a new tessellated solid.
    pub fn new() -> Self {
        TessellatedSolid {
            name: String::new(),
            items: Vec::new(),
            geometric_link: None,
            has_geometric_link: false,
        }
    }

    /// Initializes all fields.
    pub fn init(
        &mut self,
        name: String,
        items: Vec<TessellatedStructuredItem>,
        has_geometric_link: bool,
        geometric_link: Option<ManifoldSolidBrep>,
    ) {
        self.name = name;
        self.items = items;
        self.has_geometric_link = has_geometric_link;
        self.geometric_link = geometric_link;
    }

    /// Returns the items.
    pub fn items(&self) -> &[TessellatedStructuredItem] {
        &self.items
    }

    /// Sets the items.
    pub fn set_items(&mut self, items: Vec<TessellatedStructuredItem>) {
        self.items = items;
    }

    /// Returns the number of items.
    pub fn nb_items(&self) -> usize {
        self.items.len()
    }

    /// Returns the item at the given index.
    pub fn items_value(&self, idx: usize) -> Option<&TessellatedStructuredItem> {
        self.items.get(idx)
    }

    /// Returns the geometric link.
    pub fn geometric_link(&self) -> Option<&ManifoldSolidBrep> {
        self.geometric_link.as_ref()
    }

    /// Sets the geometric link.
    pub fn set_geometric_link(&mut self, link: ManifoldSolidBrep) {
        self.geometric_link = Some(link);
        self.has_geometric_link = true;
    }

    /// Returns true if geometric link is defined.
    pub fn has_geometric_link(&self) -> bool {
        self.has_geometric_link
    }
}

impl Default for TessellatedSolid {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let ts = TessellatedSolid::new();
        assert_eq!(ts.nb_items(), 0);
        assert!(!ts.has_geometric_link());
    }

    #[test]
    fn test_init() {
        let mut ts = TessellatedSolid::new();
        ts.init("MyTessellatedSolid".to_string(), vec![], false, None);
        assert_eq!(ts.name, "MyTessellatedSolid");
        assert_eq!(ts.nb_items(), 0);
        assert!(!ts.has_geometric_link());
    }

    #[test]
    fn test_items() {
        let mut ts = TessellatedSolid::new();
        let items = vec![];
        ts.set_items(items);
        assert_eq!(ts.nb_items(), 0);
    }

    #[test]
    fn test_geometric_link() {
        let mut ts = TessellatedSolid::new();
        let link = ManifoldSolidBrep;
        ts.set_geometric_link(link);
        assert!(ts.has_geometric_link());
        assert!(ts.geometric_link().is_some());
    }
}
