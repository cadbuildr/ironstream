// FILE: brep_prim_maker.rs
// occt: BRepPrim_Sphere, BRepPrim_Cone, BRepPrim_Torus, BRepPrim_Cylinder,
//       BRepPrimAPI_MakeSphere, BRepPrimAPI_MakeCone

/// Result of a primitive shell construction.
#[derive(Clone, Debug)]
pub struct BrepPrimShell {
    pub shape_id: u32,
    pub nb_faces: usize,
    pub nb_edges: usize,
    pub nb_vertices: usize,
}

impl BrepPrimShell {
    pub fn new(shape_id: u32, nb_faces: usize, nb_edges: usize, nb_vertices: usize) -> Self {
        Self { shape_id, nb_faces, nb_edges, nb_vertices }
    }
}

/// Sphere primitive builder.
/// occt: BRepPrim_Sphere / BRepPrimAPI_MakeSphere
#[derive(Clone, Debug)]
pub struct BrepPrimSphere {
    pub center: [f64; 3],
    pub radius: f64,
    pub angle1: f64,  // latitude start (radians, -pi/2 to pi/2)
    pub angle2: f64,  // latitude end
    pub angle3: f64,  // longitude sweep (0 to 2*pi)
    pub is_done: bool,
    pub shell: Option<BrepPrimShell>,
}

impl BrepPrimSphere {
    pub fn new(radius: f64) -> Self {
        use std::f64::consts::PI;
        Self {
            center: [0.0; 3],
            radius: radius.max(0.0),
            angle1: -PI / 2.0,
            angle2:  PI / 2.0,
            angle3:  2.0 * PI,
            is_done: false,
            shell: None,
        }
    }

    pub fn with_center(mut self, c: [f64; 3]) -> Self { self.center = c; self }

    pub fn with_angles(mut self, a1: f64, a2: f64, a3: f64) -> Self {
        self.angle1 = a1;
        self.angle2 = a2;
        self.angle3 = a3;
        self
    }

    /// Build the sphere shell. Stub: creates a shell with 6 faces (octant layout).
    pub fn build(&mut self) {
        self.shell = Some(BrepPrimShell::new(
            self.radius as u32 + 1,
            6,
            12,
            8,
        ));
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool { self.is_done }

    pub fn volume(&self) -> f64 {
        use std::f64::consts::PI;
        (4.0 / 3.0) * PI * self.radius.powi(3)
    }

    pub fn surface_area(&self) -> f64 {
        use std::f64::consts::PI;
        4.0 * PI * self.radius.powi(2)
    }
}

/// Cone primitive builder.
/// occt: BRepPrim_Cone / BRepPrimAPI_MakeCone
#[derive(Clone, Debug)]
pub struct BrepPrimCone {
    pub center: [f64; 3],
    pub radius1: f64,   // bottom radius
    pub radius2: f64,   // top radius (0 for sharp apex)
    pub height: f64,
    pub angle: f64,     // sweep angle (2*pi for full cone)
    pub is_done: bool,
    pub shell: Option<BrepPrimShell>,
}

impl BrepPrimCone {
    pub fn new(radius1: f64, radius2: f64, height: f64) -> Self {
        use std::f64::consts::PI;
        Self {
            center: [0.0; 3],
            radius1: radius1.max(0.0),
            radius2: radius2.max(0.0),
            height: height.max(0.0),
            angle: 2.0 * PI,
            is_done: false,
            shell: None,
        }
    }

    pub fn with_center(mut self, c: [f64; 3]) -> Self { self.center = c; self }
    pub fn with_angle(mut self, a: f64) -> Self { self.angle = a; self }

    /// Build stub: 3 faces (bottom, lateral, top) for truncated cone, 2 for apex cone.
    pub fn build(&mut self) {
        let nb_faces = if self.radius2 < 1e-14 { 2 } else { 3 };
        self.shell = Some(BrepPrimShell::new(
            (self.height as u32 + self.radius1 as u32).wrapping_add(1),
            nb_faces,
            nb_faces + 1,
            nb_faces,
        ));
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool { self.is_done }

    pub fn volume(&self) -> f64 {
        use std::f64::consts::PI;
        (PI * self.height / 3.0) * (self.radius1.powi(2) + self.radius1 * self.radius2 + self.radius2.powi(2))
    }

    pub fn slant_height(&self) -> f64 {
        let dr = self.radius1 - self.radius2;
        (dr * dr + self.height * self.height).sqrt()
    }

    pub fn lateral_surface_area(&self) -> f64 {
        use std::f64::consts::PI;
        PI * (self.radius1 + self.radius2) * self.slant_height()
    }
}

/// Cylinder primitive builder.
/// occt: BRepPrim_Cylinder / BRepPrimAPI_MakeCylinder
#[derive(Clone, Debug)]
pub struct BrepPrimCylinder {
    pub center: [f64; 3],
    pub radius: f64,
    pub height: f64,
    pub angle: f64,
    pub is_done: bool,
    pub shell: Option<BrepPrimShell>,
}

impl BrepPrimCylinder {
    pub fn new(radius: f64, height: f64) -> Self {
        use std::f64::consts::PI;
        Self {
            center: [0.0; 3],
            radius: radius.max(0.0),
            height: height.max(0.0),
            angle: 2.0 * PI,
            is_done: false,
            shell: None,
        }
    }

