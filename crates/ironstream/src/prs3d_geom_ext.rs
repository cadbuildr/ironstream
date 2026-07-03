// FILE: prs3d_geom_ext.rs
// occt: Prs3d_ToolSphere, Prs3d_ToolCylinder, Prs3d_ToolTorus, Prs3d_ToolDisk

/// Common mesh tool state: resolution along U and V.
#[derive(Clone, Copy, Debug)]
pub struct PrsMeshResolution {
    pub nb_u: usize,
    pub nb_v: usize,
}

impl Default for PrsMeshResolution {
    fn default() -> Self { Self { nb_u: 16, nb_v: 8 } }
}

impl PrsMeshResolution {
    pub fn new(nb_u: usize, nb_v: usize) -> Self {
        Self { nb_u: nb_u.max(3), nb_v: nb_v.max(2) }
    }

    pub fn nb_triangles(&self) -> usize { 2 * self.nb_u * self.nb_v }
    pub fn nb_vertices(&self) -> usize { (self.nb_u + 1) * (self.nb_v + 1) }
}

/// Generates a UV sphere mesh for presentation.
/// occt: Prs3d_ToolSphere
#[derive(Clone, Debug)]
pub struct Prs3dToolSphere {
    pub radius: f64,
    pub resolution: PrsMeshResolution,
}

impl Prs3dToolSphere {
    pub fn new(radius: f64, nb_u: usize, nb_v: usize) -> Self {
        Self { radius: radius.max(0.0), resolution: PrsMeshResolution::new(nb_u, nb_v) }
    }

    /// Generate (u,v) vertices as (x, y, z) on the sphere.
    pub fn vertices(&self) -> Vec<[f64; 3]> {
        use std::f64::consts::PI;
        let nu = self.resolution.nb_u;
        let nv = self.resolution.nb_v;
        let mut verts = Vec::with_capacity((nu+1)*(nv+1));
        for iv in 0..=nv {
            let phi = PI * (iv as f64 / nv as f64) - PI * 0.5;
            for iu in 0..=nu {
                let theta = 2.0 * PI * (iu as f64 / nu as f64);
                verts.push([
                    self.radius * phi.cos() * theta.cos(),
                    self.radius * phi.cos() * theta.sin(),
                    self.radius * phi.sin(),
                ]);
            }
        }
        verts
    }

    pub fn nb_triangles(&self) -> usize { self.resolution.nb_triangles() }
    pub fn nb_vertices(&self) -> usize { self.resolution.nb_vertices() }
}

/// Generates a closed cylinder mesh for presentation.
/// occt: Prs3d_ToolCylinder
#[derive(Clone, Debug)]
pub struct Prs3dToolCylinder {
    pub bottom_radius: f64,
    pub top_radius: f64,
    pub height: f64,
    pub resolution: PrsMeshResolution,
}

impl Prs3dToolCylinder {
    pub fn new(bottom_r: f64, top_r: f64, height: f64, nb_u: usize) -> Self {
        Self {
            bottom_radius: bottom_r.max(0.0),
            top_radius: top_r.max(0.0),
            height: height.max(0.0),
            resolution: PrsMeshResolution::new(nb_u, 1),
        }
    }

    pub fn is_cone(&self) -> bool {
        (self.top_radius - self.bottom_radius).abs() > 1e-10
    }

    /// Ring vertices at height z (0 = bottom, 1 = top).
    pub fn ring_vertices(&self, side: usize) -> Vec<[f64; 3]> {
        use std::f64::consts::PI;
        let r = if side == 0 { self.bottom_radius } else { self.top_radius };
        let z = if side == 0 { 0.0 } else { self.height };
        let nu = self.resolution.nb_u;
        (0..=nu).map(|i| {
            let theta = 2.0 * PI * (i as f64 / nu as f64);
            [r * theta.cos(), r * theta.sin(), z]
        }).collect()
    }

    pub fn nb_side_triangles(&self) -> usize { 2 * self.resolution.nb_u }
}

/// Generates a torus mesh for presentation.
/// occt: Prs3d_ToolTorus
#[derive(Clone, Debug)]
pub struct Prs3dToolTorus {
    pub major_radius: f64,   // center of tube to center of torus
    pub minor_radius: f64,   // tube radius
    pub angle1: f64,         // start angle in radians
    pub angle2: f64,         // end angle
    pub resolution: PrsMeshResolution,
}

