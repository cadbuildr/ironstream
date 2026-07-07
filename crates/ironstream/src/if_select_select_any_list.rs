// FILE: if_select_select_any_list.rs
// occt: IFSelect_SelectAnyList

#[derive(Clone, Debug)]
pub struct IfSelectSelectAnyList {
    items: Vec<usize>,
}

impl IfSelectSelectAnyList {
    pub fn new() -> Self {
        IfSelectSelectAnyList {
            items: vec![],
        }
    }

    pub fn add_item(&mut self, item: usize) {
        self.items.push(item);
    }

    pub fn item_count(&self) -> usize {
        self.items.len()
    }
}

impl Default for IfSelectSelectAnyList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let sal = IfSelectSelectAnyList::new();
        assert_eq!(sal.item_count(), 0);
    }

    #[test]
    fn test_add_item() {
        let mut sal = IfSelectSelectAnyList::new();
        sal.add_item(1);
        assert_eq!(sal.item_count(), 1);
    }
}
