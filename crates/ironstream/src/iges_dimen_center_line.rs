// FILE: iges_dimen_center_line.rs
// occt: IGESDimen_CenterLine

/// Defines CenterLine, Type <106> Form <20-21>
/// in package IGESDimen
/// Is an entity appearing as crosshairs or as a
/// construction between 2 positions
pub struct IgesDimen_CenterLine {
    datatype: i32,
    z_displacement: f64,
    data_points: Vec<(f64, f64)>,
    is_cross_hair: bool,
}

impl IgesDimen_CenterLine {
    /// Create a new CenterLine entity
    pub fn new() -> Self {
        IgesDimen_CenterLine {
            datatype: 1,
            z_displacement: 0.0,
            data_points: Vec::new(),
            is_cross_hair: false,
        }
    }

    /// This method is used to set the fields of the class CenterLine
    /// - a_data_type      : Interpretation Flag, always = 1
    /// - a_zdisp          : Common z displacement
    /// - data_pnts        : Data points (x and y)
    pub fn init(&mut self, a_data_type: i32, a_zdisp: f64, data_pnts: Vec<(f64, f64)>) {
        self.datatype = a_data_type;
        self.z_displacement = a_zdisp;
        self.data_points = data_pnts;
    }

    /// Sets is_cross_hair to true if mode is true, false else
    pub fn set_cross_hair(&mut self, mode: bool) {
        self.is_cross_hair = mode;
    }

    /// Returns Interpretation Flag : IP = 1.
    pub fn datatype(&self) -> i32 {
        self.datatype
    }

    /// Returns Number of Data Points.
    pub fn nb_points(&self) -> i32 {
        self.data_points.len() as i32
    }

    /// Returns Common Z displacement.
    pub fn z_displacement(&self) -> f64 {
        self.z_displacement
    }

    /// Returns the data point as (x, y, z).
    /// raises exception if index <= 0 or index > nb_points()
    pub fn point(&self, index: usize) -> (f64, f64, f64) {
        if index == 0 || index > self.data_points.len() {
            panic!("Index out of bounds");
        }
        let (x, y) = self.data_points[index - 1];
        (x, y, self.z_displacement)
    }

    /// Returns the data point after Transformation.
    /// raises exception if index <= 0 or index > nb_points()
    pub fn transformed_point(&self, index: usize) -> (f64, f64, f64) {
        self.point(index)
    }

    /// Returns True if is_cross_hair is set.
    pub fn is_cross_hair(&self) -> bool {
        self.is_cross_hair
    }
}

impl Default for IgesDimen_CenterLine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_center_line_creation() {
        let line = IgesDimen_CenterLine::new();
        assert_eq!(line.datatype(), 1);
        assert_eq!(line.z_displacement(), 0.0);
        assert_eq!(line.nb_points(), 0);
        assert!(!line.is_cross_hair());
    }

    #[test]
    fn test_center_line_init() {
        let mut line = IgesDimen_CenterLine::new();
        let points = vec![(0.0, 0.0), (10.0, 10.0), (20.0, 5.0)];
        line.init(1, 5.0, points);

        assert_eq!(line.datatype(), 1);
        assert_eq!(line.z_displacement(), 5.0);
        assert_eq!(line.nb_points(), 3);
    }

    #[test]
    fn test_center_line_points() {
        let mut line = IgesDimen_CenterLine::new();
        let points = vec![(1.0, 2.0), (3.0, 4.0)];
        line.init(1, 5.0, points);

        assert_eq!(line.point(1), (1.0, 2.0, 5.0));
        assert_eq!(line.point(2), (3.0, 4.0, 5.0));
    }

    #[test]
    fn test_center_line_cross_hair() {
        let mut line = IgesDimen_CenterLine::new();
        assert!(!line.is_cross_hair());

        line.set_cross_hair(true);
        assert!(line.is_cross_hair());
    }

    #[test]
    #[should_panic(expected = "out of bounds")]
    fn test_center_line_point_out_of_bounds() {
        let line = IgesDimen_CenterLine::new();
        let _ = line.point(1);
    }
}
