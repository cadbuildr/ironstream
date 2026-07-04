// FILE: if_select_select_in_list.rs
// occt: IFSelect_SelectInList

#[derive(Clone, Debug)]
pub struct IfSelectSelectInList {
    list: Vec<usize>,
}

impl IfSelectSelectInList {
    pub fn new() -> Self {
        IfSelectSelectInList {
            list: vec![],
        }
    }

    pub fn add_to_list(&mut self, item: usize) {
        self.list.push(item);
    }

    pub fn in_list(&self, item: usize) -> bool {
        self.list.contains(&item)
    }

    pub fn list_count(&self) -> usize {
        self.list.len()
    }
}

impl Default for IfSelectSelectInList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let sil = IfSelectSelectInList::new();
        assert_eq!(sil.list_count(), 0);
    }

    #[test]
    fn test_add_to_list() {
        let mut sil = IfSelectSelectInList::new();
        sil.add_to_list(1);
        assert!(sil.in_list(1));
        assert_eq!(sil.list_count(), 1);
    }
}
