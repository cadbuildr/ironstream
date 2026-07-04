// FILE: iges_geom_line.rs
// occt: IGESGeom_Line

/// Defines IGESLine, Type <110> Form <0> in package IGESGeom.
/// A line is a bounded, connected portion of a parent straight line
/// which consists of more than one point. A line is defined by its endpoints.
///
/// From IGES-5.3, two other Forms are admitted (same params):
/// 0 remains for standard limited line (the default)
/// 1 for semi-infinite line (End is just a passing point)
/// 2 for full infinite Line (both Start and End are arbitrary)
#[derive(Clone, Debug)]
pub struct Line {
    /// Start point of the line (x, y, z)
    start: [f64; 3],
    /// End point of the line (x, y, z)
    end: [f64; 3],
    /// Form number: 0 (limited), 1 (semi-infinite), 2 (infinite)
    form: i32,
    /// Type number for IGES (always 110)
    entity_type: i32,
}

impl Line {
    /// Creates a new Line entity.
    pub fn new() -> Self {
        Line {
            start: [0.0, 0.0, 0.0],
            end: [0.0, 0.0, 0.0],
            form: 0,
            entity_type: 110,
        }
    }

    /// Initializes the line with start and end points.
    pub fn init(&mut self, start: [f64; 3], end: [f64; 3]) {
        self.start = start;
        self.end = end;
        self.form = 0;
        self.entity_type = 110;
    }

    /// Returns the infinite status (form number): 0, 1, or 2.
    pub fn infinite(&self) -> i32 {
        self.form
    }

    /// Sets the infinite status (form number).
    /// Does nothing if status is not 0, 1, or 2.
    pub fn set_infinite(&mut self, status: i32) {
        if status >= 0 && status <= 2 {
            self.form = status;
        }
    }

    /// Returns the start point of the line.
    pub fn start_point(&self) -> [f64; 3] {
        self.start
    }

    /// Returns the start point after applying a transformation matrix.
    /// In the context of IGES, this would apply the entity's transformation.
    /// For this port, we return the start point directly.
    pub fn transformed_start_point(&self) -> [f64; 3] {
        // TODO: Apply transformation matrix if present
        self.start
    }

    /// Returns the end point of the line.
    pub fn end_point(&self) -> [f64; 3] {
        self.end
    }

    /// Returns the end point after applying a transformation matrix.
    /// In the context of IGES, this would apply the entity's transformation.
    /// For this port, we return the end point directly.
    pub fn transformed_end_point(&self) -> [f64; 3] {
        // TODO: Apply transformation matrix if present
        self.end
    }

    /// Returns the entity type number (always 110 for Line).
    pub fn entity_type(&self) -> i32 {
        self.entity_type
    }

    /// Returns the form number (0, 1, or 2).
    pub fn form_number(&self) -> i32 {
        self.form
    }
}

impl Default for Line {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_line() {
        let line = Line::new();
        assert_eq!(line.start_point(), [0.0, 0.0, 0.0]);
        assert_eq!(line.end_point(), [0.0, 0.0, 0.0]);
        assert_eq!(line.infinite(), 0);
        assert_eq!(line.entity_type(), 110);
    }

    #[test]
    fn test_init_line() {
        let mut line = Line::new();
        line.init([1.0, 2.0, 3.0], [4.0, 5.0, 6.0]);
        assert_eq!(line.start_point(), [1.0, 2.0, 3.0]);
        assert_eq!(line.end_point(), [4.0, 5.0, 6.0]);
        assert_eq!(line.infinite(), 0);
    }

    #[test]
    fn test_infinite_status() {
        let mut line = Line::new();
        line.init([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);

        // Default is limited (0)
        assert_eq!(line.infinite(), 0);

        // Set to semi-infinite (1)
        line.set_infinite(1);
        assert_eq!(line.infinite(), 1);

        // Set to fully infinite (2)
        line.set_infinite(2);
        assert_eq!(line.infinite(), 2);

        // Invalid value should be ignored
        line.set_infinite(3);
        assert_eq!(line.infinite(), 2);

        // Set back to limited (0)
        line.set_infinite(0);
        assert_eq!(line.infinite(), 0);
    }

    #[test]
    fn test_transformed_points() {
        let mut line = Line::new();
        line.init([1.0, 2.0, 3.0], [4.0, 5.0, 6.0]);

        // Without transformation, transformed points equal original points
        assert_eq!(line.transformed_start_point(), [1.0, 2.0, 3.0]);
        assert_eq!(line.transformed_end_point(), [4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_form_and_type_numbers() {
        let line = Line::new();
        assert_eq!(line.form_number(), 0);
        assert_eq!(line.entity_type(), 110);
    }
}
