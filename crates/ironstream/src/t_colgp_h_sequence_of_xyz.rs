// FILE: t_colgp_h_sequence_of_xyz.rs
// occt: TColgp_HSequenceOfXYZ

use std::sync::Arc;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
pub struct XYZ { pub x: f64, pub y: f64, pub z: f64 }

#[derive(Debug, Clone)]
pub struct TColgpHSequenceOfXYZ {
    data: Arc<Data>,
}

#[derive(Debug)]
struct Data {
    items: VecDeque<XYZ>,
}

impl TColgpHSequenceOfXYZ {
    pub fn new() -> Self { TColgpHSequenceOfXYZ { data: Arc::new(Data { items: VecDeque::new() }) } }
    pub fn len(&self) -> usize { self.data.items.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() { let _seq = TColgpHSequenceOfXYZ::new(); }
}
