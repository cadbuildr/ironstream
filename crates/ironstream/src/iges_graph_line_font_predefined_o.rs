// FILE: iges_graph_line_font_predefined_o.rs
// occt: IGESGraph_LineFontPredefined

/// Represents an IGES Line Font Predefined entity (Type 406, Form 19).
/// Provides the ability to specify a line font pattern from a predefined list
/// rather than from Directory Entry Field 4.
pub struct IgesGraphLineFontPredefined {
    nb_property_values: i32,
    line_font_pattern_code: i32,
}

impl IgesGraphLineFontPredefined {
    /// Creates a new empty LineFontPredefined entity.
    pub fn new() -> Self {
        IgesGraphLineFontPredefined {
            nb_property_values: 0,
            line_font_pattern_code: 0,
        }
    }

    /// Sets the fields of the LineFontPredefined entity.
    ///
    /// # Arguments
    /// - `nb_props`: Number of property values (should be 1)
    /// - `code`: Line font pattern code
    pub fn init(&mut self, nb_props: i32, code: i32) {
        self.nb_property_values = nb_props;
        self.line_font_pattern_code = code;
    }

    /// Returns the number of property values.
    pub fn nb_property_values(&self) -> i32 {
        self.nb_property_values
    }

    /// Returns the line font pattern code.
    pub fn line_font_pattern_code(&self) -> i32 {
        self.line_font_pattern_code
    }

    /// Returns a description of the predefined line font pattern.
    pub fn pattern_description(&self) -> &'static str {
        match self.line_font_pattern_code {
            1 => "Solid",
            2 => "Dashed",
            3 => "Dotted",
            4 => "Dash-Dot",
            5 => "Dash-Dot-Dot",
            _ => "Unknown",
        }
    }
}

impl Default for IgesGraphLineFontPredefined {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_font_predefined_creation() {
        let lfp = IgesGraphLineFontPredefined::new();
        assert_eq!(lfp.nb_property_values(), 0);
        assert_eq!(lfp.line_font_pattern_code(), 0);
    }

    #[test]
    fn test_line_font_predefined_init() {
        let mut lfp = IgesGraphLineFontPredefined::new();
        lfp.init(1, 2);
        assert_eq!(lfp.nb_property_values(), 1);
        assert_eq!(lfp.line_font_pattern_code(), 2);
    }

    #[test]
    fn test_line_font_predefined_solid() {
        let mut lfp = IgesGraphLineFontPredefined::new();
        lfp.init(1, 1);
        assert_eq!(lfp.pattern_description(), "Solid");
    }

    #[test]
    fn test_line_font_predefined_dashed() {
        let mut lfp = IgesGraphLineFontPredefined::new();
        lfp.init(1, 2);
        assert_eq!(lfp.pattern_description(), "Dashed");
    }

    #[test]
    fn test_line_font_predefined_dotted() {
        let mut lfp = IgesGraphLineFontPredefined::new();
        lfp.init(1, 3);
        assert_eq!(lfp.pattern_description(), "Dotted");
    }

    #[test]
    fn test_line_font_predefined_dash_dot() {
        let mut lfp = IgesGraphLineFontPredefined::new();
        lfp.init(1, 4);
        assert_eq!(lfp.pattern_description(), "Dash-Dot");
    }

    #[test]
    fn test_line_font_predefined_dash_dot_dot() {
        let mut lfp = IgesGraphLineFontPredefined::new();
        lfp.init(1, 5);
        assert_eq!(lfp.pattern_description(), "Dash-Dot-Dot");
    }

    #[test]
    fn test_line_font_predefined_unknown() {
        let mut lfp = IgesGraphLineFontPredefined::new();
        lfp.init(1, 99);
        assert_eq!(lfp.pattern_description(), "Unknown");
    }
}
