// FILE: step_visual_direction_count_select.rs
// occt: StepVisual_DirectionCountSelect

/// A direction count select in STEP representation.
///
/// This selects between U and V direction counts.
pub struct DirectionCountSelect {
    u_direction_count: i32,
    v_direction_count: i32,
    type_of_content: i32,
}

impl DirectionCountSelect {
    /// Creates a new direction count select.
    pub fn new() -> Self {
        DirectionCountSelect {
            u_direction_count: 0,
            v_direction_count: 0,
            type_of_content: 0,
        }
    }

    /// Sets the type of content.
    pub fn set_type_of_content(&mut self, content_type: i32) {
        self.type_of_content = content_type;
    }

    /// Returns the type of content.
    pub fn type_of_content(&self) -> i32 {
        self.type_of_content
    }

    /// Returns the U direction count.
    pub fn u_direction_count(&self) -> i32 {
        self.u_direction_count
    }

    /// Sets the U direction count.
    pub fn set_u_direction_count(&mut self, count: i32) {
        self.u_direction_count = count;
    }

    /// Returns the V direction count.
    pub fn v_direction_count(&self) -> i32 {
        self.v_direction_count
    }

    /// Sets the V direction count.
    pub fn set_v_direction_count(&mut self, count: i32) {
        self.v_direction_count = count;
    }
}

impl Default for DirectionCountSelect {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction_count_select_new() {
        let select = DirectionCountSelect::new();
        assert_eq!(select.type_of_content(), 0);
        assert_eq!(select.u_direction_count(), 0);
        assert_eq!(select.v_direction_count(), 0);
    }

    #[test]
    fn test_set_counts() {
        let mut select = DirectionCountSelect::new();
        select.set_u_direction_count(10);
        select.set_v_direction_count(5);
        assert_eq!(select.u_direction_count(), 10);
        assert_eq!(select.v_direction_count(), 5);
    }

    #[test]
    fn test_set_type_of_content() {
        let mut select = DirectionCountSelect::new();
        select.set_type_of_content(2);
        assert_eq!(select.type_of_content(), 2);
    }
}