impl Prs3dToolTorus {
    pub fn new(major_r: f64, minor_r: f64, nb_u: usize, nb_v: usize) -> Self {
        use std::f64::consts::PI;
        Self {
            major_radius: major_r.max(0.0),
            minor_radius: minor_r.max(0.0),
            angle1: 0.0,
            angle2: 2.0 * PI,
            resolution: PrsMeshResolution::new(nb_u, nb_v),
        }
    }

    pub fn set_angles(&mut self, a1: f64, a2: f64) { self.angle1 = a1; self.angle2 = a2; }

    pub fn arc_angle(&self) -> f64 { (self.angle2 - self.angle1).abs() }

    pub fn is_full(&self) -> bool {
        (self.arc_angle() - 2.0 * std::f64::consts::PI).abs() < 1e-10
    }

    /// Sample vertex at (iu, iv) grid position.
    pub fn vertex(&self, iu: usize, iv: usize) -> [f64; 3] {
        use std::f64::consts::PI;
        let nu = self.resolution.nb_u;
        let nv = self.resolution.nb_v;
        let phi = 2.0 * PI * (iv as f64 / nv as f64);
        let frac = iu as f64 / nu as f64;
        let theta = self.angle1 + frac * self.arc_angle();
        let r = self.major_radius + self.minor_radius * phi.cos();
        [r * theta.cos(), r * theta.sin(), self.minor_radius * phi.sin()]
    }

    pub fn nb_triangles(&self) -> usize { self.resolution.nb_triangles() }
}

/// Generates a disk (annular sector) mesh for presentation.
/// occt: Prs3d_ToolDisk
#[derive(Clone, Debug)]
pub struct Prs3dToolDisk {
    pub inner_radius: f64,
    pub outer_radius: f64,
    pub nb_u: usize,
    pub angle_start: f64,
    pub angle_end: f64,
}

impl Prs3dToolDisk {
    pub fn new(inner: f64, outer: f64, nb_u: usize) -> Self {
        use std::f64::consts::PI;
        Self {
            inner_radius: inner.max(0.0),
            outer_radius: outer.max(inner),
            nb_u: nb_u.max(3),
            angle_start: 0.0,
            angle_end: 2.0 * PI,
        }
    }

    pub fn width(&self) -> f64 { self.outer_radius - self.inner_radius }
    pub fn nb_triangles(&self) -> usize { 2 * self.nb_u }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sphere_vertex_count() {
        let s = Prs3dToolSphere::new(1.0, 16, 8);
        let verts = s.vertices();
        assert_eq!(verts.len(), 17 * 9);
        // Every vertex should be on the unit sphere
        for v in &verts {
            let r = (v[0].powi(2) + v[1].powi(2) + v[2].powi(2)).sqrt();
            assert!((r - 1.0).abs() < 1e-10, "radius = {r}");
        }
    }

    #[test]
    fn sphere_nb_triangles() {
        let s = Prs3dToolSphere::new(2.0, 8, 4);
        assert_eq!(s.nb_triangles(), 2 * 8 * 4);
    }

    #[test]
    fn cylinder_ring_vertices() {
        let c = Prs3dToolCylinder::new(1.0, 1.0, 5.0, 8);
        let bottom = c.ring_vertices(0);
        assert_eq!(bottom.len(), 9);
        assert!((bottom[0][2]).abs() < 1e-10); // z = 0
        let top = c.ring_vertices(1);
        assert!((top[0][2] - 5.0).abs() < 1e-10);
    }

    #[test]
    fn torus_vertex_sample() {
        let t = Prs3dToolTorus::new(3.0, 1.0, 16, 8);
        assert!(t.is_full());
        let v = t.vertex(0, 0);
        // At (0,0): theta=0, phi=0: x = (R + r) = 4, y = 0, z = 0
        assert!((v[0] - 4.0).abs() < 1e-10);
    }

    #[test]
    fn disk_width() {
        let d = Prs3dToolDisk::new(1.0, 3.0, 12);
        assert!((d.width() - 2.0).abs() < 1e-10);
        assert_eq!(d.nb_triangles(), 24);
    }
}
