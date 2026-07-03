// FILE: mat_t_list_node_of_list_of_bisector.rs
// occt: MAT_TListNodeOfListOfBisector

/// List node for bisector lists.
#[derive(Debug, Clone)]
pub struct MatTListNodeOfListOfBisector;

impl MatTListNodeOfListOfBisector {
    pub fn new() -> Self {
        MatTListNodeOfListOfBisector
    }
}

impl Default for MatTListNodeOfListOfBisector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_node() {
        let _ln = MatTListNodeOfListOfBisector::new();
    }
}
