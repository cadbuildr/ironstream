// FILE: t_colgp_sequence_of_pnt2d.rs
// occt: TColgp_SequenceOfPnt2d

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct TColgpSequenceOfPnt2d { data: VecDeque<usize> }

impl TColgpSequenceOfPnt2d {
    pub fn new() -> Self { TColgpSequenceOfPnt2d { data: VecDeque::new() } }
    pub fn len(&self) -> usize { self.data.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() { let _seq = TColgpSequenceOfPnt2d::new(); }
}
