//! `BRepPrimAPI` + `BRepBuilderAPI` sweepers — construction of primitive and
//! swept solids, mirroring OCCT's `BRepPrimAPI_MakeBox` / `MakeCylinder` /
//! `MakeSphere` / `MakeCone` / `MakeTorus` and `BRepPrimAPI_MakePrism` /
//! `MakeRevol`.
//!
//! Every constructor returns a watertight [`Solid`] (triangle boundary). Curved
//! surfaces are tessellated at a deterministic resolution; the caller can scale
//! resolution through [`MeshParams`]. Orientation is normalized so signed
//! volume is positive (outward normals).

use crate::geom::Surface;
use crate::gp::{Ax1, Ax3, Pnt, Trsf};
use crate::mesh::TriMesh;
use crate::topods::{Face, Solid, Wire};
use std::f64::consts::PI;

/// Tessellation resolution knobs.
#[derive(Clone, Copy, Debug)]
pub struct MeshParams {
    /// Facets around a full circle for cylinders/cones/spheres/revolutions.
    pub circle_segments: usize,
    /// Latitude bands for spheres / tori minor circle.
    pub lat_segments: usize,
}

impl Default for MeshParams {
    fn default() -> Self {
        Self {
            circle_segments: 64,
            lat_segments: 32,
        }
    }
}

impl MeshParams {
    /// Segments for a circle of the given radius, never below the floor.
    fn segs_for(&self, _radius: f64) -> usize {
        self.circle_segments.max(8)
    }
}

/// Finalize a mesh: weld and flip to outward orientation (positive volume).
fn finalize(mut m: TriMesh) -> Solid {
    m = m.welded(1e-7);
    if m.signed_volume() < 0.0 {
        m.flip();
    }
    Solid::from_mesh(m)
}

/// `BRepPrimAPI_MakeBox` — axis-aligned box with a corner at `corner` extending
/// by `(dx, dy, dz)`.
// occt: BRepPrimAPI_MakeBox
pub fn make_box(corner: Pnt, dx: f64, dy: f64, dz: f64) -> Solid {
    let (dx, dy, dz) = (dx.abs(), dy.abs(), dz.abs());
    let x0 = corner.x;
    let y0 = corner.y;
    let z0 = corner.z;
    let p = |i: usize, j: usize, k: usize| {
        Pnt::new(x0 + dx * i as f64, y0 + dy * j as f64, z0 + dz * k as f64)
    };
    let mut m = TriMesh::new();
    let mut quad = |a: Pnt, b: Pnt, c: Pnt, d: Pnt| {
        m.push_triangle(a, b, c);
        m.push_triangle(a, c, d);
    };
    quad(p(0, 0, 0), p(0, 1, 0), p(1, 1, 0), p(1, 0, 0)); // z=0  (-Z)
    quad(p(0, 0, 1), p(1, 0, 1), p(1, 1, 1), p(0, 1, 1)); // z=dz (+Z)
    quad(p(0, 0, 0), p(1, 0, 0), p(1, 0, 1), p(0, 0, 1)); // y=0  (-Y)
    quad(p(1, 0, 0), p(1, 1, 0), p(1, 1, 1), p(1, 0, 1)); // x=dx (+X)
    quad(p(1, 1, 0), p(0, 1, 0), p(0, 1, 1), p(1, 1, 1)); // y=dy (+Y)
    quad(p(0, 1, 0), p(0, 0, 0), p(0, 0, 1), p(0, 1, 1)); // x=0  (-X)
    finalize(m)
}

/// Box centered on `center` in X/Y and growing in +Z (a convenience next to the
/// corner-anchored [`make_box`]).
pub fn make_box_centered_xy(center: Pnt, w: f64, h: f64, d: f64) -> Solid {
    let corner = Pnt::new(center.x - w / 2.0, center.y - h / 2.0, center.z);
    make_box(corner, w, h, d)
}

fn ring(center: Pnt, radius: f64, z: f64, segs: usize) -> Vec<Pnt> {
    (0..segs)
        .map(|i| {
            let a = 2.0 * PI * i as f64 / segs as f64;
            Pnt::new(center.x + radius * a.cos(), center.y + radius * a.sin(), z)
        })
        .collect()
}

