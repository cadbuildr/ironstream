// FILE: tdf_attribute_data_map.rs
// occt: TDF_AttributeDataMap

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TDFAttributeDataMap {
    data: HashMap<usize, usize>,
}

impl TDFAttributeDataMap {
    pub fn new() -> Self { TDFAttributeDataMap { data: HashMap::new() } }
    pub fn len(&self) -> usize { self.data.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() { let _map = TDFAttributeDataMap::new(); }
}
