// FILE: t_colgp_sequence_of_array1_of_pnt2d.rs
// occt: TColgp_SequenceOfArray1OfPnt2d

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct TColgpSequenceOfArray1OfPnt2d {
    data: VecDeque<usize>,
}

impl TColgpSequenceOfArray1OfPnt2d {
    pub fn new() -> Self { TColgpSequenceOfArray1OfPnt2d { data: VecDeque::new() } }
    pub fn len(&self) -> usize { self.data.len() }
    pub fn append(&mut self, item: usize) { self.data.push_back(item); }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() { let _seq = TColgpSequenceOfArray1OfPnt2d::new(); }
}
