// FILE: step_visual_presented_item.rs
// occt: StepVisual_PresentedItem

pub struct PresentedItem {
    _data: (),
}

impl PresentedItem {
    pub fn new() -> Self {
        PresentedItem { _data: () }
    }
}

impl Default for PresentedItem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let pi = PresentedItem::new();
        let _pi2 = PresentedItem::new();
        let _ = pi;
    }

    #[test]
    fn test_default() {
        let pi = PresentedItem::default();
        let _pi2 = PresentedItem::new();
        let _ = pi;
    }
}
