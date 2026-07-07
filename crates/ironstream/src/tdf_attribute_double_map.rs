// FILE: tdf_attribute_double_map.rs
// occt: TDF_AttributeDoubleMap

#[derive(Debug, Clone)]
pub struct TDFAttributeDoubleMap {
    data: std::collections::HashMap<usize, usize>,
}

impl TDFAttributeDoubleMap {
    pub fn new() -> Self { TDFAttributeDoubleMap { data: std::collections::HashMap::new() } }
    pub fn len(&self) -> usize { self.data.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() { let _map = TDFAttributeDoubleMap::new(); }
}
