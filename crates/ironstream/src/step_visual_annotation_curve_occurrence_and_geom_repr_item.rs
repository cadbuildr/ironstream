// FILE: step_visual_annotation_curve_occurrence_and_geom_repr_item.rs
// occt: StepVisual_AnnotationCurveOccurrenceAndGeomReprItem

/// Represents combined AnnotationCurveOccurrence and GeomReprItem
#[derive(Debug, Clone, Default)]
pub struct StepVisual_AnnotationCurveOccurrenceAndGeomReprItem {
    name: Option<String>,
}

impl StepVisual_AnnotationCurveOccurrenceAndGeomReprItem {
    pub fn new() -> Self {
        StepVisual_AnnotationCurveOccurrenceAndGeomReprItem { name: None }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let acog = StepVisual_AnnotationCurveOccurrenceAndGeomReprItem::new();
        assert!(acog.name().is_none());
    }

    #[test]
    fn test_set_name() {
        let mut acog = StepVisual_AnnotationCurveOccurrenceAndGeomReprItem::new();
        acog.set_name("test".to_string());
        assert_eq!(acog.name(), Some("test"));
    }
}
