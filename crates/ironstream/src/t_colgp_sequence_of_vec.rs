// FILE: t_colgp_sequence_of_vec.rs
// occt: TColgp_SequenceOfVec

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct TColgpSequenceOfVec { data: VecDeque<usize> }

impl TColgpSequenceOfVec {
    pub fn new() -> Self { TColgpSequenceOfVec { data: VecDeque::new() } }
    pub fn len(&self) -> usize { self.data.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() { let _seq = TColgpSequenceOfVec::new(); }
}
