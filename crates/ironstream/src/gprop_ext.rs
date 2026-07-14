// FILE: gprop_ext.rs
// occt: GProp_PEquation, GProp_SelGProps
// occt-ref: GProp_GProps, GProp_PGProps

/// Kind of global properties system.
// occt-ref: GProp_GProps // (mass distribution type)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPropSystemType {
    VertexBased,
    EdgeBased,
    SurfaceBased,
    VolumeBased,
}

impl Default for GPropSystemType {
    fn default() -> Self { Self::VolumeBased }
}

/// Point cloud equation classification.
/// occt: GProp_PEquation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPropPEquationType {
    Plane,
    Line,
    Point,
    Space,    // unclassified
}

/// Global properties of a shape: mass, centroid, inertia.
// occt-ref: GProp_GProps
#[derive(Clone, Debug)]
pub struct GPropGProps {
    pub mass: f64,
    pub centroid: [f64; 3],
    /// Inertia matrix (row-major, symmetric 3×3) relative to origin.
    pub inertia: [f64; 9],
    pub system_type: GPropSystemType,
}

impl Default for GPropGProps {
    fn default() -> Self {
        Self {
            mass: 0.0,
            centroid: [0.0; 3],
            inertia: [0.0; 9],
            system_type: GPropSystemType::VolumeBased,
        }
    }
}

impl GPropGProps {
    pub fn new(system_type: GPropSystemType) -> Self {
        Self { system_type, ..Self::default() }
    }

    pub fn mass(&self) -> f64 { self.mass }
    pub fn centroid(&self) -> [f64; 3] { self.centroid }

    pub fn inertia(&self) -> [f64; 9] { self.inertia }

    /// Moments of inertia Ixx, Iyy, Izz.
    pub fn principal_moments(&self) -> [f64; 3] {
        [self.inertia[0], self.inertia[4], self.inertia[8]]
    }

    /// Static moment: mass × centroid.
    pub fn static_moments(&self) -> [f64; 3] {
        [self.mass * self.centroid[0], self.mass * self.centroid[1], self.mass * self.centroid[2]]
    }

    pub fn set_mass(&mut self, m: f64) { self.mass = m; }
    pub fn set_centroid(&mut self, c: [f64; 3]) { self.centroid = c; }
    pub fn set_inertia(&mut self, i: [f64; 9]) { self.inertia = i; }

    /// Add another properties system (accumulate).
    pub fn add(&mut self, other: &GPropGProps) {
        let total_mass = self.mass + other.mass;
        if total_mass < 1e-15 {
            self.mass = 0.0;
            return;
        }
        let w1 = self.mass / total_mass;
        let w2 = other.mass / total_mass;
        for k in 0..3 {
            self.centroid[k] = w1 * self.centroid[k] + w2 * other.centroid[k];
        }
        for k in 0..9 {
            self.inertia[k] += other.inertia[k];
        }
        self.mass = total_mass;
    }
}

/// Properties of a finite set of points: centroid and deviation.
// occt-ref: GProp_PGProps
#[derive(Clone, Debug, Default)]
pub struct GPropPGProps {
    pub points: Vec<[f64; 3]>,
    pub weights: Vec<f64>,
}

impl GPropPGProps {
    pub fn new() -> Self { Self::default() }

    pub fn add_point(&mut self, p: [f64; 3], w: f64) {
        self.points.push(p);
        self.weights.push(w.max(0.0));
    }

    pub fn centroid(&self) -> [f64; 3] {
        let total_w: f64 = self.weights.iter().sum();
        if total_w < 1e-15 { return [0.0; 3]; }
        let mut c = [0.0f64; 3];
        for (p, &w) in self.points.iter().zip(self.weights.iter()) {
            for k in 0..3 { c[k] += w * p[k]; }
        }
        for k in 0..3 { c[k] /= total_w; }
        c
    }

    pub fn total_weight(&self) -> f64 { self.weights.iter().sum() }

    pub fn nb_points(&self) -> usize { self.points.len() }

    /// Classify the point set as plane, line, point, or space.
    /// occt: GProp_PEquation
    pub fn classify(&self, tol: f64) -> GPropPEquationType {
        if self.points.len() < 2 { return GPropPEquationType::Point; }
        let c = self.centroid();
        let mut max_dist = 0.0f64;
        for p in &self.points {
            let d = [(p[0]-c[0]), (p[1]-c[1]), (p[2]-c[2])];
            let dist = (d[0]*d[0]+d[1]*d[1]+d[2]*d[2]).sqrt();
            if dist > max_dist { max_dist = dist; }
        }
        if max_dist < tol { return GPropPEquationType::Point; }
        // Approximate: compute variance in each dimension
        let mut var = [0.0f64; 3];
        for p in &self.points {
            for k in 0..3 { let v = p[k] - c[k]; var[k] += v*v; }
        }
        let n = self.points.len() as f64;
        for k in 0..3 { var[k] /= n; }
        let mut nonzero = var.iter().filter(|&&v| v.sqrt() > tol).count();
        match nonzero {
            0 => GPropPEquationType::Point,
            1 => GPropPEquationType::Line,
            2 => GPropPEquationType::Plane,
            _ => GPropPEquationType::Space,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gprops_defaults() {
        let g = GPropGProps::default();
        assert!((g.mass() - 0.0).abs() < 1e-12);
        assert_eq!(g.centroid(), [0.0; 3]);
    }

    #[test]
    fn gprops_add() {
        let mut g1 = GPropGProps::new(GPropSystemType::VolumeBased);
        g1.set_mass(2.0);
        g1.set_centroid([1.0, 0.0, 0.0]);
        let mut g2 = GPropGProps::new(GPropSystemType::VolumeBased);
        g2.set_mass(2.0);
        g2.set_centroid([3.0, 0.0, 0.0]);
        g1.add(&g2);
        assert!((g1.mass() - 4.0).abs() < 1e-10);
        assert!((g1.centroid()[0] - 2.0).abs() < 1e-10);
    }

    #[test]
    fn gprops_static_moments() {
        let mut g = GPropGProps::default();
        g.set_mass(3.0);
        g.set_centroid([2.0, 1.0, 0.5]);
        let sm = g.static_moments();
        assert!((sm[0] - 6.0).abs() < 1e-10);
        assert!((sm[1] - 3.0).abs() < 1e-10);
    }

    #[test]
    fn pgprops_centroid() {
        let mut pg = GPropPGProps::new();
        pg.add_point([0.0, 0.0, 0.0], 1.0);
        pg.add_point([2.0, 0.0, 0.0], 1.0);
        pg.add_point([1.0, 2.0, 0.0], 2.0);
        let c = pg.centroid();
        // weighted: (0+2+2)/4=1.0, (0+0+4)/4=1.0
        assert!((c[0] - 1.0).abs() < 1e-10);
        assert!((c[1] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn pgprops_classify_plane() {
        let mut pg = GPropPGProps::new();
        pg.add_point([0.0, 0.0, 0.0], 1.0);
        pg.add_point([1.0, 0.0, 0.0], 1.0);
        pg.add_point([0.0, 1.0, 0.0], 1.0);
        pg.add_point([1.0, 1.0, 0.0], 1.0);
        assert_eq!(pg.classify(1e-6), GPropPEquationType::Plane);
    }

    #[test]
    fn pgprops_classify_point() {
        let mut pg = GPropPGProps::new();
        pg.add_point([0.0, 0.0, 0.0], 1.0);
        pg.add_point([0.0, 0.0, 0.0], 1.0);
        assert_eq!(pg.classify(1e-6), GPropPEquationType::Point);
    }
}
