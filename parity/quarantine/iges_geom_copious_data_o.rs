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
        self.form_number = if data_type == 1 { 1 } else { 2 };
    }

    pub fn set_polyline(&mut self, mode: bool) {
        if mode {
            self.form_number = match self.data_type {
                1 => 11,
                2 => 12,
                3 => 13,
                _ => 11,
            };
        } else {
            self.form_number = match self.data_type {
                1 => 1,
                2 => 2,
                3 => 3,
                _ => 1,
            };
        }
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

    pub fn nb_points(&self) -> usize {
        match self.data_type {
            1 => self.data.len() / 2,
            2 | 3 => self.data.len() / 3,
            _ => 0,
        }
    }

    pub fn data(&self, num_point: usize, num_data: usize) -> Option<f64> {
        let base_index = match self.data_type {
            1 => (num_point - 1) * 2,
            2 | 3 => (num_point - 1) * self.data_type as usize,
            _ => return None,
        };
        if base_index + num_data - 1 < self.data.len() {
            Some(self.data[base_index + num_data - 1])
        } else {
            None
        }
    }

    pub fn z_plane_value(&self) -> f64 {
        if self.data_type == 1 {
            self.z_plane
        } else {
            0.0
        }
    }

    pub fn point(&self, index: usize) -> Option<(f64, f64, f64)> {
        match self.data_type {
            1 => {
                self.data((index, 1)).and_then(|x| {
                    self.data((index, 2)).map(|y| (x, y, self.z_plane))
                })
            }
            2 | 3 => {
                self.data((index, 1)).and_then(|x| {
                    self.data((index, 2)).and_then(|y| {
                        self.data((index, 3)).map(|z| (x, y, z))
                    })
                })
            }
            _ => None,
        }
    }

    pub fn vector(&self, index: usize) -> Option<(f64, f64, f64)> {
        if self.data_type == 3 {
            self.data((index, 4)).and_then(|x| {
                self.data((index, 5)).and_then(|y| {
                    self.data((index, 6)).map(|z| (x, y, z))
                })
            })
        } else {
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
    }
}

impl IgesGeomCopiousData {
    fn data(&self, _indices: (usize, usize)) -> Option<f64> {
        // Placeholder implementation
        None
    }
}
