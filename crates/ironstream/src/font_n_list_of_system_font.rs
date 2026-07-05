// FILE: font_n_list_of_system_font.rs
// occt: Font_NListOfSystemFont

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub struct SystemFont {}

pub type SystemFontHandle = Rc<RefCell<SystemFont>>;

#[derive(Clone, Debug)]
pub struct NListOfSystemFont {
    items: Vec<SystemFontHandle>,
}

impl NListOfSystemFont {
    pub fn new() -> Self { NListOfSystemFont { items: Vec::new() } }
    pub fn append(&mut self, item: SystemFontHandle) { self.items.push(item); }
    pub fn len(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
}

impl Default for NListOfSystemFont {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_list_creation() {
        let list = NListOfSystemFont::new();
        assert!(list.is_empty());
    }
}
