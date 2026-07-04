// FILE: step_visual_marker_member.rs
// occt: StepVisual_MarkerMember

/// A marker member in STEP representation.
///
/// This represents a member of a marker style definition.
pub struct MarkerMember {
    member_id: i32,
    marker_type: i32,
}

impl MarkerMember {
    /// Creates a new marker member.
    pub fn new(id: i32) -> Self {
        MarkerMember {
            member_id: id,
            marker_type: 0,
        }
    }

    /// Returns the member ID.
    pub fn member_id(&self) -> i32 {
        self.member_id
    }

    /// Sets the marker type.
    pub fn set_marker_type(&mut self, marker_type: i32) {
        self.marker_type = marker_type;
    }

    /// Returns the marker type.
    pub fn marker_type(&self) -> i32 {
        self.marker_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marker_member_new() {
        let member = MarkerMember::new(5);
        assert_eq!(member.member_id(), 5);
        assert_eq!(member.marker_type(), 0);
    }

    #[test]
    fn test_set_marker_type() {
        let mut member = MarkerMember::new(1);
        member.set_marker_type(3);
        assert_eq!(member.marker_type(), 3);
    }
}