/// `BRepPrimAPI_MakeCylinder` — radius `r`, height `h`, axis +Z from origin.
// occt: BRepPrimAPI_MakeCylinder
pub fn make_cylinder(r: f64, h: f64, mp: MeshParams) -> Solid {
    let mut s = make_cone(r, r, h, mp);
    s.add_hint(Surface::Cylinder {
        placement: Ax3::identity(),
        radius: r,
    });
    s
}

/// `BRepPrimAPI_MakeCone` — base radius `r1`, top radius `r2`, height `h`,
/// axis +Z from origin. `r2 == 0` yields a true cone apex.
// occt: BRepPrimAPI_MakeCone
pub fn make_cone(r1: f64, r2: f64, h: f64, mp: MeshParams) -> Solid {
    let segs = mp.segs_for(r1.max(r2));
    let bottom = ring(Pnt::origin(), r1, 0.0, segs);
    let top = ring(Pnt::origin(), r2, h, segs);
    let mut m = TriMesh::new();
    let c_bot = Pnt::new(0.0, 0.0, 0.0);
    let c_top = Pnt::new(0.0, 0.0, h);
    for i in 0..segs {
        let j = (i + 1) % segs;
        // Side wall.
        if r1 > 1e-12 || r2 > 1e-12 {
            m.push_triangle(bottom[i], bottom[j], top[j]);
            m.push_triangle(bottom[i], top[j], top[i]);
        }
        // Bottom cap (normal -Z): wind CW about +Z.
        if r1 > 1e-12 {
            m.push_triangle(c_bot, bottom[j], bottom[i]);
        }
        // Top cap (normal +Z).
        if r2 > 1e-12 {
            m.push_triangle(c_top, top[i], top[j]);
        }
    }
    let mut s = finalize(m);
    // Stamp a conical-surface hint for genuine frustums/cones (a straight
    // cylinder — r1 == r2 — is handled by make_cylinder). The placement is
    // oriented so the radius grows along +z_dir, as CONICAL_SURFACE expects.
    if (r1 - r2).abs() > 1e-9 && h.abs() > 1e-9 {
        let (loc, axis, r_ref, half_angle) = if r2 > r1 {
            (Pnt::origin(), Pnt::new(0.0, 0.0, 1.0), r1, ((r2 - r1) / h).atan())
        } else {
            (
                Pnt::new(0.0, 0.0, h),
                Pnt::new(0.0, 0.0, -1.0),
                r2,
                ((r1 - r2) / h).atan(),
            )
        };
        s.add_hint(Surface::Cone {
            placement: Ax3::from_origin_normal(loc, axis, Pnt::new(1.0, 0.0, 0.0)),
            radius: r_ref,
            half_angle,
        });
    }
    s
}

/// `BRepPrimAPI_MakeSphere` — radius `r` centered at origin (UV sphere).
// occt: BRepPrimAPI_MakeSphere
pub fn make_sphere(r: f64, mp: MeshParams) -> Solid {
    let u_segs = mp.segs_for(r);
    let v_segs = mp.lat_segments.max(4);
    let p = |iu: usize, iv: usize| {
        let theta = 2.0 * PI * iu as f64 / u_segs as f64; // around Z
        let phi = PI * iv as f64 / v_segs as f64; // 0..PI from +Z to -Z
        Pnt::new(
            r * phi.sin() * theta.cos(),
            r * phi.sin() * theta.sin(),
            r * phi.cos(),
        )
    };
    let mut m = TriMesh::new();
    for iv in 0..v_segs {
        for iu in 0..u_segs {
            let iu1 = (iu + 1) % u_segs;
            let a = p(iu, iv);
            let b = p(iu1, iv);
            let c = p(iu1, iv + 1);
            let d = p(iu, iv + 1);
            if iv == 0 {
                m.push_triangle(a, c, d); // top cap triangle
            } else if iv == v_segs - 1 {
                m.push_triangle(a, b, c); // bottom cap triangle
            } else {
                m.push_triangle(a, b, c);
                m.push_triangle(a, c, d);
            }
        }
    }
    let mut s = finalize(m);
    s.add_hint(Surface::Sphere {
        placement: Ax3::identity(),
        radius: r,
    });
    s
}

