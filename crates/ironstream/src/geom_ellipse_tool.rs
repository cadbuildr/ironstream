// FILE: geom_ellipse_tool.rs
// occt-note: GC_MakeEllipse, GC_MakeArcOfEllipse, ElSLib ellipse params

// occt-ref: GC_MakeEllipse
/// Builds an ellipse from center, major axis, major radius, minor radius.
#[derive(Clone, Debug)]
pub struct MakeEllipse {
    pub center: [f64; 3],
    pub x_dir: [f64; 3],
    pub major_radius: f64,
    pub minor_radius: f64,
    pub done: bool,
}

impl MakeEllipse {
    pub fn new(center: [f64; 3], x_dir: [f64; 3], major: f64, minor: f64) -> Self {
        let done = major >= minor && minor > 0.0;
        Self { center, x_dir: normalize3(x_dir), major_radius: major, minor_radius: minor, done }
    }

    pub fn is_done(&self) -> bool { self.done }
    pub fn eccentricity(&self) -> f64 {
        if self.major_radius < 1e-14 { return 0.0; }
        let c = (self.major_radius * self.major_radius - self.minor_radius * self.minor_radius).sqrt();
        c / self.major_radius
    }

    pub fn parameter(&self) -> f64 {
        if self.major_radius < 1e-14 { return 0.0; }
        self.minor_radius * self.minor_radius / self.major_radius
    }

    pub fn point_at(&self, t: f64) -> [f64; 3] {
        let y_dir = perp_to(self.x_dir);
        [
            self.center[0] + self.major_radius * t.cos() * self.x_dir[0] + self.minor_radius * t.sin() * y_dir[0],
            self.center[1] + self.major_radius * t.cos() * self.x_dir[1] + self.minor_radius * t.sin() * y_dir[1],
            self.center[2] + self.major_radius * t.cos() * self.x_dir[2] + self.minor_radius * t.sin() * y_dir[2],
        ]
    }

    pub fn discretize(&self, n: usize) -> Vec<[f64; 3]> {
        (0..=n).map(|i| {
            let t = 2.0 * std::f64::consts::PI * i as f64 / n as f64;
            self.point_at(t)
        }).collect()
    }

    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.major_radius * self.minor_radius
    }

    pub fn focus1(&self) -> [f64; 3] {
        let c = (self.major_radius * self.major_radius - self.minor_radius * self.minor_radius).max(0.0).sqrt();
        [self.center[0]+c*self.x_dir[0], self.center[1]+c*self.x_dir[1], self.center[2]+c*self.x_dir[2]]
    }

    pub fn focus2(&self) -> [f64; 3] {
        let c = (self.major_radius * self.major_radius - self.minor_radius * self.minor_radius).max(0.0).sqrt();
        [self.center[0]-c*self.x_dir[0], self.center[1]-c*self.x_dir[1], self.center[2]-c*self.x_dir[2]]
    }
}

// occt-ref: GC_MakeArcOfEllipse
#[derive(Clone, Debug)]
pub struct MakeArcOfEllipse {
    pub ellipse: MakeEllipse,
    pub start_angle: f64,
    pub end_angle: f64,
    pub done: bool,
}

impl MakeArcOfEllipse {
    pub fn new(center: [f64; 3], x_dir: [f64; 3], major: f64, minor: f64, start_deg: f64, end_deg: f64) -> Self {
        let pi = std::f64::consts::PI;
        let ellipse = MakeEllipse::new(center, x_dir, major, minor);
        let done = ellipse.done;
        Self { ellipse, start_angle: start_deg * pi / 180.0, end_angle: end_deg * pi / 180.0, done }
    }

    pub fn is_done(&self) -> bool { self.done }

    pub fn arc_length_approx(&self, n: usize) -> f64 {
        let pts: Vec<[f64; 3]> = (0..=n).map(|i| {
            let t = self.start_angle + (self.end_angle - self.start_angle) * i as f64 / n as f64;
            self.ellipse.point_at(t)
        }).collect();
        pts.windows(2).map(|w| {
            let dx=w[1][0]-w[0][0]; let dy=w[1][1]-w[0][1]; let dz=w[1][2]-w[0][2];
            (dx*dx+dy*dy+dz*dz).sqrt()
        }).sum()
    }

    pub fn discretize(&self, n: usize) -> Vec<[f64; 3]> {
        (0..=n).map(|i| {
            let t = self.start_angle + (self.end_angle - self.start_angle) * i as f64 / n as f64;
            self.ellipse.point_at(t)
        }).collect()
    }
}

fn normalize3(v: [f64; 3]) -> [f64; 3] {
    let len = (v[0]*v[0]+v[1]*v[1]+v[2]*v[2]).sqrt();
    if len < 1e-14 { return [1.0, 0.0, 0.0]; }
    [v[0]/len, v[1]/len, v[2]/len]
}

fn perp_to(n: [f64; 3]) -> [f64; 3] {
    // Choose a reference vector not parallel to n, then cross product to get perp.
    // Use Z-axis as reference unless n is close to Z, then use X.
    let c = if n[2].abs() < 0.9 { [0.0, 0.0, 1.0] } else { [1.0, 0.0, 0.0] };
    normalize3([n[1]*c[2]-n[2]*c[1], n[2]*c[0]-n[0]*c[2], n[0]*c[1]-n[1]*c[0]])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ellipse_params() {
        let e = MakeEllipse::new([0.0;3], [1.0,0.0,0.0], 5.0, 3.0);
        assert!(e.is_done());
        assert!((e.eccentricity() - 0.8).abs() < 1e-8);
        assert!((e.area() - std::f64::consts::PI * 15.0).abs() < 1e-8);
    }

    #[test]
    fn ellipse_foci() {
        let e = MakeEllipse::new([0.0;3], [1.0,0.0,0.0], 5.0, 3.0);
        let f1 = e.focus1();
        let f2 = e.focus2();
        // Distance from center to focus = 4.0 (sqrt(25-9) = 4)
        let d = (f1[0]*f1[0]+f1[1]*f1[1]+f1[2]*f1[2]).sqrt();
        assert!((d - 4.0).abs() < 1e-8);
        // f1 and f2 symmetric about center
        assert!((f1[0]+f2[0]).abs() < 1e-8);
    }

    #[test]
    fn ellipse_point_at() {
        let e = MakeEllipse::new([0.0;3], [1.0,0.0,0.0], 3.0, 2.0);
        let p0 = e.point_at(0.0);
        assert!((p0[0] - 3.0).abs() < 1e-8);
        let p90 = e.point_at(std::f64::consts::FRAC_PI_2);
        assert!((p90[1].abs() - 2.0).abs() < 1e-8);
    }

    #[test]
    fn ellipse_degenerate() {
        let e = MakeEllipse::new([0.0;3], [1.0,0.0,0.0], 2.0, 3.0); // minor > major
        assert!(!e.is_done());
    }

    #[test]
    fn arc_of_ellipse() {
        let arc = MakeArcOfEllipse::new([0.0;3], [1.0,0.0,0.0], 3.0, 2.0, 0.0, 90.0);
        assert!(arc.is_done());
        let len = arc.arc_length_approx(100);
        assert!(len > 3.0 && len < 6.0);
    }
}
