// FILE: if_select_list_editor.rs
// occt: IFSelect_ListEditor

#[derive(Clone, Debug)]
pub struct IfSelectListEditor {
    items: Vec<String>,
}

impl IfSelectListEditor {
    pub fn new() -> Self {
        IfSelectListEditor {
            items: vec![],
        }
    }

    pub fn add_item(&mut self, item: &str) {
        self.items.push(item.to_string());
    }

    pub fn remove_item(&mut self, item: &str) {
        self.items.retain(|i| i != item);
    }

    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    pub fn items(&self) -> &[String] {
        &self.items
    }
}

impl Default for IfSelectListEditor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let le = IfSelectListEditor::new();
        assert_eq!(le.item_count(), 0);
    }

    #[test]
    fn test_add_item() {
        let mut le = IfSelectListEditor::new();
        le.add_item("item1");
        assert_eq!(le.item_count(), 1);
    }

    #[test]
    fn test_remove_item() {
        let mut le = IfSelectListEditor::new();
        le.add_item("item1");
        le.remove_item("item1");
        assert_eq!(le.item_count(), 0);
    }
}
