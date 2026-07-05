// FILE: t_colgp_sequence_of_xy.rs
// occt: TColgp_SequenceOfXY

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct TColgpSequenceOfXY { data: VecDeque<usize> }

impl TColgpSequenceOfXY {
    pub fn new() -> Self { TColgpSequenceOfXY { data: VecDeque::new() } }
    pub fn len(&self) -> usize { self.data.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() { let _seq = TColgpSequenceOfXY::new(); }
}
