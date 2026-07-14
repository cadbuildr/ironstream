// FILE: geom_diff_geom.rs
// occt-ref: LProp_SLProps, LProp_CLProps, LProp3d_SLProps

/// First-order properties of a 3D curve at a parameter.
#[derive(Clone, Debug)]
pub struct CurveProps {
    pub point: [f64; 3],
    pub tangent: [f64; 3],
    pub normal: Option<[f64; 3]>,
    pub binormal: Option<[f64; 3]>,
    pub curvature: f64,
    pub torsion: f64,
}

impl CurveProps {
    /// Compute Frenet frame for a discrete polyline at index i.
    pub fn from_polyline(pts: &[[f64; 3]], i: usize) -> Option<Self> {
        let n = pts.len();
        if n < 2 || i >= n { return None; }
        let point = pts[i];
        // Tangent from central differences
        let tangent = if i == 0 {
            normalize3(sub3(pts[1], pts[0]))
        } else if i == n - 1 {
            normalize3(sub3(pts[n-1], pts[n-2]))
        } else {
            normalize3(sub3(pts[i+1], pts[i-1]))
        };
        // Curvature and normal
        let (normal, curvature) = if i > 0 && i < n - 1 {
            let t1 = normalize3(sub3(pts[i], pts[i-1]));
            let t2 = normalize3(sub3(pts[i+1], pts[i]));
            let dt = sub3(t2, t1);
            let h = {
                let d1 = len3(sub3(pts[i], pts[i-1]));
                let d2 = len3(sub3(pts[i+1], pts[i]));
                (d1 + d2) / 2.0
            };
            let k = len3(dt) / h.max(1e-14);
            let n_dir = if k > 1e-12 { Some(normalize3(dt)) } else { None };
            (n_dir, k)
        } else { (None, 0.0) };
        let binormal = normal.map(|n_dir| cross3(tangent, n_dir));
        Some(Self { point, tangent, normal, binormal, curvature, torsion: 0.0 })
    }

    pub fn has_normal(&self) -> bool { self.normal.is_some() }
}

// occt-note: LProp_SLProps (surface local properties)
#[derive(Clone, Debug)]
pub struct SurfaceProps {
    pub point: [f64; 3],
    pub normal: [f64; 3],
    pub u: f64,
    pub v: f64,
    pub min_curvature: f64,
    pub max_curvature: f64,
    pub mean_curvature: f64,
    pub gaussian_curvature: f64,
}

impl SurfaceProps {
    pub fn from_flat_surface(u: f64, v: f64) -> Self {
        Self {
            point: [u, v, 0.0],
            normal: [0.0, 0.0, 1.0],
            u, v,
            min_curvature: 0.0, max_curvature: 0.0,
            mean_curvature: 0.0, gaussian_curvature: 0.0,
        }
    }

    pub fn from_sphere(center: [f64; 3], radius: f64, u: f64, v: f64) -> Self {
        let x = radius * u.cos() * v.cos();
        let y = radius * u.sin() * v.cos();
        let z = radius * v.sin();
        let point = [center[0]+x, center[1]+y, center[2]+z];
        let normal = normalize3([x, y, z]);
        let k = 1.0 / radius;
        Self {
            point, normal, u, v,
            min_curvature: k, max_curvature: k,
            mean_curvature: k, gaussian_curvature: k * k,
        }
    }

    pub fn is_umbilic(&self) -> bool {
        (self.min_curvature - self.max_curvature).abs() < 1e-8
    }

    pub fn is_developable(&self) -> bool {
        self.gaussian_curvature.abs() < 1e-8
    }
}

// occt-ref: GeomLProp_SLProps
pub struct GeomSurfaceProps {
    pub props: Vec<SurfaceProps>,
}

impl GeomSurfaceProps {
    pub fn new() -> Self { Self { props: Vec::new() } }
    pub fn add(&mut self, p: SurfaceProps) { self.props.push(p); }
    pub fn nb_points(&self) -> usize { self.props.len() }
}

impl Default for GeomSurfaceProps {
    fn default() -> Self { Self::new() }
}

fn normalize3(v: [f64; 3]) -> [f64; 3] {
    let len = len3(v);
    if len < 1e-14 { return [1.0, 0.0, 0.0]; }
    [v[0]/len, v[1]/len, v[2]/len]
}

fn cross3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]
}

fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] { [a[0]-b[0], a[1]-b[1], a[2]-b[2]] }

fn len3(v: [f64; 3]) -> f64 { (v[0]*v[0]+v[1]*v[1]+v[2]*v[2]).sqrt() }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curve_props_from_polyline() {
        let pts = vec![[0.0,0.0,0.0],[1.0,0.0,0.0],[2.0,0.0,0.0]];
        let p = CurveProps::from_polyline(&pts, 1).unwrap();
        assert!((p.tangent[0] - 1.0).abs() < 1e-8);
        assert!((p.curvature).abs() < 1e-8);
    }

    #[test]
    fn curve_props_endpoints() {
        let pts = vec![[0.0,0.0,0.0],[1.0,0.0,0.0]];
        let p0 = CurveProps::from_polyline(&pts, 0).unwrap();
        assert!((p0.tangent[0] - 1.0).abs() < 1e-8);
    }

    #[test]
    fn surface_props_sphere() {
        let s = SurfaceProps::from_sphere([0.0;3], 1.0, 0.0, 0.0);
        assert!(s.is_umbilic());
        assert!((s.mean_curvature - 1.0).abs() < 1e-8);
        assert!((s.gaussian_curvature - 1.0).abs() < 1e-8);
    }

    #[test]
    fn surface_props_flat() {
        let s = SurfaceProps::from_flat_surface(0.5, 0.5);
        assert!(s.is_developable());
        assert!((s.normal[2] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn geom_surface_props_collect() {
        let mut g = GeomSurfaceProps::new();
        g.add(SurfaceProps::from_flat_surface(0.0, 0.0));
        g.add(SurfaceProps::from_flat_surface(1.0, 1.0));
        assert_eq!(g.nb_points(), 2);
    }
}
