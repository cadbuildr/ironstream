// FILE: step_repr_representation.rs
// occt: StepRepr_Representation

/// Placeholder for RepresentationItem
#[derive(Clone, Debug, PartialEq)]
pub struct RepresentationItem {
    name: String,
}

/// Placeholder for RepresentationContext
#[derive(Clone, Debug, PartialEq)]
pub struct RepresentationContext {
    name: String,
}

/// Represents a STEP representation object containing items and context information.
pub struct Representation {
    name: Option<String>,
    items: Vec<RepresentationItem>,
    context_of_items: Option<RepresentationContext>,
}

impl Representation {
    /// Create a new Representation
    pub fn new() -> Self {
        Representation {
            name: None,
            items: Vec::new(),
            context_of_items: None,
        }
    }

    /// Initialize representation with name, items, and context
    pub fn init(
        &mut self,
        name: String,
        items: Vec<RepresentationItem>,
        context_of_items: RepresentationContext,
    ) {
        self.name = Some(name);
        self.items = items;
        self.context_of_items = Some(context_of_items);
    }

    /// Set the name
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// Get the name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Set the items
    pub fn set_items(&mut self, items: Vec<RepresentationItem>) {
        self.items = items;
    }

    /// Get the items as a slice
    pub fn items(&self) -> &[RepresentationItem] {
        &self.items
    }

    /// Get an item by index (1-based for STEP compatibility)
    pub fn items_value(&self, num: usize) -> Option<&RepresentationItem> {
        if num > 0 && num <= self.items.len() {
            Some(&self.items[num - 1])
        } else {
            None
        }
    }

    /// Get the number of items
    pub fn nb_items(&self) -> usize {
        self.items.len()
    }

    /// Set the context of items
    pub fn set_context_of_items(&mut self, context: RepresentationContext) {
        self.context_of_items = Some(context);
    }

    /// Get the context of items
    pub fn context_of_items(&self) -> Option<&RepresentationContext> {
        self.context_of_items.as_ref()
    }
}

impl Default for Representation {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_default() {
        let rep = Representation::new();
        assert_eq!(rep.name(), None);
        assert_eq!(rep.nb_items(), 0);
        assert_eq!(rep.context_of_items(), None);
    }

    #[test]
    fn test_set_and_get_name() {
        let mut rep = Representation::new();
        rep.set_name("TestRepresentation".to_string());
        assert_eq!(rep.name(), Some("TestRepresentation"));
    }

    #[test]
    fn test_set_and_get_items() {
        let mut rep = Representation::new();
        let item1 = RepresentationItem {
            name: "item1".to_string(),
        };
        let item2 = RepresentationItem {
            name: "item2".to_string(),
        };
        rep.set_items(vec![item1.clone(), item2.clone()]);
        assert_eq!(rep.nb_items(), 2);
        assert_eq!(rep.items_value(1), Some(&item1));
        assert_eq!(rep.items_value(2), Some(&item2));
    }

    #[test]
    fn test_set_context_of_items() {
        let mut rep = Representation::new();
        let ctx = RepresentationContext {
            name: "context1".to_string(),
        };
        rep.set_context_of_items(ctx.clone());
        assert_eq!(rep.context_of_items(), Some(&ctx));
    }

    #[test]
    fn test_init() {
        let mut rep = Representation::new();
        let item = RepresentationItem {
            name: "item".to_string(),
        };
        let ctx = RepresentationContext {
            name: "ctx".to_string(),
        };
        rep.init("TestRep".to_string(), vec![item], ctx);
        assert_eq!(rep.name(), Some("TestRep"));
        assert_eq!(rep.nb_items(), 1);
        assert!(rep.context_of_items().is_some());
    }
}
