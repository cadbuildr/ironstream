// FILE: graphic3d_view_affinity.rs
// occt: Graphic3d_ViewAffinity

#[derive(Clone, Copy, Debug, Default)]
pub struct ViewAffinity {
    mask: u32,
}

impl ViewAffinity {
    pub fn new() -> Self { Self { mask: 0xFFFFFFFF } }
    pub fn is_visible(&self, view_id: u32) -> bool {
        if view_id >= 32 { return false; }
        let bit = 1u32 << view_id;
        (self.mask & bit) != 0
    }
    pub fn set_visible_all(&mut self, is_visible: bool) {
        self.mask = if is_visible { 0xFFFFFFFF } else { 0 };
    }
    pub fn set_visible(&mut self, view_id: u32, is_visible: bool) {
        if view_id >= 32 { return; }
        let bit = 1u32 << view_id;
        if is_visible { self.mask |= bit; } else { self.mask &= !bit; }
    }
    pub fn mask(&self) -> u32 { self.mask }
    pub fn set_mask(&mut self, mask: u32) { self.mask = mask; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let a = ViewAffinity::new();
        assert!(a.is_visible(0));
    }
}
