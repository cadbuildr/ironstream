// FILE: step_visual_invisibility_context.rs
// occt: StepVisual_InvisibilityContext

/// An invisibility context selection in STEP representation.
///
/// This selects the context for invisibility (presentation, model, etc).
#[derive(Clone, Debug, PartialEq)]
pub enum InvisibilityContext {
    PresentationRepresentation(i32),
    PresentationSet(i32),
    DraughtingModel(i32),
}

impl InvisibilityContext {
    /// Creates an InvisibilityContext from a presentation representation.
    pub fn presentation_representation(id: i32) -> Self {
        InvisibilityContext::PresentationRepresentation(id)
    }

    /// Creates an InvisibilityContext from a presentation set.
    pub fn presentation_set(id: i32) -> Self {
        InvisibilityContext::PresentationSet(id)
    }

    /// Creates an InvisibilityContext from a draughting model.
    pub fn draughting_model(id: i32) -> Self {
        InvisibilityContext::DraughtingModel(id)
    }

    /// Returns the case number.
    pub fn case_num(&self) -> i32 {
        match self {
            InvisibilityContext::PresentationRepresentation(_) => 1,
            InvisibilityContext::PresentationSet(_) => 2,
            InvisibilityContext::DraughtingModel(_) => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invisibility_context_presentation_representation() {
        let ctx = InvisibilityContext::presentation_representation(1);
        assert_eq!(ctx.case_num(), 1);
    }

    #[test]
    fn test_invisibility_context_presentation_set() {
        let ctx = InvisibilityContext::presentation_set(2);
        assert_eq!(ctx.case_num(), 2);
    }

    #[test]
    fn test_invisibility_context_draughting_model() {
        let ctx = InvisibilityContext::draughting_model(3);
        assert_eq!(ctx.case_num(), 3);
    }
}
