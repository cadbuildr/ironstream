// FILE: t_colgp_sequence_of_xyz.rs
// occt: TColgp_SequenceOfXYZ

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct TColgpSequenceOfXYZ { data: VecDeque<usize> }

impl TColgpSequenceOfXYZ {
    pub fn new() -> Self { TColgpSequenceOfXYZ { data: VecDeque::new() } }
    pub fn len(&self) -> usize { self.data.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() { let _seq = TColgpSequenceOfXYZ::new(); }
}
