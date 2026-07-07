// FILE: step_visual_surface_style_parameter_line.rs
// occt: StepVisual_SurfaceStyleParameterLine

use std::sync::Arc;

pub struct CurveStyle;
pub struct DirectionCountSelect;

pub struct SurfaceStyleParameterLine {
    style_of_parameter_lines: Option<Arc<CurveStyle>>,
    direction_counts: Option<Arc<Vec<DirectionCountSelect>>>,
}

impl SurfaceStyleParameterLine {
    pub fn new() -> Self {
        SurfaceStyleParameterLine {
            style_of_parameter_lines: None,
            direction_counts: None,
        }
    }

    pub fn init(
        &mut self,
        style: Option<Arc<CurveStyle>>,
        direction_counts: Option<Arc<Vec<DirectionCountSelect>>>,
    ) {
        self.style_of_parameter_lines = style;
        self.direction_counts = direction_counts;
    }

    pub fn set_style_of_parameter_lines(&mut self, style: Option<Arc<CurveStyle>>) {
        self.style_of_parameter_lines = style;
    }

    pub fn style_of_parameter_lines(&self) -> Option<&Arc<CurveStyle>> {
        self.style_of_parameter_lines.as_ref()
    }

    pub fn set_direction_counts(&mut self, counts: Option<Arc<Vec<DirectionCountSelect>>>) {
        self.direction_counts = counts;
    }

    pub fn direction_counts(&self) -> Option<&Arc<Vec<DirectionCountSelect>>> {
        self.direction_counts.as_ref()
    }

    pub fn direction_counts_value(&self, num: usize) -> Option<&DirectionCountSelect> {
        self.direction_counts.as_ref().and_then(|counts| counts.get(num))
    }

    pub fn nb_direction_counts(&self) -> usize {
        self.direction_counts.as_ref().map(|c| c.len()).unwrap_or(0)
    }
}

impl Default for SurfaceStyleParameterLine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sspl = SurfaceStyleParameterLine::new();
        assert!(sspl.style_of_parameter_lines().is_none());
        assert_eq!(sspl.nb_direction_counts(), 0);
    }

    #[test]
    fn test_set_style() {
        let mut sspl = SurfaceStyleParameterLine::new();
        let style = Arc::new(CurveStyle);
        sspl.set_style_of_parameter_lines(Some(style));
        assert!(sspl.style_of_parameter_lines().is_some());
    }

    #[test]
    fn test_set_direction_counts() {
        let mut sspl = SurfaceStyleParameterLine::new();
        let counts = vec![DirectionCountSelect];
        sspl.set_direction_counts(Some(Arc::new(counts)));
        assert_eq!(sspl.nb_direction_counts(), 1);
    }

    #[test]
    fn test_init() {
        let mut sspl = SurfaceStyleParameterLine::new();
        let style = Arc::new(CurveStyle);
        let counts = vec![DirectionCountSelect];
        sspl.init(Some(style), Some(Arc::new(counts)));

        assert!(sspl.style_of_parameter_lines().is_some());
        assert_eq!(sspl.nb_direction_counts(), 1);
    }
}
