// FILE: tdf_attribute_delta_list.rs
// occt: TDF_AttributeDeltaList

use std::collections::LinkedList;

#[derive(Debug, Clone)]
pub struct TDFAttributeDeltaList {
    data: LinkedList<usize>,
}

impl TDFAttributeDeltaList {
    pub fn new() -> Self { TDFAttributeDeltaList { data: LinkedList::new() } }
    pub fn len(&self) -> usize { self.data.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() { let _list = TDFAttributeDeltaList::new(); }
}
