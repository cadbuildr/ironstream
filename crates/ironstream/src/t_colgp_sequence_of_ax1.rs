// FILE: t_colgp_sequence_of_ax1.rs
// occt: TColgp_SequenceOfAx1

use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct TColgpSequenceOfAx1 {
    data: VecDeque<usize>,
}

impl TColgpSequenceOfAx1 {
    pub fn new() -> Self { TColgpSequenceOfAx1 { data: VecDeque::new() } }
    pub fn len(&self) -> usize { self.data.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() { let _seq = TColgpSequenceOfAx1::new(); }
}
