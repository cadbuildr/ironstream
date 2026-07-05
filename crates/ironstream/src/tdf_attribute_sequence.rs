// FILE: tdf_attribute_sequence.rs
// occt: TDF_AttributeSequence

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct TDFAttributeSequence {
    data: VecDeque<usize>,
}

impl TDFAttributeSequence {
    pub fn new() -> Self { TDFAttributeSequence { data: VecDeque::new() } }
    pub fn len(&self) -> usize { self.data.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() { let _seq = TDFAttributeSequence::new(); }
}
