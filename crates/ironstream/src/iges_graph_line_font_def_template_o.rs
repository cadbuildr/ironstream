// FILE: iges_graph_line_font_def_template_o.rs
// occt: IGESGraph_LineFontDefTemplate

/// Represents an IGES Line Font Definition Template entity (Type 304, Form 1).
/// Line Font is defined as a repetition of a template figure displayed at
/// regularly spaced locations along a planar anchoring curve.
pub struct IgesGraphLineFontDefTemplate {
    orientation: i32,
    template_entity: Option<Box<dyn std::any::Any>>,
    distance: f64,
    scale: f64,
}

impl IgesGraphLineFontDefTemplate {
    /// Creates a new empty LineFontDefTemplate entity.
    pub fn new() -> Self {
        IgesGraphLineFontDefTemplate {
            orientation: 0,
            template_entity: None,
            distance: 0.0,
            scale: 1.0,
        }
    }

    /// Sets the fields of the LineFontDefTemplate entity.
    ///
    /// # Arguments
    /// - `orientation`: Orientation of template figure on anchoring curve (0 or 1)
    /// - `template`: SubfigureDef entity used as template figure
    /// - `distance`: Distance between neighbouring template figures
    /// - `scale`: Scale factor applied to the template figure
    pub fn init(
        &mut self,
        orientation: i32,
        template: Option<Box<dyn std::any::Any>>,
        distance: f64,
        scale: f64,
    ) {
        self.orientation = orientation;
        self.template_entity = template;
        self.distance = distance;
        self.scale = scale;
    }

    /// Returns the orientation value.
    ///
    /// - 0: Each template display is oriented by aligning the axis of the SubfigureDef
    ///      with the axis of the definition space of the anchoring curve.
    /// - 1: Each template display is oriented by aligning the X-axis of the SubfigureDef
    ///      with the tangent vector of the anchoring curve at the point of incidence.
    ///      Similarly, Z-axis is aligned.
    pub fn orientation(&self) -> i32 {
        self.orientation
    }

    /// Returns the SubfigureDef entity used as the template figure.
    pub fn template_entity(&self) -> Option<&dyn std::any::Any> {
        self.template_entity.as_ref().map(|b| b.as_ref())
    }

    /// Returns the distance between any two template figures on the anchoring curve.
    pub fn distance(&self) -> f64 {
        self.distance
    }

    /// Returns the scaling factor applied to the SubfigureDef.
    pub fn scale(&self) -> f64 {
        self.scale
    }
}

impl Default for IgesGraphLineFontDefTemplate {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_font_def_template_creation() {
        let lfdt = IgesGraphLineFontDefTemplate::new();
        assert_eq!(lfdt.orientation(), 0);
        assert!(lfdt.template_entity().is_none());
        assert_eq!(lfdt.distance(), 0.0);
        assert_eq!(lfdt.scale(), 1.0);
    }

    #[test]
    fn test_line_font_def_template_init() {
        let mut lfdt = IgesGraphLineFontDefTemplate::new();
        lfdt.init(1, None, 5.0, 2.5);
        assert_eq!(lfdt.orientation(), 1);
        assert!(lfdt.template_entity().is_none());
        assert_eq!(lfdt.distance(), 5.0);
        assert_eq!(lfdt.scale(), 2.5);
    }

    #[test]
    fn test_line_font_def_template_orientation_0() {
        let mut lfdt = IgesGraphLineFontDefTemplate::new();
        lfdt.init(0, None, 0.0, 1.0);
        assert_eq!(lfdt.orientation(), 0);
    }

    #[test]
    fn test_line_font_def_template_orientation_1() {
        let mut lfdt = IgesGraphLineFontDefTemplate::new();
        lfdt.init(1, None, 0.0, 1.0);
        assert_eq!(lfdt.orientation(), 1);
    }

    #[test]
    fn test_line_font_def_template_distance_and_scale() {
        let mut lfdt = IgesGraphLineFontDefTemplate::new();
        lfdt.init(0, None, 10.5, 0.5);
        assert_eq!(lfdt.distance(), 10.5);
        assert_eq!(lfdt.scale(), 0.5);
    }
}
