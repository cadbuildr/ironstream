// FILE: step_ap214_applied_presented_item.rs
// occt: StepAP214_AppliedPresentedItem

#[derive(Clone, Debug)]
pub struct PresentedItemSelect {}

#[derive(Clone, Debug)]
pub struct AppliedPresentedItem {
    items: Vec<PresentedItemSelect>,
}

impl AppliedPresentedItem {
    pub fn new() -> Self {
        AppliedPresentedItem {
            items: Vec::new(),
        }
    }

    pub fn init(&mut self, items: Vec<PresentedItemSelect>) {
        self.items = items;
    }

    pub fn set_items(&mut self, items: Vec<PresentedItemSelect>) {
        self.items = items;
    }

    pub fn items(&self) -> &[PresentedItemSelect] {
        &self.items
    }

    pub fn items_value(&self, num: usize) -> Option<&PresentedItemSelect> {
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

impl Default for AppliedPresentedItem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let item = AppliedPresentedItem::new();
        assert_eq!(item.nb_items(), 0);
    }

    #[test]
    fn test_set_items() {
        let mut item = AppliedPresentedItem::new();
        let items = vec![PresentedItemSelect {}];
        item.set_items(items);
        assert_eq!(item.nb_items(), 1);
    }
}
