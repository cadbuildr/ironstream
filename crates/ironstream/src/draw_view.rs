// FILE: draw_view.rs
// occt: Draw_View

//! Draw view for display management.

/// Represents a Draw view
pub struct DrawView {
    id: i32,
    name: String,
}

impl DrawView {
    /// Create a new view
    pub fn new(id: i32, name: impl Into<String>) -> Self {
        DrawView {
            id,
            name: name.into(),
        }
    }

    /// Get the view ID
    pub fn id(&self) -> i32 {
        self.id
    }

    /// Get the view name
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_view_creation() {
        let view = DrawView::new(1, "MainView");
        assert_eq!(view.id(), 1);
        assert_eq!(view.name(), "MainView");
    }
}
