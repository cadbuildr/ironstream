// FILE: t_colgp_sequence_of_vec2d.rs
// occt: TColgp_SequenceOfVec2d

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct TColgpSequenceOfVec2d { data: VecDeque<usize> }

impl TColgpSequenceOfVec2d {
    pub fn new() -> Self { TColgpSequenceOfVec2d { data: VecDeque::new() } }
    pub fn len(&self) -> usize { self.data.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() { let _seq = TColgpSequenceOfVec2d::new(); }
}
