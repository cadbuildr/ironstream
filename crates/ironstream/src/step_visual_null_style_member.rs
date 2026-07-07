// FILE: step_visual_null_style_member.rs
// occt: StepVisual_NullStyleMember

/// A null style member in STEP representation.
///
/// This represents a member that has no style.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NullStyleMember {
    member_id: i32,
}

impl NullStyleMember {
    /// Creates a new null style member.
    pub fn new(id: i32) -> Self {
        NullStyleMember { member_id: id }
    }

    /// Returns the member ID.
    pub fn member_id(&self) -> i32 {
        self.member_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null_style_member_new() {
        let member = NullStyleMember::new(5);
        assert_eq!(member.member_id(), 5);
    }

    #[test]
    fn test_null_style_member_equality() {
        let m1 = NullStyleMember::new(3);
        let m2 = NullStyleMember::new(3);
        assert_eq!(m1, m2);
    }
}