    pub fn with_center(mut self, c: [f64; 3]) -> Self { self.center = c; self }

    pub fn build(&mut self) {
        self.shell = Some(BrepPrimShell::new(
            self.radius as u32 + 1,
            3,
            6,
            4,
        ));
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool { self.is_done }

    pub fn volume(&self) -> f64 {
        use std::f64::consts::PI;
        PI * self.radius.powi(2) * self.height
    }

    pub fn surface_area(&self) -> f64 {
        use std::f64::consts::PI;
        2.0 * PI * self.radius * (self.radius + self.height)
    }
}

/// Torus primitive builder.
/// occt: BRepPrim_Torus / BRepPrimAPI_MakeTorus
#[derive(Clone, Debug)]
pub struct BrepPrimTorus {
    pub center: [f64; 3],
    pub major_radius: f64,
    pub minor_radius: f64,
    pub angle1: f64,
    pub angle2: f64,
    pub angle3: f64,
    pub is_done: bool,
    pub shell: Option<BrepPrimShell>,
}

impl BrepPrimTorus {
    pub fn new(major_radius: f64, minor_radius: f64) -> Self {
        use std::f64::consts::PI;
        Self {
            center: [0.0; 3],
            major_radius: major_radius.max(0.0),
            minor_radius: minor_radius.max(0.0),
            angle1: 0.0,
            angle2: 2.0 * PI,
            angle3: 2.0 * PI,
            is_done: false,
            shell: None,
        }
    }

    pub fn build(&mut self) {
        self.shell = Some(BrepPrimShell::new(1, 1, 2, 0));
        self.is_done = true;
    }

    pub fn is_done(&self) -> bool { self.is_done }

    pub fn volume(&self) -> f64 {
        use std::f64::consts::PI;
        2.0 * PI * PI * self.major_radius * self.minor_radius.powi(2)
    }

    pub fn surface_area(&self) -> f64 {
        use std::f64::consts::PI;
        4.0 * PI * PI * self.major_radius * self.minor_radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sphere_volume_surface() {
        use std::f64::consts::PI;
        let s = BrepPrimSphere::new(1.0);
        assert!((s.volume() - (4.0/3.0)*PI).abs() < 1e-10);
        assert!((s.surface_area() - 4.0*PI).abs() < 1e-10);
    }

    #[test]
    fn sphere_build() {
        let mut s = BrepPrimSphere::new(5.0);
        assert!(!s.is_done());
        s.build();
        assert!(s.is_done());
        assert!(s.shell.is_some());
    }

    #[test]
    fn cone_volume_apex() {
        use std::f64::consts::PI;
        // Full cone: r1=3, r2=0, h=4 → V = π*4/3 * (9+0+0) = 12π
        let mut c = BrepPrimCone::new(3.0, 0.0, 4.0);
        let expected = PI * 4.0 / 3.0 * 9.0;
        assert!((c.volume() - expected).abs() < 1e-10);
        c.build();
        assert!(c.is_done());
        assert_eq!(c.shell.as_ref().unwrap().nb_faces, 2);
    }

    #[test]
    fn cone_truncated_nb_faces() {
        let mut c = BrepPrimCone::new(3.0, 1.5, 4.0);
        c.build();
        assert_eq!(c.shell.as_ref().unwrap().nb_faces, 3);
    }

    #[test]
    fn cylinder_volume() {
        use std::f64::consts::PI;
        let mut cyl = BrepPrimCylinder::new(2.0, 5.0);
        assert!((cyl.volume() - PI*4.0*5.0).abs() < 1e-10);
        cyl.build();
        assert!(cyl.is_done());
    }

    #[test]
    fn torus_volume() {
        use std::f64::consts::PI;
        let t = BrepPrimTorus::new(3.0, 1.0);
        let expected = 2.0 * PI * PI * 3.0 * 1.0;
        assert!((t.volume() - expected).abs() < 1e-10);
    }
}