/// `BRepPrimAPI_MakeTorus` — major radius `major` (ring), minor radius `minor`
/// (tube), centered at origin, ring in the XY plane.
// occt: BRepPrimAPI_MakeTorus
pub fn make_torus(major: f64, minor: f64, mp: MeshParams) -> Solid {
    let u_segs = mp.segs_for(major); // around Z (ring)
    let v_segs = mp.segs_for(minor).max(8); // around tube
    let p = |iu: usize, iv: usize| {
        let u = 2.0 * PI * iu as f64 / u_segs as f64;
        let v = 2.0 * PI * iv as f64 / v_segs as f64;
        let rr = major + minor * v.cos();
        Pnt::new(rr * u.cos(), rr * u.sin(), minor * v.sin())
    };
    let mut m = TriMesh::new();
    for iu in 0..u_segs {
        for iv in 0..v_segs {
            let iu1 = (iu + 1) % u_segs;
            let iv1 = (iv + 1) % v_segs;
            let a = p(iu, iv);
            let b = p(iu1, iv);
            let c = p(iu1, iv1);
            let d = p(iu, iv1);
            m.push_triangle(a, b, c);
            m.push_triangle(a, c, d);
        }
    }
    let mut s = finalize(m);
    s.add_hint(Surface::Torus {
        placement: Ax3::identity(),
        major,
        minor,
    });
    s
}

/// `BRepPrimAPI_MakePrism` — linear extrude of a planar `face` along `vec`.
// occt: BRepPrimAPI_MakePrism
pub fn make_prism(face: &Face, vec: Pnt) -> Solid {
    let n = face.normal();
    let dir = vec;
    let mut m = TriMesh::new();

    // Caps from the face triangulation.
    let cap = face.triangulate();
    let flip_caps = dir.dot(n) < 0.0;
    for t in &cap {
        // Base cap: outward normal opposite to `dir` -> reverse winding.
        let (a, b, c) = (t[0], t[1], t[2]);
        if flip_caps {
            m.push_triangle(a, b, c);
        } else {
            m.push_triangle(a, c, b);
        }
        // Top cap, translated by dir.
        let (ta, tb, tc) = (a + dir, b + dir, c + dir);
        if flip_caps {
            m.push_triangle(ta, tc, tb);
        } else {
            m.push_triangle(ta, tb, tc);
        }
    }

    // Side walls from outer + hole loops.
    let mut walls = |loop_pts: &[Pnt]| {
        let k = loop_pts.len();
        for i in 0..k {
            let a = loop_pts[i];
            let b = loop_pts[(i + 1) % k];
            let (ad, bd) = (a + dir, b + dir);
            m.push_triangle(a, b, bd);
            m.push_triangle(a, bd, ad);
        }
    };
    walls(&face.outer.pts);
    for h in &face.holes {
        walls(&h.pts);
    }
    finalize(m)
}

