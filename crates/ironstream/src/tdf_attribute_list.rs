// FILE: tdf_attribute_list.rs
// occt: TDF_AttributeList

use std::collections::LinkedList;

#[derive(Debug, Clone)]
pub struct TDFAttributeList {
    data: LinkedList<usize>,
}

impl TDFAttributeList {
    pub fn new() -> Self { TDFAttributeList { data: LinkedList::new() } }
    pub fn len(&self) -> usize { self.data.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() { let _list = TDFAttributeList::new(); }
}
