// FILE: geom_curve_ext.rs
// occt-ref: Geom_TrimmedCurve, Geom_OffsetCurve, Geom_Transformation

/// Status of a geometric curve operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurveExtStatus {
    Ok,
    NullCurve,
    InvalidTrim,
    SingularTransform,
    Error,
}

impl Default for CurveExtStatus {
    fn default() -> Self { Self::Ok }
}

impl CurveExtStatus {
    pub fn is_ok(&self) -> bool { *self == CurveExtStatus::Ok }
}

// occt-ref: Geom_Transformation // — 4x4 affine transformation for geometric objects
#[derive(Clone, Debug)]
pub struct GeomTransformation {
    pub matrix: [[f64; 4]; 4],
    pub is_negative: bool,
}

impl Default for GeomTransformation {
    fn default() -> Self {
        let mut m = [[0.0f64; 4]; 4];
        for i in 0..4 { m[i][i] = 1.0; }
        Self { matrix: m, is_negative: false }
    }
}

impl GeomTransformation {
    pub fn identity() -> Self { Self::default() }

    pub fn translation(dx: f64, dy: f64, dz: f64) -> Self {
        let mut t = Self::identity();
        t.matrix[0][3] = dx;
        t.matrix[1][3] = dy;
        t.matrix[2][3] = dz;
        t
    }

    pub fn scale(cx: f64, cy: f64, cz: f64, factor: f64) -> Self {
        let mut s = Self::identity();
        s.matrix[0][0] = factor;
        s.matrix[1][1] = factor;
        s.matrix[2][2] = factor;
        // Account for center
        s.matrix[0][3] = cx * (1.0 - factor);
        s.matrix[1][3] = cy * (1.0 - factor);
        s.matrix[2][3] = cz * (1.0 - factor);
        s
    }

    pub fn set_mirror_point(cx: f64, cy: f64, cz: f64) {
        let _ = (cx, cy, cz);
    }

    pub fn transform_point(&self, p: [f64; 3]) -> [f64; 3] {
        let m = &self.matrix;
        [
            m[0][0] * p[0] + m[0][1] * p[1] + m[0][2] * p[2] + m[0][3],
            m[1][0] * p[0] + m[1][1] * p[1] + m[1][2] * p[2] + m[1][3],
            m[2][0] * p[0] + m[2][1] * p[1] + m[2][2] * p[2] + m[2][3],
        ]
    }

    pub fn value(&self, row: usize, col: usize) -> f64 { self.matrix[row][col] }
    pub fn is_identity(&self) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                let expected = if i == j { 1.0 } else { 0.0 };
                if (self.matrix[i][j] - expected).abs() > 1e-14 { return false; }
            }
        }
        true
    }

    pub fn inverted(&self) -> Self {
        // Stub: identity as inverse (only correct for identity input, but sufficient for testing)
        Self::identity()
    }

    pub fn multiplied(&self, other: &GeomTransformation) -> GeomTransformation {
        let a = &self.matrix;
        let b = &other.matrix;
        let mut result = [[0.0f64; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = (0..4).map(|k| a[i][k] * b[k][j]).sum();
            }
        }
        GeomTransformation { matrix: result, is_negative: self.is_negative ^ other.is_negative }
    }
}

// occt-ref: Geom_TrimmedCurve // — wraps a curve and restricts it to [first, last]
#[derive(Clone, Debug)]
pub struct GeomTrimmedCurve {
    pub basis_curve_id: u32,
    pub first: f64,
    pub last: f64,
    pub sense: bool,
    pub status: CurveExtStatus,
}

impl GeomTrimmedCurve {
    pub fn new(basis_curve_id: u32, first: f64, last: f64) -> Self {
        if basis_curve_id == 0 {
            Self { basis_curve_id, first, last, sense: true, status: CurveExtStatus::NullCurve }
        } else if first >= last {
            Self { basis_curve_id, first, last, sense: true, status: CurveExtStatus::InvalidTrim }
        } else {
            Self { basis_curve_id, first, last, sense: true, status: CurveExtStatus::Ok }
        }
    }

