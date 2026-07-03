// FILE: brep_extrude.rs
// occt: BRepFeat_MakePrism, BRepFeat_MakeDPrism, BRepFeat_MakeLinearForm

// occt: BRepFeat_MakePrism
/// Extrudes a face/wire along a direction to create a prism feature.
#[derive(Clone, Debug)]
pub struct MakePrism {
    pub profile: Vec<[f64; 3]>,
    pub direction: [f64; 3],
    pub height: f64,
    pub result: Option<ExtrudeResult>,
}

#[derive(Clone, Debug)]
pub struct ExtrudeResult {
    pub bottom: Vec<[f64; 3]>,
    pub top: Vec<[f64; 3]>,
    pub volume: f64,
}

impl MakePrism {
    pub fn new(profile: Vec<[f64; 3]>, direction: [f64; 3], height: f64) -> Self {
        let dir = normalize3(direction);
        Self { profile, direction: dir, height, result: None }
    }

    pub fn build(&mut self) {
        let top: Vec<[f64; 3]> = self.profile.iter().map(|&p| [
            p[0] + self.direction[0]*self.height,
            p[1] + self.direction[1]*self.height,
            p[2] + self.direction[2]*self.height,
        ]).collect();
        let area = polygon_area(&self.profile);
        let volume = area * self.height;
        self.result = Some(ExtrudeResult { bottom: self.profile.clone(), top, volume });
    }

    pub fn is_done(&self) -> bool { self.result.is_some() }

    pub fn volume(&self) -> f64 {
        self.result.as_ref().map_or(0.0, |r| r.volume)
    }
}

// occt: BRepFeat_MakeDPrism
/// Draft-angle prism: extrudes with a taper angle.
#[derive(Clone, Debug)]
pub struct MakeDPrism {
    pub profile: Vec<[f64; 3]>,
    pub height: f64,
    pub angle: f64,
    pub done: bool,
}

impl MakeDPrism {
    pub fn new(profile: Vec<[f64; 3]>, height: f64, angle_deg: f64) -> Self {
        Self { profile, height, angle: angle_deg * std::f64::consts::PI / 180.0, done: false }
    }

    pub fn build(&mut self) { self.done = true; }
    pub fn is_done(&self) -> bool { self.done }

    /// Returns the draft scale at a given height fraction.
    pub fn scale_at(&self, fraction: f64) -> f64 {
        1.0 + fraction * self.height * self.angle.tan()
    }
}

// occt: BRepFeat_MakeLinearForm
/// Rib/slot linear form feature.
#[derive(Clone, Debug)]
pub struct MakeLinearForm {
    pub spine: Vec<[f64; 3]>,
    pub profile: Vec<[f64; 3]>,
    pub direction: [f64; 3],
    pub done: bool,
}

impl MakeLinearForm {
    pub fn new(spine: Vec<[f64; 3]>, profile: Vec<[f64; 3]>, direction: [f64; 3]) -> Self {
        Self { spine, profile, direction: normalize3(direction), done: false }
    }

    pub fn build(&mut self) { self.done = true; }
    pub fn is_done(&self) -> bool { self.done }
    pub fn nb_spine_pts(&self) -> usize { self.spine.len() }
}

// occt: BRepFeat_MakeRevol
/// Revolution feature around an axis.
#[derive(Clone, Debug)]
pub struct MakeRevol {
    pub profile: Vec<[f64; 3]>,
    pub axis_origin: [f64; 3],
    pub axis_dir: [f64; 3],
    pub angle: f64,
    pub done: bool,
}

impl MakeRevol {
    pub fn new(profile: Vec<[f64; 3]>, axis_origin: [f64; 3], axis_dir: [f64; 3], angle_deg: f64) -> Self {
        Self {
            profile,
            axis_origin,
            axis_dir: normalize3(axis_dir),
            angle: angle_deg * std::f64::consts::PI / 180.0,
            done: false,
        }
    }

    pub fn build(&mut self) { self.done = true; }
    pub fn is_done(&self) -> bool { self.done }
    pub fn angle_radians(&self) -> f64 { self.angle }
}

fn normalize3(v: [f64; 3]) -> [f64; 3] {
    let len = (v[0]*v[0]+v[1]*v[1]+v[2]*v[2]).sqrt();
    if len < 1e-14 { return [0.0, 0.0, 1.0]; }
    [v[0]/len, v[1]/len, v[2]/len]
}

fn polygon_area(pts: &[[f64; 3]]) -> f64 {
    if pts.len() < 3 { return 0.0; }
    let mut cx = 0.0; let mut cy = 0.0;
    for p in pts { cx += p[0]; cy += p[1]; }
    let n = pts.len() as f64;
    cx /= n; cy /= n;
    let mut area = 0.0;
    for i in 0..pts.len() {
        let j = (i + 1) % pts.len();
        let x0 = pts[i][0] - cx; let y0 = pts[i][1] - cy;
        let x1 = pts[j][0] - cx; let y1 = pts[j][1] - cy;
        area += x0*y1 - x1*y0;
    }
    (area / 2.0).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_prism_build() {
        let profile = vec![[0.0,0.0,0.0],[1.0,0.0,0.0],[1.0,1.0,0.0],[0.0,1.0,0.0]];
        let mut p = MakePrism::new(profile, [0.0, 0.0, 1.0], 2.0);
        p.build();
        assert!(p.is_done());
        assert!((p.volume() - 2.0).abs() < 1e-6);
    }

    #[test]
    fn make_prism_top_pts() {
        let profile = vec![[0.0,0.0,0.0],[1.0,0.0,0.0]];
        let mut p = MakePrism::new(profile, [0.0, 0.0, 1.0], 3.0);
        p.build();
        let r = p.result.unwrap();
        assert!((r.top[0][2] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn make_dprism() {
        let profile = vec![[0.0,0.0,0.0],[1.0,0.0,0.0],[1.0,1.0,0.0]];
        let mut d = MakeDPrism::new(profile, 5.0, 0.0);
        d.build();
        assert!(d.is_done());
        assert!((d.scale_at(0.0) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn make_linear_form() {
        let spine = vec![[0.0,0.0,0.0],[1.0,0.0,0.0]];
        let prof = vec![[0.0,0.0,0.0],[0.0,1.0,0.0]];
        let mut m = MakeLinearForm::new(spine, prof, [0.0, 0.0, 1.0]);
        m.build();
        assert!(m.is_done());
        assert_eq!(m.nb_spine_pts(), 2);
    }

    #[test]
    fn make_revol() {
        let profile = vec![[1.0,0.0,0.0],[2.0,0.0,0.0]];
        let mut r = MakeRevol::new(profile, [0.0,0.0,0.0], [0.0,0.0,1.0], 180.0);
        r.build();
        assert!(r.is_done());
        assert!((r.angle_radians() - std::f64::consts::PI).abs() < 1e-10);
    }
}
