// FILE: tdf_attribute_map.rs
// occt: TDF_AttributeMap

use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct TDFAttributeMap {
    data: HashSet<usize>,
}

impl TDFAttributeMap {
    pub fn new() -> Self { TDFAttributeMap { data: HashSet::new() } }
    pub fn len(&self) -> usize { self.data.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() { let _map = TDFAttributeMap::new(); }
}
