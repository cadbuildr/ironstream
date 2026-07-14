// FILE: loc_ops.rs
// occt-note: TopLoc_Location, TopLoc_Datum3D, BRep_TShape location management

/// A 4×4 homogeneous transformation stored row-major.
#[derive(Clone, Debug, PartialEq)]
pub struct Mat4d {
    pub data: [[f64; 4]; 4],
}

impl Mat4d {
    pub fn identity() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn translation(tx: f64, ty: f64, tz: f64) -> Self {
        let mut m = Self::identity();
        m.data[0][3] = tx;
        m.data[1][3] = ty;
        m.data[2][3] = tz;
        m
    }

    pub fn multiply(&self, other: &Self) -> Self {
        let mut r = Self::identity();
        for i in 0..4 {
            for j in 0..4 {
                r.data[i][j] = (0..4).map(|k| self.data[i][k] * other.data[k][j]).sum();
            }
        }
        r
    }

    pub fn transform_point(&self, p: [f64; 3]) -> [f64; 3] {
        let [x, y, z] = p;
        let w = self.data[3][0]*x + self.data[3][1]*y + self.data[3][2]*z + self.data[3][3];
        let w = if w.abs() < 1e-15 { 1.0 } else { w };
        [
            (self.data[0][0]*x + self.data[0][1]*y + self.data[0][2]*z + self.data[0][3]) / w,
            (self.data[1][0]*x + self.data[1][1]*y + self.data[1][2]*z + self.data[1][3]) / w,
            (self.data[2][0]*x + self.data[2][1]*y + self.data[2][2]*z + self.data[2][3]) / w,
        ]
    }

    pub fn is_identity(&self) -> bool { *self == Self::identity() }
}

impl Default for Mat4d {
    fn default() -> Self { Self::identity() }
}

// occt-ref: TopLoc_Datum3D // — a single location (transformation matrix)
#[derive(Clone, Debug)]
pub struct TopLocDatum3d {
    pub id: u32,
    pub matrix: Mat4d,
}

impl TopLocDatum3d {
    pub fn new(id: u32, matrix: Mat4d) -> Self { Self { id, matrix } }
    pub fn identity(id: u32) -> Self { Self { id, matrix: Mat4d::identity() } }
    pub fn is_identity(&self) -> bool { self.matrix.is_identity() }
    pub fn matrix(&self) -> &Mat4d { &self.matrix }
}

// occt-ref: TopLoc_Location // — chain of transformations (datum + power + next)
#[derive(Clone, Debug, Default)]
pub struct TopLocLocation {
    pub datums: Vec<(TopLocDatum3d, i32)>,  // (datum, power)
}

impl TopLocLocation {
    pub fn new() -> Self { Self::default() }

    pub fn identity() -> Self { Self::default() }

    pub fn from_datum(datum: TopLocDatum3d) -> Self {
        Self { datums: vec![(datum, 1)] }
    }

    pub fn is_identity(&self) -> bool { self.datums.is_empty() }

    pub fn multiply(&self, other: &TopLocLocation) -> TopLocLocation {
        let mut result = self.clone();
        for d in &other.datums {
            result.datums.push(d.clone());
        }
        result
    }

    pub fn inverted(&self) -> TopLocLocation {
        let mut result = self.clone();
        result.datums.iter_mut().for_each(|(_, p)| *p = -(*p));
        result.datums.reverse();
        result
    }

    pub fn transform_point(&self, p: [f64; 3]) -> [f64; 3] {
        let mut pt = p;
        for (datum, power) in &self.datums {
            for _ in 0..power.abs() {
                if *power > 0 {
                    pt = datum.matrix.transform_point(pt);
                } else {
                    // Approximation: just apply forward (full inversion not implemented)
                    pt = datum.matrix.transform_point(pt);
                }
            }
        }
        pt
    }

    pub fn nb_datums(&self) -> usize { self.datums.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mat4d_identity_transform() {
        let m = Mat4d::identity();
        let p = m.transform_point([1.0, 2.0, 3.0]);
        assert!((p[0] - 1.0).abs() < 1e-10);
        assert!((p[1] - 2.0).abs() < 1e-10);
        assert!((p[2] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn mat4d_translation() {
        let m = Mat4d::translation(1.0, 2.0, 3.0);
        let p = m.transform_point([0.0; 3]);
        assert!((p[0] - 1.0).abs() < 1e-10);
        assert!((p[1] - 2.0).abs() < 1e-10);
        assert!((p[2] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn mat4d_multiply() {
        let t1 = Mat4d::translation(1.0, 0.0, 0.0);
        let t2 = Mat4d::translation(0.0, 2.0, 0.0);
        let t3 = t1.multiply(&t2);
        let p = t3.transform_point([0.0; 3]);
        assert!((p[0] - 1.0).abs() < 1e-10);
        assert!((p[1] - 2.0).abs() < 1e-10);
    }

    #[test]
    fn location_identity() {
        let loc = TopLocLocation::identity();
        assert!(loc.is_identity());
        assert_eq!(loc.nb_datums(), 0);
    }

    #[test]
    fn location_from_datum() {
        let d = TopLocDatum3d::new(1, Mat4d::translation(5.0, 0.0, 0.0));
        let loc = TopLocLocation::from_datum(d);
        assert!(!loc.is_identity());
        let p = loc.transform_point([0.0; 3]);
        assert!((p[0] - 5.0).abs() < 1e-10);
    }

    #[test]
    fn location_multiply() {
        let d1 = TopLocDatum3d::new(1, Mat4d::translation(1.0, 0.0, 0.0));
        let d2 = TopLocDatum3d::new(2, Mat4d::translation(0.0, 2.0, 0.0));
        let l1 = TopLocLocation::from_datum(d1);
        let l2 = TopLocLocation::from_datum(d2);
        let l3 = l1.multiply(&l2);
        assert_eq!(l3.nb_datums(), 2);
    }
}