/// `BRepPrimAPI_MakeRevol` — revolve a closed planar profile `wire` about
/// `axis` through `angle` radians (use `2*PI` for a full lathe).
// occt: BRepPrimAPI_MakeRevol
pub fn make_revol(wire: &Wire, axis: Ax1, angle: f64, mp: MeshParams) -> Solid {
    if !wire.is_valid() {
        return Solid::empty();
    }
    let full = (angle.abs() - 2.0 * PI).abs() < 1e-6 || angle.abs() >= 2.0 * PI;
    let segs = mp.segs_for(axis_max_radius(wire, &axis)).max(8);
    let step = angle / segs as f64;
    let rings: Vec<Vec<Pnt>> = (0..=segs)
        .map(|s| {
            let t = Trsf::rotation(axis, step * s as f64);
            wire.pts.iter().map(|p| t.apply_point(*p)).collect()
        })
        .collect();

    let mut m = TriMesh::new();
    let n = wire.pts.len();
    // `rings` has segs+1 entries; for a full turn ring[segs] coincides with
    // ring[0], so connecting consecutive rings 0..segs closes the seam without
    // any wraparound special-case.
    for s in 0..segs {
        let r0 = &rings[s];
        let r1 = &rings[s + 1];
        for i in 0..n {
            let j = (i + 1) % n;
            let (a, b, c, d) = (r0[i], r0[j], r1[j], r1[i]);
            m.push_triangle(a, b, c);
            m.push_triangle(a, c, d);
        }
    }

    // For a partial revolution, cap the two end faces with the profile.
    if !full {
        let start = Face::new(Wire::new(rings[0].clone()));
        for t in start.triangulate() {
            m.push_triangle(t[0], t[2], t[1]);
        }
        let end = Face::new(Wire::new(rings[segs].clone()));
        for t in end.triangulate() {
            m.push_triangle(t[0], t[1], t[2]);
        }
    }
    finalize(m)
}

/// Max distance from any profile point to the revolution axis (controls how
/// many facets the revolution needs to look smooth).
fn axis_max_radius(wire: &Wire, axis: &Ax1) -> f64 {
    let d = axis.direction.normalized();
    let mut maxr: f64 = 0.0;
    for p in &wire.pts {
        let rel = *p - axis.location;
        let along = rel.dot(d);
        let perp = rel - d * along;
        maxr = maxr.max(perp.norm());
    }
    maxr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_volume() {
        let s = make_box(Pnt::origin(), 2.0, 3.0, 4.0);
        assert!((s.volume() - 24.0).abs() < 1e-9, "vol={}", s.volume());
        assert_eq!(s.mesh().open_edge_count(1e-6), 0);
    }

    #[test]
    fn cylinder_volume_close_to_analytic() {
        let s = make_cylinder(1.0, 2.0, MeshParams::default());
        let exact = PI * 1.0 * 1.0 * 2.0;
        let ratio = s.volume() / exact;
        assert!(ratio > 0.99 && ratio <= 1.0, "ratio={ratio}");
        assert_eq!(s.mesh().open_edge_count(1e-6), 0, "cylinder watertight");
    }

    #[test]
    fn sphere_volume_close_to_analytic() {
        let s = make_sphere(1.0, MeshParams::default());
        let exact = 4.0 / 3.0 * PI;
        let ratio = s.volume() / exact;
        assert!(ratio > 0.99 && ratio <= 1.0, "ratio={ratio}");
    }

    #[test]
    fn prism_of_square_is_box() {
        let w = Wire::new(vec![
            Pnt::new(0.0, 0.0, 0.0),
            Pnt::new(2.0, 0.0, 0.0),
            Pnt::new(2.0, 2.0, 0.0),
            Pnt::new(0.0, 2.0, 0.0),
        ]);
        let f = Face::new(w);
        let s = make_prism(&f, Pnt::new(0.0, 0.0, 5.0));
        assert!((s.volume() - 20.0).abs() < 1e-9, "vol={}", s.volume());
        assert_eq!(s.mesh().open_edge_count(1e-6), 0);
    }

    #[test]
    fn revolve_rectangle_makes_annulus() {
        // Rectangle from x in [1,2], z in [0,1], revolved about Z -> tube,
        // volume = pi*(2^2 - 1^2)*1 = 3*pi.
        let w = Wire::new(vec![
            Pnt::new(1.0, 0.0, 0.0),
            Pnt::new(2.0, 0.0, 0.0),
            Pnt::new(2.0, 0.0, 1.0),
            Pnt::new(1.0, 0.0, 1.0),
        ]);
        let s = make_revol(&w, Ax1::oz(), 2.0 * PI, MeshParams::default());
        let exact = PI * (4.0 - 1.0) * 1.0;
        let ratio = s.volume() / exact;
        assert!(
            ratio > 0.99 && ratio <= 1.001,
            "ratio={ratio} vol={}",
            s.volume()
        );
    }
}
