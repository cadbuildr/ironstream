// FILE: bopds_point.rs
// occt: BOPDS_Point

/// Minimal 3D point stub
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpPnt {
    x: f64,
    y: f64,
    z: f64,
}

impl GpPnt {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        GpPnt { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }
}

/// Minimal 2D point stub
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GpPnt2d {
    x: f64,
    y: f64,
}

impl GpPnt2d {
    pub fn new(x: f64, y: f64) -> Self {
        GpPnt2d { x, y }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }
}

/// The class BOPDS_Point is to store the information about intersection point
#[derive(Clone, Debug)]
pub struct BodsPoint {
    my_pnt: GpPnt,
    my_pnt2d1: GpPnt2d,
    my_pnt2d2: GpPnt2d,
    my_index: i32,
}

impl BodsPoint {
    /// Empty constructor
    pub fn new() -> Self {
        BodsPoint {
            my_pnt: GpPnt::new(0.0, 0.0, 0.0),
            my_pnt2d1: GpPnt2d::new(0.0, 0.0),
            my_pnt2d2: GpPnt2d::new(0.0, 0.0),
            my_index: 0,
        }
    }

    /// Modifier: Sets 3D point
    pub fn set_pnt(&mut self, the_pnt: &GpPnt) {
        self.my_pnt = *the_pnt;
    }

    /// Selector: Returns 3D point
    pub fn pnt(&self) -> GpPnt {
        self.my_pnt
    }

    /// Modifier: Sets 2D point on the first face
    pub fn set_pnt2d1(&mut self, the_pnt: &GpPnt2d) {
        self.my_pnt2d1 = *the_pnt;
    }

    /// Selector: Returns 2D point on the first face
    pub fn pnt2d1(&self) -> GpPnt2d {
        self.my_pnt2d1
    }

    /// Modifier: Sets 2D point on the second face
    pub fn set_pnt2d2(&mut self, the_pnt: &GpPnt2d) {
        self.my_pnt2d2 = *the_pnt;
    }

    /// Selector: Returns 2D point on the second face
    pub fn pnt2d2(&self) -> GpPnt2d {
        self.my_pnt2d2
    }

    /// Modifier: Sets the index of the vertex
    pub fn set_index(&mut self, the_index: i32) {
        self.my_index = the_index;
    }

    /// Selector: Returns index of the vertex
    pub fn index(&self) -> i32 {
        self.my_index
    }
}

impl Default for BodsPoint {
    fn default() -> Self {
        BodsPoint::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gp_pnt() {
        let pnt = GpPnt::new(1.0, 2.0, 3.0);
        assert_eq!(pnt.x(), 1.0);
        assert_eq!(pnt.y(), 2.0);
        assert_eq!(pnt.z(), 3.0);
    }

    #[test]
    fn test_gp_pnt2d() {
        let pnt = GpPnt2d::new(0.5, 0.6);
        assert_eq!(pnt.x(), 0.5);
        assert_eq!(pnt.y(), 0.6);
    }

    #[test]
    fn test_bods_point_new() {
        let point = BodsPoint::new();
        let pnt = point.pnt();
        assert_eq!(pnt.x(), 0.0);
        assert_eq!(pnt.y(), 0.0);
        assert_eq!(pnt.z(), 0.0);
        assert_eq!(point.index(), 0);
    }

    #[test]
    fn test_bods_point_set_pnt() {
        let mut point = BodsPoint::new();
        let pnt = GpPnt::new(1.5, 2.5, 3.5);
        point.set_pnt(&pnt);

        let retrieved = point.pnt();
        assert_eq!(retrieved.x(), 1.5);
        assert_eq!(retrieved.y(), 2.5);
        assert_eq!(retrieved.z(), 3.5);
    }

    #[test]
    fn test_bods_point_set_pnt2d1() {
        let mut point = BodsPoint::new();
        let pnt = GpPnt2d::new(0.2, 0.3);
        point.set_pnt2d1(&pnt);

        let retrieved = point.pnt2d1();
        assert_eq!(retrieved.x(), 0.2);
        assert_eq!(retrieved.y(), 0.3);
    }

    #[test]
    fn test_bods_point_set_pnt2d2() {
        let mut point = BodsPoint::new();
        let pnt = GpPnt2d::new(0.4, 0.5);
        point.set_pnt2d2(&pnt);

        let retrieved = point.pnt2d2();
        assert_eq!(retrieved.x(), 0.4);
        assert_eq!(retrieved.y(), 0.5);
    }

    #[test]
    fn test_bods_point_set_index() {
        let mut point = BodsPoint::new();
        point.set_index(5);
        assert_eq!(point.index(), 5);
    }

    #[test]
    fn test_bods_point_full() {
        let mut point = BodsPoint::new();
        let pnt3d = GpPnt::new(1.0, 2.0, 3.0);
        let pnt2d1 = GpPnt2d::new(0.1, 0.2);
        let pnt2d2 = GpPnt2d::new(0.3, 0.4);

        point.set_pnt(&pnt3d);
        point.set_pnt2d1(&pnt2d1);
        point.set_pnt2d2(&pnt2d2);
        point.set_index(7);

        assert_eq!(point.pnt().x(), 1.0);
        assert_eq!(point.pnt2d1().x(), 0.1);
        assert_eq!(point.pnt2d2().y(), 0.4);
        assert_eq!(point.index(), 7);
    }

    #[test]
    fn test_default() {
        let point = BodsPoint::default();
        assert_eq!(point.index(), 0);
    }
}
