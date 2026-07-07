// FILE: t_colgp_sequence_of_dir2d.rs
// occt: TColgp_SequenceOfDir2d

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct TColgpSequenceOfDir2d { data: VecDeque<usize> }

impl TColgpSequenceOfDir2d {
    pub fn new() -> Self { TColgpSequenceOfDir2d { data: VecDeque::new() } }
    pub fn len(&self) -> usize { self.data.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() { let _seq = TColgpSequenceOfDir2d::new(); }
}
