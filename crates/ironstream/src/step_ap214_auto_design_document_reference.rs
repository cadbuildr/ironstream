// FILE: step_ap214_auto_design_document_reference.rs
// occt: StepAP214_AutoDesignDocumentReference

#[derive(Clone, Debug)]
pub struct AutoDesignReferencingItem {}

#[derive(Clone, Debug)]
pub struct AutoDesignDocumentReference {
    items: Vec<AutoDesignReferencingItem>,
}

impl AutoDesignDocumentReference {
    pub fn new() -> Self {
        AutoDesignDocumentReference {
            items: Vec::new(),
        }
    }

    pub fn init(&mut self, items: Vec<AutoDesignReferencingItem>) {
        self.items = items;
    }

    pub fn set_items(&mut self, items: Vec<AutoDesignReferencingItem>) {
        self.items = items;
    }

    pub fn items(&self) -> &[AutoDesignReferencingItem] {
        &self.items
    }

    pub fn items_value(&self, num: usize) -> Option<&AutoDesignReferencingItem> {
        if num > 0 && num <= self.items.len() {
            Some(&self.items[num - 1])
        } else {
            None
        }
    }

    pub fn nb_items(&self) -> usize {
        self.items.len()
    }
}

impl Default for AutoDesignDocumentReference {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let reference = AutoDesignDocumentReference::new();
        assert_eq!(reference.nb_items(), 0);
    }

    #[test]
    fn test_set_items() {
        let mut reference = AutoDesignDocumentReference::new();
        let items = vec![AutoDesignReferencingItem {}];
        reference.set_items(items);
        assert_eq!(reference.nb_items(), 1);
    }
}
