// FILE: step_visual_invisibility.rs
// occt: StepVisual_Invisibility

/// Invisibility specification in STEP representation.
///
/// This defines invisible items and their visibility context.
pub struct Invisibility {
    invisible_items: Vec<i32>,
}

impl Invisibility {
    /// Creates a new invisibility specification.
    pub fn new() -> Self {
        Invisibility {
            invisible_items: Vec::new(),
        }
    }

    /// Adds an invisible item.
    pub fn add_invisible_item(&mut self, item_id: i32) {
        self.invisible_items.push(item_id);
    }

    /// Returns the invisible items.
    pub fn invisible_items(&self) -> &[i32] {
        &self.invisible_items
    }

    /// Sets the invisible items.
    pub fn set_invisible_items(&mut self, items: Vec<i32>) {
        self.invisible_items = items;
    }

    /// Returns the number of invisible items.
    pub fn nb_invisible_items(&self) -> usize {
        self.invisible_items.len()
    }
}

impl Default for Invisibility {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invisibility_new() {
        let inv = Invisibility::new();
        assert_eq!(inv.nb_invisible_items(), 0);
    }

    #[test]
    fn test_add_invisible_item() {
        let mut inv = Invisibility::new();
        inv.add_invisible_item(1);
        inv.add_invisible_item(2);
        assert_eq!(inv.nb_invisible_items(), 2);
        assert_eq!(inv.invisible_items(), &[1, 2]);
    }

    #[test]
    fn test_set_invisible_items() {
        let mut inv = Invisibility::new();
        inv.set_invisible_items(vec![10, 20, 30]);
        assert_eq!(inv.nb_invisible_items(), 3);
    }
}
