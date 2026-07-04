// FILE: step_visual_marker_select.rs
// occt: StepVisual_MarkerSelect

/// A union type selecting a marker in STEP representation.
#[derive(Clone, Debug, PartialEq)]
pub enum MarkerSelect {
    MarkerMember(i32),
    PreDefinedMarker(i32),
}

impl MarkerSelect {
    /// Creates a MarkerSelect from a marker member.
    pub fn marker_member(id: i32) -> Self {
        MarkerSelect::MarkerMember(id)
    }

    /// Creates a MarkerSelect from a pre-defined marker.
    pub fn pre_defined_marker(id: i32) -> Self {
        MarkerSelect::PreDefinedMarker(id)
    }

    /// Returns the case number.
    pub fn case_num(&self) -> i32 {
        match self {
            MarkerSelect::MarkerMember(_) => 1,
            MarkerSelect::PreDefinedMarker(_) => 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marker_select_marker_member() {
        let sel = MarkerSelect::marker_member(1);
        assert_eq!(sel.case_num(), 1);
    }

    #[test]
    fn test_marker_select_pre_defined_marker() {
        let sel = MarkerSelect::pre_defined_marker(2);
        assert_eq!(sel.case_num(), 2);
    }
}
