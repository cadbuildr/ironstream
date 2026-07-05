// FILE: iges_graph_high_light_o.rs
// occt: IGESGraph_HighLight

/// Represents an IGES HighLight entity (Type 406, Form 20).
/// Attaches information that an entity is to be displayed in a system-dependent manner.
pub struct IgesGraphHighLight {
    nb_property_values: i32,
    highlight_status: i32,
}

impl IgesGraphHighLight {
    /// Creates a new empty HighLight entity.
    pub fn new() -> Self {
        IgesGraphHighLight {
            nb_property_values: 0,
            highlight_status: 0,
        }
    }

    /// Sets the fields of the HighLight entity.
    ///
    /// # Arguments
    /// - `nb_props`: Number of property values (should be 1)
    /// - `status`: HighLight status flag (0 = not highlighted, 1 = highlighted)
    pub fn init(&mut self, nb_props: i32, status: i32) {
        self.nb_property_values = nb_props;
        self.highlight_status = status;
    }

    /// Returns the number of property values.
    pub fn nb_property_values(&self) -> i32 {
        self.nb_property_values
    }

    /// Returns the HighLight status.
    /// - 0: Not highlighted (default)
    /// - 1: Highlighted
    pub fn highlight_status(&self) -> i32 {
        self.highlight_status
    }

    /// Returns true if the entity is highlighted.
    pub fn is_highlighted(&self) -> bool {
        self.highlight_status != 0
    }
}

impl Default for IgesGraphHighLight {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_creation() {
        let hl = IgesGraphHighLight::new();
        assert_eq!(hl.nb_property_values(), 0);
        assert_eq!(hl.highlight_status(), 0);
        assert!(!hl.is_highlighted());
    }

    #[test]
    fn test_highlight_init_not_highlighted() {
        let mut hl = IgesGraphHighLight::new();
        hl.init(1, 0);
        assert_eq!(hl.nb_property_values(), 1);
        assert_eq!(hl.highlight_status(), 0);
        assert!(!hl.is_highlighted());
    }

    #[test]
    fn test_highlight_init_highlighted() {
        let mut hl = IgesGraphHighLight::new();
        hl.init(1, 1);
        assert_eq!(hl.nb_property_values(), 1);
        assert_eq!(hl.highlight_status(), 1);
        assert!(hl.is_highlighted());
    }

    #[test]
    fn test_highlight_is_highlighted_with_any_nonzero() {
        let mut hl = IgesGraphHighLight::new();
        hl.init(1, 2);
        assert!(hl.is_highlighted()); // Any non-zero is highlighted
    }
}
