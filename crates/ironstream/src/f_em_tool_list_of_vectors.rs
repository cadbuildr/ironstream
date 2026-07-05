// FILE: f_em_tool_list_of_vectors.rs
// occt: FEmTool_ListOfVectors

#[derive(Clone, Debug)]
pub struct Vector {}

#[derive(Clone, Debug)]
pub struct ListOfVectors {
    items: Vec<Vector>,
}

impl ListOfVectors {
    pub fn new() -> Self { ListOfVectors { items: Vec::new() } }
    pub fn append(&mut self, item: Vector) { self.items.push(item); }
    pub fn len(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
}

impl Default for ListOfVectors {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_list_creation() {
        let list = ListOfVectors::new();
        assert!(list.is_empty());
    }
}
