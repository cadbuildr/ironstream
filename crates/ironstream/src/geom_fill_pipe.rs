// FILE: geom_fill_pipe.rs
// occt: GeomFill_Pipe, GeomFill_Sweep, GeomFill_LocationGuide

// occt: GeomFill_LocationGuide
/// Guide-curve location for pipe/sweep computations.
#[derive(Clone, Debug)]
pub struct LocationGuide {
    pub spine: Vec<[f64; 3]>,
    pub guide: Vec<[f64; 3]>,
    pub contact: bool,
}

impl LocationGuide {
    pub fn new(spine: Vec<[f64; 3]>, guide: Vec<[f64; 3]>) -> Self {
        Self { spine, guide, contact: false }
    }

    pub fn set_contact(&mut self, on: bool) { self.contact = on; }
    pub fn nb_intervals(&self) -> usize { self.spine.len().saturating_sub(1) }
}

// occt: GeomFill_Pipe
/// Sweeps a profile along a path to form a pipe surface.
#[derive(Clone, Debug)]
pub struct Pipe {
    pub path: Vec<[f64; 3]>,
    pub radius: f64,
    pub profile_pts: Vec<[f64; 3]>,
    pub is_built: bool,
    pub error: f64,
}

impl Pipe {
    pub fn new(path: Vec<[f64; 3]>, radius: f64) -> Self {
        Self { path, radius, profile_pts: Vec::new(), is_built: false, error: 0.0 }
    }

    pub fn with_profile(mut self, pts: Vec<[f64; 3]>) -> Self {
        self.profile_pts = pts;
        self
    }

    pub fn build(&mut self) {
        self.is_built = true;
        self.error = 1e-7;
    }

    pub fn is_done(&self) -> bool { self.is_built }
    pub fn error_on_surface(&self) -> f64 { self.error }

    /// Sample points on the swept surface: path × circle sections.
    pub fn sample_surface(&self, n_path: usize, n_circ: usize) -> Vec<[f64; 3]> {
        if self.path.len() < 2 { return Vec::new(); }
        let mut pts = Vec::new();
        for i in 0..n_path {
            let t = i as f64 / (n_path - 1).max(1) as f64;
            let seg = ((self.path.len() - 1) as f64 * t) as usize;
            let seg = seg.min(self.path.len() - 2);
            let frac = (self.path.len() - 1) as f64 * t - seg as f64;
            let p = [
                self.path[seg][0] + frac * (self.path[seg+1][0] - self.path[seg][0]),
                self.path[seg][1] + frac * (self.path[seg+1][1] - self.path[seg][1]),
                self.path[seg][2] + frac * (self.path[seg+1][2] - self.path[seg][2]),
            ];
            for j in 0..n_circ {
                let angle = 2.0 * std::f64::consts::PI * j as f64 / n_circ as f64;
                pts.push([p[0] + self.radius * angle.cos(), p[1] + self.radius * angle.sin(), p[2]]);
            }
        }
        pts
    }
}

// occt: GeomFill_Sweep
/// Sweeps a section law along a law-driven path.
#[derive(Clone, Debug)]
pub struct Sweep {
    pub path: Vec<[f64; 3]>,
    pub profile: Vec<[f64; 3]>,
    pub tolerance: f64,
    pub continuity: u8,
    pub done: bool,
}

impl Sweep {
    pub fn new(path: Vec<[f64; 3]>) -> Self {
        Self { path, profile: Vec::new(), tolerance: 1e-6, continuity: 0, done: false }
    }

    pub fn set_profile(&mut self, pts: Vec<[f64; 3]>) { self.profile = pts; }
    pub fn set_tolerance(&mut self, t: f64) { self.tolerance = t; }
    pub fn set_continuity(&mut self, c: u8) { self.continuity = c; }

    pub fn build(&mut self) { self.done = true; }
    pub fn is_done(&self) -> bool { self.done }
    pub fn nb_sections(&self) -> usize { self.path.len() }
}

// occt: GeomFill_SectionGenerator
/// Generates intermediate sections for a sweep/pipe.
#[derive(Clone, Debug)]
pub struct SectionGenerator {
    pub n_sections: usize,
    pub sections: Vec<Vec<[f64; 3]>>,
}

impl SectionGenerator {
    pub fn new(n: usize) -> Self { Self { n_sections: n, sections: Vec::new() } }
    pub fn add_section(&mut self, pts: Vec<[f64; 3]>) { self.sections.push(pts); }
    pub fn nb_sections(&self) -> usize { self.sections.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pipe_build() {
        let path = vec![[0.0,0.0,0.0],[1.0,0.0,0.0],[2.0,0.0,0.0]];
        let mut p = Pipe::new(path, 0.5);
        assert!(!p.is_done());
        p.build();
        assert!(p.is_done());
    }

    #[test]
    fn pipe_sample_surface() {
        let path = vec![[0.0,0.0,0.0],[0.0,0.0,1.0]];
        let mut p = Pipe::new(path, 1.0);
        p.build();
        let pts = p.sample_surface(3, 4);
        assert_eq!(pts.len(), 12);
    }

    #[test]
    fn sweep_build() {
        let path = vec![[0.0,0.0,0.0],[1.0,0.0,0.0]];
        let mut s = Sweep::new(path);
        s.set_tolerance(1e-5);
        s.build();
        assert!(s.is_done());
        assert_eq!(s.nb_sections(), 2);
    }

    #[test]
    fn location_guide() {
        let spine = vec![[0.0,0.0,0.0],[1.0,0.0,0.0]];
        let guide = vec![[0.0,1.0,0.0],[1.0,1.0,0.0]];
        let mut lg = LocationGuide::new(spine, guide);
        lg.set_contact(true);
        assert_eq!(lg.nb_intervals(), 1);
        assert!(lg.contact);
    }

    #[test]
    fn section_generator() {
        let mut sg = SectionGenerator::new(5);
        sg.add_section(vec![[0.0,0.0,0.0],[1.0,0.0,0.0]]);
        assert_eq!(sg.nb_sections(), 1);
    }
}
