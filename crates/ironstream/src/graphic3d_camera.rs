// FILE: rust/ironstream/crates/ironstream/src/graphic3d_camera.rs

// occt: Graphic3d_Camera // ::Projection
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Graphic3dProjectionType {
    Orthographic,
    Perspective,
    Stereo,
    MonoLeftEye,
    MonoRightEye,
}

// occt: Graphic3d_Camera
pub struct Graphic3dCamera {
    eye: [f64; 3],
    center: [f64; 3],
    up: [f64; 3],
    fov_y: f64,
    z_near: f64,
    z_far: f64,
    projection: Graphic3dProjectionType,
    scale: f64,
}

impl Graphic3dCamera {
    /// occt: Graphic3d_Camera // ::Graphic3d_Camera — default constructor
    pub fn new() -> Self {
        Self {
            eye: [0.0, 0.0, 1.0],
            center: [0.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            fov_y: 45.0,
            z_near: 0.01,
            z_far: 1000.0,
            projection: Graphic3dProjectionType::Perspective,
            scale: 1.0,
        }
    }

    // --- eye / center / up ---

    /// occt: Graphic3d_Camera // ::SetEye
    pub fn set_eye(&mut self, pos: [f64; 3]) {
        self.eye = pos;
    }

    /// occt: Graphic3d_Camera // ::Eye
    pub fn eye(&self) -> [f64; 3] {
        self.eye
    }

    /// occt: Graphic3d_Camera // ::SetCenter
    pub fn set_center(&mut self, c: [f64; 3]) {
        self.center = c;
    }

    /// occt: Graphic3d_Camera // ::Center
    pub fn center(&self) -> [f64; 3] {
        self.center
    }

    /// occt: Graphic3d_Camera // ::SetUp
    pub fn set_up(&mut self, u: [f64; 3]) {
        self.up = u;
    }

    /// occt: Graphic3d_Camera // ::Up
    pub fn up(&self) -> [f64; 3] {
        self.up
    }

    // --- FOV ---

    /// occt: Graphic3d_Camera // ::SetFOVy
    pub fn set_fov_y(&mut self, f: f64) {
        self.fov_y = f;
    }

    /// occt: Graphic3d_Camera // ::FOVy
    pub fn fov_y(&self) -> f64 {
        self.fov_y
    }

    // --- clipping planes ---

    /// occt: Graphic3d_Camera // ::SetZNear
    pub fn set_z_near(&mut self, v: f64) {
        self.z_near = v;
    }

    /// occt: Graphic3d_Camera // ::ZNear
    pub fn z_near(&self) -> f64 {
        self.z_near
    }

    /// occt: Graphic3d_Camera // ::SetZFar
    pub fn set_z_far(&mut self, v: f64) {
        self.z_far = v;
    }

    /// occt: Graphic3d_Camera // ::ZFar
    pub fn z_far(&self) -> f64 {
        self.z_far
    }

    // --- projection ---

    /// occt: Graphic3d_Camera // ::SetProjectionType
    pub fn set_projection(&mut self, p: Graphic3dProjectionType) {
        self.projection = p;
    }

    /// occt: Graphic3d_Camera // ::GetProjectionType
    pub fn projection(&self) -> Graphic3dProjectionType {
        self.projection
    }

    // --- scale (orthographic) ---

    /// occt: Graphic3d_Camera // ::SetScale
    pub fn set_scale(&mut self, s: f64) {
        self.scale = s;
    }

    /// occt: Graphic3d_Camera // ::Scale
    pub fn scale(&self) -> f64 {
        self.scale
    }

    // --- derived ---

    /// occt: Graphic3d_Camera // ::Distance — magnitude of the eye-to-center vector
    pub fn distance(&self) -> f64 {
        let dx = self.eye[0] - self.center[0];
        let dy = self.eye[1] - self.center[1];
        let dz = self.eye[2] - self.center[2];
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// occt: Graphic3d_Camera // ::OrientationMatrix — column-major look-at view matrix.
    ///
    /// Builds a right-handed look-at matrix identical to the one produced by
    /// OpenGL `gluLookAt` (and OCCT's own `Graphic3d_Camera::OrientationMatrix`).
    ///
    /// Row layout (row-major storage, OpenGL-compatible column-major semantics):
    ///   [ rx  ry  rz  -dot(r,eye) ]
    ///   [ ux  uy  uz  -dot(u,eye) ]
    ///   [ fx  fy  fz  -dot(f,eye) ]
    ///   [ 0   0   0   1           ]
    /// where f = normalize(eye - center) (forward = -z), r = normalize(f × up), u = f × r.
    pub fn view_matrix(&self) -> [[f64; 4]; 4] {
        // forward = normalize(eye - center)  (points from center toward eye, i.e. +Z in view space)
        let fx = self.eye[0] - self.center[0];
        let fy = self.eye[1] - self.center[1];
        let fz = self.eye[2] - self.center[2];
        let flen = (fx * fx + fy * fy + fz * fz).sqrt();
        let (fx, fy, fz) = if flen > 0.0 {
            (fx / flen, fy / flen, fz / flen)
        } else {
            (0.0, 0.0, 1.0)
        };

        // right = normalize(up × forward)
        let ux = self.up[0];
        let uy = self.up[1];
        let uz = self.up[2];

        // cross(up, forward) — note: gluLookAt uses cross(up, -view_dir) = cross(up, forward)
        // Actually gluLookAt: s = normalize(f × up), u = s × f where f = normalize(center-eye).
        // Using the same convention: f here is normalize(eye-center) = -view_dir.
        // right = normalize(up × f) = normalize(-up × view_dir)
        // To match gluLookAt exactly: use forward = normalize(center-eye), right = forward × up.
        let vx = -fx; // view_dir = normalize(center - eye)
        let vy = -fy;
        let vz = -fz;

        // right = normalize(view_dir × up)
        let rx = vy * uz - vz * uy;
        let ry = vz * ux - vx * uz;
        let rz = vx * uy - vy * ux;
        let rlen = (rx * rx + ry * ry + rz * rz).sqrt();
        let (rx, ry, rz) = if rlen > 0.0 {
            (rx / rlen, ry / rlen, rz / rlen)
        } else {
            (1.0, 0.0, 0.0)
        };

        // recompute up = right × view_dir  (orthonormalize)
        let ux2 = ry * vz - rz * vy;
        let uy2 = rz * vx - rx * vz;
        let uz2 = rx * vy - ry * vx;

        let ex = self.eye[0];
        let ey = self.eye[1];
        let ez = self.eye[2];

        [
            [rx,  ry,  rz,  -(rx * ex + ry * ey + rz * ez)],
            [ux2, uy2, uz2, -(ux2 * ex + uy2 * ey + uz2 * ez)],
            [-vx, -vy, -vz,  vx * ex + vy * ey + vz * ez],
            [0.0, 0.0, 0.0, 1.0],
        ]
    }

    /// occt: Graphic3d_Camera // ::ProjectionMatrix (orthographic variant) —
    /// axis-aligned orthographic projection with symmetric frustum.
    ///
    /// Maps the box  [-s/2, s/2] × [-s/2, s/2] × [z_near, z_far]  to the
    /// OpenGL NDC cube [-1,1]³ (depth: z_near → -1, z_far → +1).
    /// `s` is `self.scale`.
    ///
    /// Derived from NDC_z = A*z + B where A*n+B=-1 and A*f+B=1:
    ///   A = 2/(f-n),  B = -(f+n)/(f-n)
    ///
    /// Matrix (row-major):
    ///   [ 2/s   0     0      0          ]
    ///   [ 0     2/s   0      0          ]
    ///   [ 0     0     2/(f-n)  -(f+n)/(f-n) ]
    ///   [ 0     0     0      1          ]
    pub fn ortho_projection_matrix(&self) -> [[f64; 4]; 4] {
        let s = self.scale;
        let n = self.z_near;
        let f = self.z_far;
        let inv_s = if s != 0.0 { 1.0 / s } else { 1.0 };
        let inv_depth = if (f - n).abs() > 0.0 { 1.0 / (f - n) } else { 1.0 };

        [
            [2.0 * inv_s, 0.0,         0.0,                 0.0],
            [0.0,         2.0 * inv_s, 0.0,                 0.0],
            [0.0,         0.0,         2.0 * inv_depth,     -(f + n) * inv_depth],
            [0.0,         0.0,         0.0,                 1.0],
        ]
    }
}

impl Default for Graphic3dCamera {
    fn default() -> Self {
        Self::new()
    }
}
