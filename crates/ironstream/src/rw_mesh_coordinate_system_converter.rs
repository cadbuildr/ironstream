// FILE: rw_mesh_coordinate_system_converter.rs
// occt: RWMesh_CoordinateSystemConverter

//! Coordinate system converter for mesh import/export, following
//! RWMesh_CoordinateSystemConverter.hxx/.cxx. Converts positions, normals
//! and transformations between two axis systems (e.g. Z-up used by OCCT /
//! Blender and Y-up used by glTF) with an optional length-unit scale.
//!
//! The gp primitives it relies on (gp_XYZ, gp_Ax3, gp_Trsf) are modelled as
//! local helper types implementing exactly the operations the converter
//! needs, with the same math as the OCCT sources (see
//! gp_Trsf::SetTransformation).

/// gp::Resolution(): smallest positive real (RealSmall in OCCT).
pub fn gp_resolution() -> f64 {
    f64::MIN_POSITIVE
}

// ---------------------------------------------------------------------------
// Minimal gp-like linear algebra (local plumbing)
// ---------------------------------------------------------------------------

/// Models gp_XYZ.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Xyz {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Xyz {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn modulus(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn is_equal(&self, other: &Xyz, tol: f64) -> bool {
        (self.x - other.x).abs() <= tol
            && (self.y - other.y).abs() <= tol
            && (self.z - other.z).abs() <= tol
    }

    pub fn scaled(&self, s: f64) -> Xyz {
        Xyz::new(self.x * s, self.y * s, self.z * s)
    }

    pub fn added(&self, o: &Xyz) -> Xyz {
        Xyz::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }

    pub fn reversed(&self) -> Xyz {
        Xyz::new(-self.x, -self.y, -self.z)
    }

    pub fn cross(&self, o: &Xyz) -> Xyz {
        Xyz::new(
            self.y * o.z - self.z * o.y,
            self.z * o.x - self.x * o.z,
            self.x * o.y - self.y * o.x,
        )
    }

    pub fn normalized(&self) -> Xyz {
        let m = self.modulus();
        assert!(m > 0.0, "cannot normalize null vector");
        self.scaled(1.0 / m)
    }
}

/// Models gp_Mat (3x3, row-major).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat3 {
    pub m: [[f64; 3]; 3],
}

impl Mat3 {
    pub fn identity() -> Self {
        Self { m: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]] }
    }

    /// gp_Mat::SetRows.
    pub fn from_rows(r1: Xyz, r2: Xyz, r3: Xyz) -> Self {
        Self {
            m: [
                [r1.x, r1.y, r1.z],
                [r2.x, r2.y, r2.z],
                [r3.x, r3.y, r3.z],
            ],
        }
    }

    /// gp_Mat constructor from three columns.
    pub fn from_cols(c1: Xyz, c2: Xyz, c3: Xyz) -> Self {
        Self {
            m: [
                [c1.x, c2.x, c3.x],
                [c1.y, c2.y, c3.y],
                [c1.z, c2.z, c3.z],
            ],
        }
    }

    pub fn multiply_xyz(&self, v: &Xyz) -> Xyz {
        Xyz::new(
            self.m[0][0] * v.x + self.m[0][1] * v.y + self.m[0][2] * v.z,
            self.m[1][0] * v.x + self.m[1][1] * v.y + self.m[1][2] * v.z,
            self.m[2][0] * v.x + self.m[2][1] * v.y + self.m[2][2] * v.z,
        )
    }

    pub fn multiplied(&self, o: &Mat3) -> Mat3 {
        let mut r = [[0.0; 3]; 3];
        for (i, row) in r.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                *cell = (0..3).map(|k| self.m[i][k] * o.m[k][j]).sum();
            }
        }
        Mat3 { m: r }
    }

    pub fn transposed(&self) -> Mat3 {
        let mut r = [[0.0; 3]; 3];
        for (i, row) in r.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                *cell = self.m[j][i];
            }
        }
        Mat3 { m: r }
    }

    pub fn is_identity(&self, tol: f64) -> bool {
        let id = Mat3::identity();
        for i in 0..3 {
            for j in 0..3 {
                if (self.m[i][j] - id.m[i][j]).abs() > tol {
                    return false;
                }
            }
        }
        true
    }
}

