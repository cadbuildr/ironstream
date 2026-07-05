// FILE: select_mgr_vector_types.rs
// occt: SelectMgr_VectorTypes

//! Deprecated typedef header (deprecated since OCCT 8.0.0):
//! `typedef NCollection_Vec3<double> SelectMgr_Vec3;`
//! `typedef NCollection_Vec4<double> SelectMgr_Vec4;`
//! `typedef NCollection_Mat4<double> SelectMgr_Mat4;`
//!
//! Faithful port: small double-precision vec3/vec4 and a column-major
//! 4x4 matrix with the NCollection operations these typedefs expose.

/// `SelectMgr_Vec3` = NCollection_Vec3<double>.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SelectMgrVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl SelectMgrVec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        SelectMgrVec3 { x, y, z }
    }

    pub fn zero() -> Self {
        SelectMgrVec3::new(0.0, 0.0, 0.0)
    }

    pub fn dot(&self, o: &SelectMgrVec3) -> f64 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }

    pub fn cross(&self, o: &SelectMgrVec3) -> SelectMgrVec3 {
        SelectMgrVec3::new(
            self.y * o.z - self.z * o.y,
            self.z * o.x - self.x * o.z,
            self.x * o.y - self.y * o.x,
        )
    }

    pub fn modulus(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalized(&self) -> SelectMgrVec3 {
        let m = self.modulus();
        assert!(m > 0.0, "NCollection_Vec3: normalize of zero vector");
        SelectMgrVec3::new(self.x / m, self.y / m, self.z / m)
    }

    pub fn added(&self, o: &SelectMgrVec3) -> SelectMgrVec3 {
        SelectMgrVec3::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }

    pub fn subtracted(&self, o: &SelectMgrVec3) -> SelectMgrVec3 {
        SelectMgrVec3::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }

    pub fn multiplied(&self, s: f64) -> SelectMgrVec3 {
        SelectMgrVec3::new(self.x * s, self.y * s, self.z * s)
    }
}

/// `SelectMgr_Vec4` = NCollection_Vec4<double>.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SelectMgrVec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl SelectMgrVec4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        SelectMgrVec4 { x, y, z, w }
    }

    /// NCollection_Vec4::xyz().
    pub fn xyz(&self) -> SelectMgrVec3 {
        SelectMgrVec3::new(self.x, self.y, self.z)
    }

    pub fn dot(&self, o: &SelectMgrVec4) -> f64 {
        self.x * o.x + self.y * o.y + self.z * o.z + self.w * o.w
    }
}

/// `SelectMgr_Mat4` = NCollection_Mat4<double>, column-major storage,
/// initialized to identity (NCollection_Mat4 default).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SelectMgrMat4 {
    /// mat[col][row] as in NCollection_Mat4 (column-major).
    m: [[f64; 4]; 4],
}

impl Default for SelectMgrMat4 {
    fn default() -> Self {
        SelectMgrMat4::identity()
    }
}

impl SelectMgrMat4 {
    pub fn identity() -> Self {
        let mut m = [[0.0; 4]; 4];
        for i in 0..4 {
            m[i][i] = 1.0;
        }
        SelectMgrMat4 { m }
    }

    /// GetValue(row, col).
    pub fn get_value(&self, row: usize, col: usize) -> f64 {
        self.m[col][row]
    }

    /// SetValue(row, col, value).
    pub fn set_value(&mut self, row: usize, col: usize, value: f64) {
        self.m[col][row] = value;
    }

    /// IsIdentity check.
    pub fn is_identity(&self) -> bool {
        *self == SelectMgrMat4::identity()
    }

    /// Multiplied(other) — matrix product self * other.
    pub fn multiplied(&self, o: &SelectMgrMat4) -> SelectMgrMat4 {
        let mut r = SelectMgrMat4 { m: [[0.0; 4]; 4] };
        for row in 0..4 {
            for col in 0..4 {
                let mut acc = 0.0;
                for k in 0..4 {
                    acc += self.get_value(row, k) * o.get_value(k, col);
                }
                r.set_value(row, col, acc);
            }
        }
        r
    }

    /// Matrix * Vec4 product.
    pub fn multiply_vec4(&self, v: &SelectMgrVec4) -> SelectMgrVec4 {
        let comp = |row: usize| {
            self.get_value(row, 0) * v.x
                + self.get_value(row, 1) * v.y
                + self.get_value(row, 2) * v.z
                + self.get_value(row, 3) * v.w
        };
        SelectMgrVec4::new(comp(0), comp(1), comp(2), comp(3))
    }

    /// Translation matrix helper (SetColumn(3, vec) pattern).
    pub fn translation(t: &SelectMgrVec3) -> SelectMgrMat4 {
        let mut m = SelectMgrMat4::identity();
        m.set_value(0, 3, t.x);
        m.set_value(1, 3, t.y);
        m.set_value(2, 3, t.z);
        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3_algebra() {
        let a = SelectMgrVec3::new(1.0, 0.0, 0.0);
        let b = SelectMgrVec3::new(0.0, 1.0, 0.0);
        assert_eq!(a.dot(&b), 0.0);
        assert_eq!(a.cross(&b), SelectMgrVec3::new(0.0, 0.0, 1.0));
        assert_eq!(a.added(&b).modulus(), 2.0_f64.sqrt());
        assert_eq!(a.multiplied(3.0).x, 3.0);
        let n = SelectMgrVec3::new(0.0, 0.0, 5.0).normalized();
        assert_eq!(n, SelectMgrVec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn vec4_swizzle_and_dot() {
        let v = SelectMgrVec4::new(1.0, 2.0, 3.0, 1.0);
        assert_eq!(v.xyz(), SelectMgrVec3::new(1.0, 2.0, 3.0));
        assert_eq!(v.dot(&SelectMgrVec4::new(1.0, 1.0, 1.0, 1.0)), 7.0);
    }

    #[test]
    fn mat4_defaults_to_identity() {
        let m = SelectMgrMat4::default();
        assert!(m.is_identity());
        assert_eq!(m.get_value(2, 2), 1.0);
        assert_eq!(m.get_value(0, 3), 0.0);
    }

    #[test]
    fn mat4_translation_applied_to_point() {
        let t = SelectMgrMat4::translation(&SelectMgrVec3::new(10.0, -2.0, 0.5));
        let p = SelectMgrVec4::new(1.0, 1.0, 1.0, 1.0);
        let moved = t.multiply_vec4(&p);
        assert_eq!(moved, SelectMgrVec4::new(11.0, -1.0, 1.5, 1.0));
        // Direction vectors (w = 0) are unaffected by translation.
        let d = SelectMgrVec4::new(1.0, 0.0, 0.0, 0.0);
        assert_eq!(t.multiply_vec4(&d), d);
    }

    #[test]
    fn mat4_product_with_identity() {
        let mut a = SelectMgrMat4::identity();
        a.set_value(0, 1, 4.0);
        let prod = a.multiplied(&SelectMgrMat4::identity());
        assert_eq!(prod, a);
    }
}
