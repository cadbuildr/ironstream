// FILE: mat_list_of_bisector.rs
// occt: MAT_ListOfBisector

/// List of bisectors.
#[derive(Debug, Clone)]
pub struct MatListOfBisector {
    nb_items: i32,
}

impl MatListOfBisector {
    pub fn new() -> Self {
        MatListOfBisector { nb_items: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.nb_items == 0
    }

    pub fn number(&self) -> i32 {
        self.nb_items
    }
}

impl Default for MatListOfBisector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_bisector() {
        let list = MatListOfBisector::new();
        assert!(list.is_empty());
    }
}
