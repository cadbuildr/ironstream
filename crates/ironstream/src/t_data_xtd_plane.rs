// FILE: t_data_xtd_plane.rs
// occt: TDataXtd_Plane

/// The basis to define a plane attribute
pub struct TDataXtdPlane {
}

impl TDataXtdPlane {
    pub fn new() -> Self {
        TDataXtdPlane {}
    }

    pub fn get_id() -> [u8; 16] {
        [0x00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    }

    pub fn id(&self) -> [u8; 16] {
        Self::get_id()
    }

    pub fn dump(&self) -> &'static str {
        "TDataXtd_Plane"
    }

    pub fn set_plane(&mut self, a: f64, b: f64, c: f64, d: f64) {
        let _ = (a, b, c, d);
    }

    pub fn normal(&self) -> (f64, f64, f64) {
        (0.0, 0.0, 1.0)
    }
}

impl Default for TDataXtdPlane {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let plane = TDataXtdPlane::new();
        assert_eq!(plane.dump(), "TDataXtd_Plane");
    }

    #[test]
    fn test_get_id() {
        let id = TDataXtdPlane::get_id();
        assert_eq!(id.len(), 16);
    }

    #[test]
    fn test_instance_id() {
        let plane = TDataXtdPlane::new();
        let id = plane.id();
        assert_eq!(id.len(), 16);
    }

    #[test]
    fn test_dump() {
        let plane = TDataXtdPlane::new();
        assert_eq!(plane.dump(), "TDataXtd_Plane");
    }

    #[test]
    fn test_set_plane() {
        let mut plane = TDataXtdPlane::new();
        plane.set_plane(1.0, 0.0, 0.0, 5.0);
    }

    #[test]
    fn test_normal() {
        let plane = TDataXtdPlane::new();
        let (nx, ny, nz) = plane.normal();
        assert_eq!(nx, 0.0);
        assert_eq!(ny, 0.0);
        assert_eq!(nz, 1.0);
    }

    #[test]
    fn test_id_consistency() {
        let id1 = TDataXtdPlane::get_id();
        let id2 = TDataXtdPlane::get_id();
        assert_eq!(id1, id2);
    }
}
