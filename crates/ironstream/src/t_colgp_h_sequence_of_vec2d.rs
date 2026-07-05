// FILE: t_colgp_h_sequence_of_vec2d.rs
// occt: TColgp_HSequenceOfVec2d

use std::sync::Arc;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
pub struct Vec2d { pub x: f64, pub y: f64 }

#[derive(Debug, Clone)]
pub struct TColgpHSequenceOfVec2d {
    data: Arc<Data>,
}

#[derive(Debug)]
struct Data {
    items: VecDeque<Vec2d>,
}

impl TColgpHSequenceOfVec2d {
    pub fn new() -> Self { TColgpHSequenceOfVec2d { data: Arc::new(Data { items: VecDeque::new() }) } }
    pub fn len(&self) -> usize { self.data.items.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() { let _seq = TColgpHSequenceOfVec2d::new(); }
}
