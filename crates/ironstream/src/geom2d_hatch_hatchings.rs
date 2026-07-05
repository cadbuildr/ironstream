// FILE: geom2d_hatch_hatchings.rs
// occt: Geom2dHatch_Hatchings

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct Hatching {}

pub type HatchingHandle = Rc<RefCell<Hatching>>;

#[derive(Clone, Debug)]
pub struct Hatchings {
    items: Vec<HatchingHandle>,
}

impl Hatchings {
    pub fn new() -> Self { Hatchings { items: Vec::new() } }
    pub fn append(&mut self, item: HatchingHandle) { self.items.push(item); }
    pub fn len(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
}

impl Default for Hatchings {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hatchings_creation() {
        let h = Hatchings::new();
        assert!(h.is_empty());
    }
}
