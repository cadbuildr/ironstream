// FILE: iges_geom_copious_data_o.rs
// occt: IGESGeom_CopiousData

pub struct IgesGeomCopiousData {
    data_type: i32,
    z_plane: f64,
    data: Vec<f64>,
    form_number: i32,
}

impl IgesGeomCopiousData {
    pub fn new() -> Self {
        IgesGeomCopiousData {
            data_type: 1,
            z_plane: 0.0,
            data: Vec::new(),
            form_number: 1,
        }
    }

    pub fn init(&mut self, data_type: i32, z_plane: f64, data: Vec<f64>) {
        self.data_type = data_type;
        self.z_plane = z_plane;
        self.data = data;
        // FormNumber = DataType + <N>; N=0 -> Set of Points (as in OCCT Init).
        self.form_number = data_type;
    }

    pub fn set_polyline(&mut self, mode: bool) {
        // OCCT SetPolyline: newfn = theDataType (+10 when polyline).
        self.form_number = if mode {
            self.data_type + 10
        } else {
            self.data_type
        };
    }

    pub fn set_closed_path2d(&mut self) {
        self.form_number = 63;
    }

    pub fn is_point_set(&self) -> bool {
        self.form_number <= 3 && self.form_number >= 1
    }

    pub fn is_polyline(&self) -> bool {
        self.form_number >= 11 && self.form_number <= 13
    }

    pub fn is_closed_path2d(&self) -> bool {
        self.form_number == 63
    }

    pub fn data_type(&self) -> i32 {
        self.data_type
    }

    /// Number of values stored per point tuple for the current data type
    /// (OCCT: 1 -> 2 values (x,y), 2 -> 3 values (x,y,z), 3 -> 6 values
    /// (x,y,z,i,j,k)).
    fn tuple_size(&self) -> Option<usize> {
        match self.data_type {
            1 => Some(2),
            2 => Some(3),
            3 => Some(6),
            _ => None,
        }
    }

    pub fn nb_points(&self) -> usize {
        match self.tuple_size() {
            Some(n) => self.data.len() / n,
            None => 0,
        }
    }

    pub fn data(&self, num_point: usize, num_data: usize) -> Option<f64> {
        if num_point == 0 || num_data == 0 {
            return None;
        }
        let tuple = self.tuple_size()?;
        if num_data > tuple {
            return None;
        }
        let index = (num_point - 1) * tuple + num_data - 1;
        self.data.get(index).copied()
    }

    pub fn z_plane_value(&self) -> f64 {
        self.z_plane
    }

    pub fn point(&self, index: usize) -> Option<(f64, f64, f64)> {
        match self.data_type {
            1 => self
                .data(index, 1)
                .and_then(|x| self.data(index, 2).map(|y| (x, y, self.z_plane))),
            2 | 3 => self.data(index, 1).and_then(|x| {
                self.data(index, 2)
                    .and_then(|y| self.data(index, 3).map(|z| (x, y, z)))
            }),
            _ => None,
        }
    }

    pub fn vector(&self, index: usize) -> Option<(f64, f64, f64)> {
        if self.data_type == 3 {
            self.data(index, 4).and_then(|x| {
                self.data(index, 5)
                    .and_then(|y| self.data(index, 6).map(|z| (x, y, z)))
            })
        } else {
            // OCCT returns a null vector when data type is not 3.
            Some((0.0, 0.0, 0.0))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copious_data_creation() {
        let cd = IgesGeomCopiousData::new();
        assert_eq!(cd.data_type(), 1);
        assert_eq!(cd.z_plane_value(), 0.0);
        assert!(cd.is_point_set());
    }

    #[test]
    fn test_polyline_flag() {
        let mut cd = IgesGeomCopiousData::new();
        cd.init(1, 0.0, vec![]);
        assert!(cd.is_point_set());

        cd.set_polyline(true);
        assert!(cd.is_polyline());
        assert!(!cd.is_point_set());
    }

    #[test]
    fn test_closed_path_2d() {
        let mut cd = IgesGeomCopiousData::new();
        cd.init(1, 0.0, vec![]);
        cd.set_closed_path2d();
        assert!(cd.is_closed_path2d());
    }

    #[test]
    fn test_nb_points() {
        let mut cd = IgesGeomCopiousData::new();
        cd.init(1, 0.0, vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(cd.nb_points(), 2);

        // Type 2: 3 values (x, y, z) per point.
        cd.init(2, 0.0, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        assert_eq!(cd.nb_points(), 2);

        // Type 3: 6 values (x, y, z, i, j, k) per point.
        cd.init(3, 0.0, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        assert_eq!(cd.nb_points(), 1);
    }

    #[test]
    fn test_data_indexing_type1() {
        let mut cd = IgesGeomCopiousData::new();
        cd.init(1, 5.0, vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(cd.data(1, 1), Some(1.0));
        assert_eq!(cd.data(1, 2), Some(2.0));
        assert_eq!(cd.data(2, 1), Some(3.0));
        assert_eq!(cd.data(2, 2), Some(4.0));
        assert_eq!(cd.data(3, 1), None); // out of range
        assert_eq!(cd.data(1, 3), None); // beyond tuple size
    }

    #[test]
    fn test_point_type1_uses_zplane() {
        let mut cd = IgesGeomCopiousData::new();
        cd.init(1, 7.5, vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(cd.point(1), Some((1.0, 2.0, 7.5)));
        assert_eq!(cd.point(2), Some((3.0, 4.0, 7.5)));
        assert_eq!(cd.z_plane_value(), 7.5);
    }

    #[test]
    fn test_point_type2() {
        let mut cd = IgesGeomCopiousData::new();
        cd.init(2, 0.0, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        assert_eq!(cd.point(1), Some((1.0, 2.0, 3.0)));
        assert_eq!(cd.point(2), Some((4.0, 5.0, 6.0)));
    }

    #[test]
    fn test_point_and_vector_type3() {
        let mut cd = IgesGeomCopiousData::new();
        cd.init(
            3,
            0.0,
            vec![
                1.0, 2.0, 3.0, 0.1, 0.2, 0.3, // point 1: xyz + ijk
                4.0, 5.0, 6.0, 0.4, 0.5, 0.6, // point 2: xyz + ijk
            ],
        );
        assert_eq!(cd.nb_points(), 2);
        assert_eq!(cd.point(1), Some((1.0, 2.0, 3.0)));
        assert_eq!(cd.point(2), Some((4.0, 5.0, 6.0)));
        assert_eq!(cd.vector(1), Some((0.1, 0.2, 0.3)));
        assert_eq!(cd.vector(2), Some((0.4, 0.5, 0.6)));
    }

    #[test]
    fn test_vector_non_type3_is_null_vector() {
        let mut cd = IgesGeomCopiousData::new();
        cd.init(2, 0.0, vec![1.0, 2.0, 3.0]);
        assert_eq!(cd.vector(1), Some((0.0, 0.0, 0.0)));
    }

    #[test]
    fn test_polyline_form_numbers_by_type() {
        let mut cd = IgesGeomCopiousData::new();
        cd.init(3, 0.0, vec![]);
        assert!(cd.is_point_set());
        cd.set_polyline(true);
        assert!(cd.is_polyline());
        cd.set_polyline(false);
        assert!(cd.is_point_set());
    }
}

