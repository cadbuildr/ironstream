// FILE: step_visual_draughting_annotation_occurrence.rs
// occt: StepVisual_DraughtingAnnotationOccurrence

/// A draughting annotation occurrence in STEP representation.
///
/// This represents an occurrence of a draughting annotation in a design.
pub struct DraughtingAnnotationOccurrence {
    name: String,
    annotation_type: String,
}

impl DraughtingAnnotationOccurrence {
    /// Creates a new draughting annotation occurrence.
    pub fn new(name: String) -> Self {
        DraughtingAnnotationOccurrence {
            name,
            annotation_type: String::new(),
        }
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the annotation type.
    pub fn set_annotation_type(&mut self, ann_type: String) {
        self.annotation_type = ann_type;
    }

    /// Returns the annotation type.
    pub fn annotation_type(&self) -> &str {
        &self.annotation_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draughting_annotation_occurrence_new() {
        let occ = DraughtingAnnotationOccurrence::new("Annotation1".to_string());
        assert_eq!(occ.name(), "Annotation1");
        assert_eq!(occ.annotation_type(), "");
    }

    #[test]
    fn test_set_annotation_type() {
        let mut occ = DraughtingAnnotationOccurrence::new("Ann".to_string());
        occ.set_annotation_type("Dimension".to_string());
        assert_eq!(occ.annotation_type(), "Dimension");
    }
}