/// Models gp_Ax3: a right- or left-handed coordinate system.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ax3 {
    location: Xyz,
    /// Main direction ("Z axis" of the system).
    direction: Xyz,
    x_direction: Xyz,
    y_direction: Xyz,
}

impl Ax3 {
    /// gp_Ax3(P, N, Vx): main direction N, X direction from Vx (made
    /// orthogonal to N), Y direction = N ^ X.
    pub fn new(location: Xyz, n: Xyz, vx: Xyz) -> Self {
        let direction = n.normalized();
        // Component of vx orthogonal to n.
        let dot = vx.x * direction.x + vx.y * direction.y + vx.z * direction.z;
        let x_direction = Xyz::new(
            vx.x - dot * direction.x,
            vx.y - dot * direction.y,
            vx.z - dot * direction.z,
        )
        .normalized();
        let y_direction = direction.cross(&x_direction);
        Self { location, direction, x_direction, y_direction }
    }

    pub fn location(&self) -> Xyz {
        self.location
    }

    pub fn direction(&self) -> Xyz {
        self.direction
    }

    pub fn x_direction(&self) -> Xyz {
        self.x_direction
    }

    pub fn y_direction(&self) -> Xyz {
        self.y_direction
    }
}

impl Default for Ax3 {
    /// gp_Ax3(): standard right-handed system at origin, Z main direction.
    fn default() -> Self {
        Ax3::new(
            Xyz::new(0.0, 0.0, 0.0),
            Xyz::new(0.0, 0.0, 1.0),
            Xyz::new(1.0, 0.0, 0.0),
        )
    }
}

/// Models gp_TrsfForm (only the values the converter distinguishes).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrsfForm {
    Identity,
    CompoundTrsf,
}

/// Models gp_Trsf restricted to rotation + translation (scale = 1), which is
/// all SetTransformation(Ax3, Ax3) produces.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Trsf {
    form: TrsfForm,
    matrix: Mat3,
    loc: Xyz,
}

impl Default for Trsf {
    fn default() -> Self {
        Self { form: TrsfForm::Identity, matrix: Mat3::identity(), loc: Xyz::default() }
    }
}

impl Trsf {
    pub fn form(&self) -> TrsfForm {
        self.form
    }

    /// gp_Trsf::SetTransformation(FromA1, ToA2). Mirrors gp_Trsf.cxx.
    pub fn set_transformation(&mut self, from_a1: &Ax3, to_a2: &Ax3) {
        self.form = TrsfForm::CompoundTrsf;
        // matrix from XOY to A2:
        let mut matrix = Mat3::from_rows(
            to_a2.x_direction(),
            to_a2.y_direction(),
            to_a2.direction(),
        );
        let mut loc = matrix.multiply_xyz(&to_a2.location()).reversed();

        // matrix FromA1 to XOY:
        let ma1 = Mat3::from_cols(
            from_a1.x_direction(),
            from_a1.y_direction(),
            from_a1.direction(),
        );
        let ma1loc = matrix.multiply_xyz(&from_a1.location());
        loc = loc.added(&ma1loc);
        matrix = matrix.multiplied(&ma1);

        self.matrix = matrix;
        self.loc = loc;
    }

    pub fn translation_part(&self) -> Xyz {
        self.loc
    }

    pub fn set_translation_part(&mut self, v: Xyz) {
        self.loc = v;
        if self.form == TrsfForm::Identity
            && !v.is_equal(&Xyz::default(), gp_resolution())
        {
            self.form = TrsfForm::CompoundTrsf;
        }
    }

    /// True if the rotation part is the identity rotation.
    pub fn rotation_is_identity(&self) -> bool {
        // Equivalent to GetRotation().IsEqual(gp_Quaternion()) for a
        // rotation-only matrix.
        self.matrix.is_identity(1.0e-12)
    }

    /// Resets to the identity transformation (gp_Trsf()).
    pub fn set_identity(&mut self) {
        *self = Trsf::default();
    }

    /// gp_Trsf::Transforms: applies rotation then translation in place.
    pub fn transforms(&self, p: &mut Xyz) {
        let r = self.matrix.multiply_xyz(p);
        *p = r.added(&self.loc);
    }

    /// gp_Trsf::Inverted for a rotation+translation transform.
    pub fn inverted(&self) -> Trsf {
        let rt = self.matrix.transposed();
        let loc = rt.multiply_xyz(&self.loc).reversed();
        Trsf { form: self.form, matrix: rt, loc }
    }