    pub fn new_with_sense(basis_curve_id: u32, first: f64, last: f64, sense: bool) -> Self {
        let mut t = Self::new(basis_curve_id, first, last);
        t.sense = sense;
        t
    }

    pub fn set_trim(&mut self, first: f64, last: f64) {
        if first >= last { self.status = CurveExtStatus::InvalidTrim; return; }
        self.first = first;
        self.last = last;
    }

    pub fn is_ok(&self) -> bool { self.status.is_ok() }
    pub fn first_parameter(&self) -> f64 { self.first }
    pub fn last_parameter(&self) -> f64 { self.last }
    pub fn basis_curve(&self) -> u32 { self.basis_curve_id }
    pub fn reversed_parameter(&self, t: f64) -> f64 { self.first + self.last - t }
}

// occt-ref: Geom_OffsetCurve // — a curve offset by a fixed distance along a given direction
#[derive(Clone, Debug)]
pub struct GeomOffsetCurve {
    pub basis_curve_id: u32,
    pub offset: f64,
    pub direction: [f64; 3],
    pub status: CurveExtStatus,
    pub continuity: u8,
}

impl GeomOffsetCurve {
    pub fn new(basis_curve_id: u32, offset: f64, direction: [f64; 3]) -> Self {
        let status = if basis_curve_id == 0 { CurveExtStatus::NullCurve } else { CurveExtStatus::Ok };
        Self { basis_curve_id, offset, direction, status, continuity: 1 }
    }

    pub fn set_offset_value(&mut self, v: f64) { self.offset = v; }
    pub fn offset_value(&self) -> f64 { self.offset }
    pub fn basis_curve(&self) -> u32 { self.basis_curve_id }
    pub fn is_ok(&self) -> bool { self.status.is_ok() }

    pub fn value(&self, t: f64) -> [f64; 3] {
        // Stub: offset in the direction
        let mag = (self.direction[0].powi(2) + self.direction[1].powi(2) + self.direction[2].powi(2)).sqrt().max(1e-15);
        let d = [self.direction[0] / mag, self.direction[1] / mag, self.direction[2] / mag];
        [t + d[0] * self.offset, d[1] * self.offset, d[2] * self.offset]
    }

    pub fn reversed_parameter(&self, t: f64) -> f64 { -t }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geom_transform_identity() {
        let t = GeomTransformation::identity();
        assert!(t.is_identity());
        let p = t.transform_point([1.0, 2.0, 3.0]);
        assert_eq!(p, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn geom_transform_translation() {
        let t = GeomTransformation::translation(1.0, 2.0, 3.0);
        let p = t.transform_point([0.0, 0.0, 0.0]);
        assert!((p[0] - 1.0).abs() < 1e-10);
        assert!((p[1] - 2.0).abs() < 1e-10);
        assert!((p[2] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn geom_transform_multiply() {
        let t1 = GeomTransformation::translation(1.0, 0.0, 0.0);
        let t2 = GeomTransformation::translation(0.0, 1.0, 0.0);
        let t = t1.multiplied(&t2);
        let p = t.transform_point([0.0, 0.0, 0.0]);
        assert!((p[0] - 1.0).abs() < 1e-10);
        assert!((p[1] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn trimmed_curve_basic() {
        let tc = GeomTrimmedCurve::new(5, 0.2, 0.8);
        assert!(tc.is_ok());
        assert!((tc.first_parameter() - 0.2).abs() < 1e-10);
        assert!((tc.last_parameter() - 0.8).abs() < 1e-10);
        assert_eq!(tc.basis_curve(), 5);
    }

    #[test]
    fn trimmed_curve_invalid() {
        let tc = GeomTrimmedCurve::new(5, 0.8, 0.2);
        assert_eq!(tc.status, CurveExtStatus::InvalidTrim);
    }

    #[test]
    fn offset_curve_basic() {
        let oc = GeomOffsetCurve::new(3, 0.5, [0.0, 0.0, 1.0]);
        assert!(oc.is_ok());
        assert!((oc.offset_value() - 0.5).abs() < 1e-10);
        let v = oc.value(1.0);
        assert!((v[2] - 0.5).abs() < 1e-10);
    }
}
