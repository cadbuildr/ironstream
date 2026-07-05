// FILE: t_colgp_sequence_of_dir.rs
// occt: TColgp_SequenceOfDir

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct TColgpSequenceOfDir { data: VecDeque<usize> }

impl TColgpSequenceOfDir {
    pub fn new() -> Self { TColgpSequenceOfDir { data: VecDeque::new() } }
    pub fn len(&self) -> usize { self.data.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() { let _seq = TColgpSequenceOfDir::new(); }
}