    /// Composition self * other (apply `other` first).
    pub fn multiplied(&self, other: &Trsf) -> Trsf {
        let matrix = self.matrix.multiplied(&other.matrix);
        let loc = self.matrix.multiply_xyz(&other.loc).added(&self.loc);
        let form = if matrix.is_identity(1.0e-12)
            && loc.is_equal(&Xyz::default(), 1.0e-12)
        {
            TrsfForm::Identity
        } else {
            TrsfForm::CompoundTrsf
        };
        Trsf { form, matrix, loc }
    }

    pub fn matrix(&self) -> &Mat3 {
        &self.matrix
    }
}

// ---------------------------------------------------------------------------
// RWMesh_CoordinateSystem enum
// ---------------------------------------------------------------------------

/// Models the RWMesh_CoordinateSystem enumeration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RwMeshCoordinateSystem {
    Undefined,
    /// +YForward +Zup +Xright (Blender / Z-up).
    PosYfwdPosZup,
    /// -ZForward +Yup +Xright (glTF / Y-up).
    NegZfwdPosYup,
}

impl RwMeshCoordinateSystem {
    pub const BLENDER: Self = Self::PosYfwdPosZup;
    pub const GLTF: Self = Self::NegZfwdPosYup;
    pub const ZUP: Self = Self::PosYfwdPosZup;
    pub const YUP: Self = Self::NegZfwdPosYup;
}

// ---------------------------------------------------------------------------
// The converter itself (RWMesh_CoordinateSystemConverter)
// ---------------------------------------------------------------------------

/// Coordinate system converter for meshes.
#[derive(Clone, Debug)]
pub struct RwMeshCoordinateSystemConverter {
    input_ax3: Ax3,
    output_ax3: Ax3,
    input_length_unit: f64,
    output_length_unit: f64,
    has_input_ax3: bool,
    has_output_ax3: bool,
    trsf: Trsf,
    trsf_inv: Trsf,
    unit_factor: f64,
    has_scale: bool,
    is_empty: bool,
}

impl Default for RwMeshCoordinateSystemConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl RwMeshCoordinateSystemConverter {
    /// Returns a standard coordinate system definition.
    pub fn standard_coordinate_system(sys: RwMeshCoordinateSystem) -> Ax3 {
        let origin = Xyz::new(0.0, 0.0, 0.0);
        match sys {
            RwMeshCoordinateSystem::PosYfwdPosZup => {
                Ax3::new(origin, Xyz::new(0.0, 0.0, 1.0), Xyz::new(1.0, 0.0, 0.0))
            }
            RwMeshCoordinateSystem::NegZfwdPosYup => {
                Ax3::new(origin, Xyz::new(0.0, 1.0, 0.0), Xyz::new(1.0, 0.0, 0.0))
            }
            RwMeshCoordinateSystem::Undefined => Ax3::default(),
        }
    }

    /// Empty converter.
    pub fn new() -> Self {
        Self {
            input_ax3: Ax3::default(),
            output_ax3: Ax3::default(),
            input_length_unit: -1.0,
            output_length_unit: -1.0,
            has_input_ax3: false,
            has_output_ax3: false,
            trsf: Trsf::default(),
            trsf_inv: Trsf::default(),
            unit_factor: 1.0,
            has_scale: false,
            is_empty: true,
        }
    }

    /// Returns true if the transformation is empty (no rotation, no scale).
    pub fn is_empty(&self) -> bool {
        self.is_empty
    }

    pub fn input_length_unit(&self) -> f64 {
        self.input_length_unit
    }

    /// Sets the input length unit (scale factor to meters).
    pub fn set_input_length_unit(&mut self, input_scale: f64) {
        self.init(
            self.input_ax3,
            input_scale,
            self.output_ax3,
            self.output_length_unit,
        );
    }

    pub fn output_length_unit(&self) -> f64 {
        self.output_length_unit
    }

    /// Sets the output length unit (scale factor to meters).
    pub fn set_output_length_unit(&mut self, output_scale: f64) {
        self.init(
            self.input_ax3,
            self.input_length_unit,
            self.output_ax3,
            output_scale,
        );
    }

    pub fn has_input_coordinate_system(&self) -> bool {
        self.has_input_ax3
    }

