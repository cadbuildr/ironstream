// FILE: tdf_attribute_array1.rs
// occt: TDF_AttributeArray1

#[derive(Debug, Clone)]
pub struct TDFAttributeArray1 {
    lower: usize,
    upper: usize,
    items: Vec<usize>,
}

impl TDFAttributeArray1 {
    pub fn new(lower: usize, upper: usize) -> Self {
        if lower > upper { panic!("Invalid bounds"); }
        let size = upper - lower + 1;
        TDFAttributeArray1 { lower, upper, items: vec![0; size] }
    }
    pub fn lower(&self) -> usize { self.lower }
    pub fn upper(&self) -> usize { self.upper }
    pub fn len(&self) -> usize { self.items.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() { let _arr = TDFAttributeArray1::new(1, 2); }
}
