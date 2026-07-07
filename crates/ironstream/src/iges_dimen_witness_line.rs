// FILE: iges_dimen_witness_line.rs
// occt: IGESDimen_WitnessLine

/// Defines WitnessLine, Type <106> Form <40>
/// in package IGESDimen
/// Contains one or more straight line segments associated
/// with drafting entities of various types
pub struct IgesDimenWitnessLine {
    data_type: i32,
    z_displacement: f64,
    data_points: Vec<(f64, f64)>, // xy points
}

impl IgesDimenWitnessLine {
    /// Create a new WitnessLine
    pub fn new() -> Self {
        IgesDimenWitnessLine {
            data_type: 1,
            z_displacement: 0.0,
            data_points: Vec::new(),
        }
    }

    /// This method is used to set the fields of the class WitnessLine
    /// - data_type   : Interpretation Flag, always = 1
    /// - z_disp      : Common z displacement
    /// - data_points : Data points
    pub fn init(&mut self, data_type: i32, z_disp: f64, data_points: Vec<(f64, f64)>) {
        self.data_type = data_type;
        self.z_displacement = z_disp;
        self.data_points = data_points;
    }

    /// Returns Interpretation Flag, always = 1
    pub fn datatype(&self) -> i32 {
        self.data_type
    }

    /// Returns number of Data Points
    pub fn nb_points(&self) -> i32 {
        self.data_points.len() as i32
    }

    /// Returns common Z displacement
    pub fn z_displacement(&self) -> f64 {
        self.z_displacement
    }

    /// Returns Index'th. data point (0-indexed)
    /// Panics if Index < 0 or Index >= nb_points
    pub fn point(&self, index: i32) -> (f64, f64) {
        if index < 0 || index >= self.nb_points() {
            panic!("Index out of bounds");
        }
        self.data_points[index as usize]
    }

    /// Returns data point after Transformation
    /// Panics if Index < 0 or Index >= nb_points
    pub fn transformed_point(&self, index: i32) -> (f64, f64, f64) {
        if index < 0 || index >= self.nb_points() {
            panic!("Index out of bounds");
        }
        let (x, y) = self.data_points[index as usize];
        (x, y, self.z_displacement)
    }
}

impl Default for IgesDimenWitnessLine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let line = IgesDimenWitnessLine::new();
        assert_eq!(line.datatype(), 1);
        assert_eq!(line.z_displacement(), 0.0);
        assert_eq!(line.nb_points(), 0);
    }

    #[test]
    fn test_init() {
        let mut line = IgesDimenWitnessLine::new();
        let points = vec![(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)];
        line.init(1, 5.5, points);

        assert_eq!(line.datatype(), 1);
        assert_eq!(line.z_displacement(), 5.5);
        assert_eq!(line.nb_points(), 3);
    }

    #[test]
    fn test_point() {
        let mut line = IgesDimenWitnessLine::new();
        let points = vec![(1.0, 2.0), (3.0, 4.0)];
        line.init(1, 0.0, points);

        let p0 = line.point(0);
        assert_eq!(p0, (1.0, 2.0));

        let p1 = line.point(1);
        assert_eq!(p1, (3.0, 4.0));
    }

    #[test]
    fn test_transformed_point() {
        let mut line = IgesDimenWitnessLine::new();
        let points = vec![(1.0, 2.0), (3.0, 4.0)];
        line.init(1, 7.5, points);

        let p0 = line.transformed_point(0);
        assert_eq!(p0, (1.0, 2.0, 7.5));

        let p1 = line.transformed_point(1);
        assert_eq!(p1, (3.0, 4.0, 7.5));
    }

    #[test]
    #[should_panic]
    fn test_point_out_of_bounds() {
        let line = IgesDimenWitnessLine::new();
        let _ = line.point(0);
    }
}