    pub fn input_coordinate_system(&self) -> &Ax3 {
        &self.input_ax3
    }

    /// Sets the input coordinate system from an Ax3.
    pub fn set_input_coordinate_system_ax3(&mut self, sys_from: Ax3) {
        self.has_input_ax3 = true;
        self.init(
            sys_from,
            self.input_length_unit,
            self.output_ax3,
            self.output_length_unit,
        );
    }

    /// Sets the input coordinate system from the enumeration.
    pub fn set_input_coordinate_system(&mut self, sys_from: RwMeshCoordinateSystem) {
        self.has_input_ax3 = sys_from != RwMeshCoordinateSystem::Undefined;
        self.init(
            Self::standard_coordinate_system(sys_from),
            self.input_length_unit,
            self.output_ax3,
            self.output_length_unit,
        );
    }

    pub fn has_output_coordinate_system(&self) -> bool {
        self.has_output_ax3
    }

    pub fn output_coordinate_system(&self) -> &Ax3 {
        &self.output_ax3
    }

    /// Sets the output coordinate system from an Ax3.
    pub fn set_output_coordinate_system_ax3(&mut self, sys_to: Ax3) {
        self.has_output_ax3 = true;
        self.init(
            self.input_ax3,
            self.input_length_unit,
            sys_to,
            self.output_length_unit,
        );
    }

    /// Sets the output coordinate system from the enumeration.
    pub fn set_output_coordinate_system(&mut self, sys_to: RwMeshCoordinateSystem) {
        self.has_output_ax3 = sys_to != RwMeshCoordinateSystem::Undefined;
        self.init(
            self.input_ax3,
            self.input_length_unit,
            Self::standard_coordinate_system(sys_to),
            self.output_length_unit,
        );
    }

    /// Initializes the converter. Mirrors Init in the cxx.
    pub fn init(
        &mut self,
        input_system: Ax3,
        input_length_unit: f64,
        output_system: Ax3,
        output_length_unit: f64,
    ) {
        self.input_length_unit = input_length_unit;
        self.output_length_unit = output_length_unit;
        self.input_ax3 = input_system;
        self.output_ax3 = output_system;

        if input_length_unit > 0.0 && output_length_unit > 0.0 {
            self.unit_factor = input_length_unit / output_length_unit;
            self.has_scale = (self.unit_factor - 1.0).abs() > gp_resolution();
        } else {
            self.unit_factor = 1.0;
            self.has_scale = false;
        }

        let mut a_trsf = Trsf::default();
        if self.has_input_ax3 && self.has_output_ax3 {
            a_trsf.set_transformation(&output_system, &input_system);
            if a_trsf
                .translation_part()
                .is_equal(&Xyz::new(0.0, 0.0, 0.0), gp_resolution())
                && a_trsf.rotation_is_identity()
            {
                a_trsf.set_identity();
            }
        }
        self.trsf = a_trsf;
        self.trsf_inv = a_trsf.inverted();
        self.is_empty = !self.has_scale && self.trsf.form() == TrsfForm::Identity;
    }

    /// Transforms a transformation. Mirrors TransformTransformation.
    pub fn transform_transformation(&self, trsf: &mut Trsf) {
        if self.has_scale {
            let trans_part = trsf.translation_part().scaled(self.unit_factor);
            trsf.set_translation_part(trans_part);
        }
        if self.trsf.form() != TrsfForm::Identity {
            *trsf = self.trsf.multiplied(&trsf.multiplied(&self.trsf_inv));
        }
    }

    /// Transforms a position. Mirrors TransformPosition.
    pub fn transform_position(&self, pos: &mut Xyz) {
        if self.has_scale {
            *pos = pos.scaled(self.unit_factor);
        }
        if self.trsf.form() != TrsfForm::Identity {
            self.trsf.transforms(pos);
        }
    }

