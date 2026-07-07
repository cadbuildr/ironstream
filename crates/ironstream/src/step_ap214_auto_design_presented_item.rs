// FILE: step_ap214_auto_design_presented_item.rs
// occt: StepAP214_AutoDesignPresentedItem

#[derive(Clone, Debug)]
pub struct AutoDesignPresentedItemSelect {}

#[derive(Clone, Debug)]
pub struct AutoDesignPresentedItem {
    items: Vec<AutoDesignPresentedItemSelect>,
}

impl AutoDesignPresentedItem {
    pub fn new() -> Self {
        AutoDesignPresentedItem {
            items: Vec::new(),
        }
    }

    pub fn init(&mut self, items: Vec<AutoDesignPresentedItemSelect>) {
        self.items = items;
    }

    pub fn set_items(&mut self, items: Vec<AutoDesignPresentedItemSelect>) {
        self.items = items;
    }

    pub fn items(&self) -> &[AutoDesignPresentedItemSelect] {
        &self.items
    }

    pub fn items_value(&self, num: usize) -> Option<&AutoDesignPresentedItemSelect> {
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

impl Default for AutoDesignPresentedItem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let item = AutoDesignPresentedItem::new();
        assert_eq!(item.nb_items(), 0);
    }

    #[test]
    fn test_set_items() {
        let mut item = AutoDesignPresentedItem::new();
        let items = vec![AutoDesignPresentedItemSelect {}];
        item.set_items(items);
        assert_eq!(item.nb_items(), 1);
    }
}
