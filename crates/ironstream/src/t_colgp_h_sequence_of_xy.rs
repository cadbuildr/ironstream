// FILE: t_colgp_h_sequence_of_xy.rs
// occt: TColgp_HSequenceOfXY

use std::sync::Arc;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
pub struct XY { pub x: f64, pub y: f64 }

#[derive(Debug, Clone)]
pub struct TColgpHSequenceOfXY {
    data: Arc<Data>,
}

#[derive(Debug)]
struct Data {
    items: VecDeque<XY>,
}

impl TColgpHSequenceOfXY {
    pub fn new() -> Self { TColgpHSequenceOfXY { data: Arc::new(Data { items: VecDeque::new() }) } }
    pub fn len(&self) -> usize { self.data.items.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() { let _seq = TColgpHSequenceOfXY::new(); }
}