    /// Transforms a normal (rotation only, no scale, no translation).
    /// Mirrors TransformNormal.
    pub fn transform_normal(&self, norm: &mut [f32; 3]) {
        if self.trsf.form() != TrsfForm::Identity {
            let v = Xyz::new(norm[0] as f64, norm[1] as f64, norm[2] as f64);
            let r = self.trsf.matrix().multiply_xyz(&v);
            *norm = [r.x as f32, r.y as f32, r.z as f32];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1.0e-12;

    fn assert_xyz_eq(a: &Xyz, b: &Xyz) {
        assert!(a.is_equal(b, EPS), "{:?} != {:?}", a, b);
    }

    #[test]
    fn test_empty_converter() {
        let conv = RwMeshCoordinateSystemConverter::new();
        assert!(conv.is_empty());
        assert_eq!(conv.input_length_unit(), -1.0);
        assert_eq!(conv.output_length_unit(), -1.0);
        assert!(!conv.has_input_coordinate_system());
        assert!(!conv.has_output_coordinate_system());

        let mut p = Xyz::new(1.0, 2.0, 3.0);
        conv.transform_position(&mut p);
        assert_xyz_eq(&p, &Xyz::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_standard_coordinate_systems() {
        let zup = RwMeshCoordinateSystemConverter::standard_coordinate_system(
            RwMeshCoordinateSystem::ZUP,
        );
        assert_xyz_eq(&zup.direction(), &Xyz::new(0.0, 0.0, 1.0));
        assert_xyz_eq(&zup.x_direction(), &Xyz::new(1.0, 0.0, 0.0));
        assert_xyz_eq(&zup.y_direction(), &Xyz::new(0.0, 1.0, 0.0));

        let yup = RwMeshCoordinateSystemConverter::standard_coordinate_system(
            RwMeshCoordinateSystem::YUP,
        );
        assert_xyz_eq(&yup.direction(), &Xyz::new(0.0, 1.0, 0.0));
        assert_xyz_eq(&yup.x_direction(), &Xyz::new(1.0, 0.0, 0.0));
        // Y direction = N ^ X = (0,1,0) ^ (1,0,0) = (0,0,-1)
        assert_xyz_eq(&yup.y_direction(), &Xyz::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_gltf_to_zup_position() {
        // Reading glTF: input Y-up, output Z-up.
        let mut conv = RwMeshCoordinateSystemConverter::new();
        conv.set_input_coordinate_system(RwMeshCoordinateSystem::GLTF);
        conv.set_output_coordinate_system(RwMeshCoordinateSystem::ZUP);
        assert!(!conv.is_empty());

        // glTF "up" (0,1,0) becomes Z-up "up" (0,0,1).
        let mut up = Xyz::new(0.0, 1.0, 0.0);
        conv.transform_position(&mut up);
        assert_xyz_eq(&up, &Xyz::new(0.0, 0.0, 1.0));

        // glTF forward (0,0,-1) becomes Z-up forward (0,1,0).
        let mut fwd = Xyz::new(0.0, 0.0, -1.0);
        conv.transform_position(&mut fwd);
        assert_xyz_eq(&fwd, &Xyz::new(0.0, 1.0, 0.0));

        // X stays X.
        let mut right = Xyz::new(1.0, 0.0, 0.0);
        conv.transform_position(&mut right);
        assert_xyz_eq(&right, &Xyz::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_zup_to_gltf_is_inverse() {
        let mut fwdconv = RwMeshCoordinateSystemConverter::new();
        fwdconv.set_input_coordinate_system(RwMeshCoordinateSystem::GLTF);
        fwdconv.set_output_coordinate_system(RwMeshCoordinateSystem::ZUP);

        let mut invconv = RwMeshCoordinateSystemConverter::new();
        invconv.set_input_coordinate_system(RwMeshCoordinateSystem::ZUP);
        invconv.set_output_coordinate_system(RwMeshCoordinateSystem::GLTF);

        let original = Xyz::new(0.3, -1.7, 2.5);
        let mut p = original;
        fwdconv.transform_position(&mut p);
        invconv.transform_position(&mut p);
        assert_xyz_eq(&p, &original);
    }

    #[test]
    fn test_same_system_is_identity() {
        let mut conv = RwMeshCoordinateSystemConverter::new();
        conv.set_input_coordinate_system(RwMeshCoordinateSystem::ZUP);
        conv.set_output_coordinate_system(RwMeshCoordinateSystem::ZUP);
        assert!(conv.is_empty());

        let mut p = Xyz::new(4.0, 5.0, 6.0);
        conv.transform_position(&mut p);
        assert_xyz_eq(&p, &Xyz::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn test_unit_scale() {
        // Input in millimeters (0.001 m), output in meters (1.0 m).
        let mut conv = RwMeshCoordinateSystemConverter::new();
        conv.set_input_length_unit(0.001);
        conv.set_output_length_unit(1.0);
        assert!(!conv.is_empty());
        assert_eq!(conv.input_length_unit(), 0.001);
        assert_eq!(conv.output_length_unit(), 1.0);

        let mut p = Xyz::new(1000.0, 2000.0, -500.0);
        conv.transform_position(&mut p);
        assert_xyz_eq(&p, &Xyz::new(1.0, 2.0, -0.5));
    }

    #[test]
    fn test_undefined_unit_means_no_scale() {
        let mut conv = RwMeshCoordinateSystemConverter::new();
        conv.set_input_length_unit(0.001);
        // Output unit left undefined (-1): no scaling applied.
        assert!(conv.is_empty());
        let mut p = Xyz::new(7.0, 8.0, 9.0);
        conv.transform_position(&mut p);
        assert_xyz_eq(&p, &Xyz::new(7.0, 8.0, 9.0));
    }

    #[test]
    fn test_scale_and_rotation_combined() {
        let mut conv = RwMeshCoordinateSystemConverter::new();
        conv.set_input_coordinate_system(RwMeshCoordinateSystem::GLTF);
        conv.set_output_coordinate_system(RwMeshCoordinateSystem::ZUP);
        conv.set_input_length_unit(0.001);
        conv.set_output_length_unit(1.0);

        // Scale first, then rotate: (0, 1000, 0) mm -> (0, 1, 0) m -> Z-up (0, 0, 1).
        let mut p = Xyz::new(0.0, 1000.0, 0.0);
        conv.transform_position(&mut p);
        assert_xyz_eq(&p, &Xyz::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_transform_normal() {
        let mut conv = RwMeshCoordinateSystemConverter::new();
        conv.set_input_coordinate_system(RwMeshCoordinateSystem::GLTF);
        conv.set_output_coordinate_system(RwMeshCoordinateSystem::ZUP);

        let mut n = [0.0f32, 1.0, 0.0];
        conv.transform_normal(&mut n);
        assert_eq!(n, [0.0, 0.0, 1.0]);

        // Normals are not scaled by unit factor.
        conv.set_input_length_unit(0.001);
        conv.set_output_length_unit(1.0);
        let mut n2 = [1.0f32, 0.0, 0.0];
        conv.transform_normal(&mut n2);
        assert_eq!(n2, [1.0, 0.0, 0.0]);
    }

    #[test]
    fn test_transform_transformation_scale_only() {
        let mut conv = RwMeshCoordinateSystemConverter::new();
        conv.set_input_length_unit(0.001);
        conv.set_output_length_unit(1.0);

        let mut trsf = Trsf::default();
        trsf.set_translation_part(Xyz::new(1000.0, 0.0, 0.0));
        conv.transform_transformation(&mut trsf);
        assert_xyz_eq(&trsf.translation_part(), &Xyz::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_transform_transformation_conjugation() {
        // A translation expressed in glTF coordinates becomes the equivalent
        // translation in Z-up coordinates: T' = R * T * R^-1.
        let mut conv = RwMeshCoordinateSystemConverter::new();
        conv.set_input_coordinate_system(RwMeshCoordinateSystem::GLTF);
        conv.set_output_coordinate_system(RwMeshCoordinateSystem::ZUP);

        let mut trsf = Trsf::default();
        trsf.set_translation_part(Xyz::new(0.0, 1.0, 0.0)); // move "up" in glTF
        conv.transform_transformation(&mut trsf);
        assert_xyz_eq(&trsf.translation_part(), &Xyz::new(0.0, 0.0, 1.0));
        assert!(trsf.rotation_is_identity());
    }

    #[test]
    fn test_input_output_accessors() {
        let mut conv = RwMeshCoordinateSystemConverter::new();
        conv.set_input_coordinate_system(RwMeshCoordinateSystem::GLTF);
        assert!(conv.has_input_coordinate_system());
        assert!(!conv.has_output_coordinate_system());
        assert_xyz_eq(
            &conv.input_coordinate_system().direction(),
            &Xyz::new(0.0, 1.0, 0.0),
        );
        // Only one system defined: no rotation applied.
        assert!(conv.is_empty());

        conv.set_input_coordinate_system(RwMeshCoordinateSystem::Undefined);
        assert!(!conv.has_input_coordinate_system());
    }
}
