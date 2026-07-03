// FILE: graphic3d_clip.rs
// occt: Graphic3d_ClipPlane, Graphic3d_SequenceOfHClipPlane

/// Clipping equation: ax + by + cz + d >= 0 keeps the positive half-space.
#[derive(Clone, Debug)]
pub struct ClipPlane {
    pub equation: [f64; 4],   // [a, b, c, d]
    pub is_on: bool,
    pub is_capping: bool,
    pub capping_color: [f32; 4],
    pub capping_hatch: bool,
    pub chain_next_plane_id: Option<u32>,
}

impl ClipPlane {
    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Self {
        let len = (a*a + b*b + c*c).sqrt();
        let eq = if len > 1e-14 { [a/len, b/len, c/len, d/len] } else { [0.0, 0.0, 1.0, 0.0] };
        Self {
            equation: eq,
            is_on: true,
            is_capping: false,
            capping_color: [0.8, 0.8, 0.8, 1.0],
            capping_hatch: false,
            chain_next_plane_id: None,
        }
    }

    pub fn from_point_normal(point: [f64; 3], normal: [f64; 3]) -> Self {
        let d = -(normal[0]*point[0] + normal[1]*point[1] + normal[2]*point[2]);
        Self::new(normal[0], normal[1], normal[2], d)
    }

    pub fn normal(&self) -> [f64; 3] {
        [self.equation[0], self.equation[1], self.equation[2]]
    }

    pub fn offset(&self) -> f64 { self.equation[3] }

    pub fn set_on(&mut self, v: bool) { self.is_on = v; }
    pub fn set_capping(&mut self, v: bool) { self.is_capping = v; }

    pub fn distance_to_point(&self, pt: &[f64; 3]) -> f64 {
        self.equation[0]*pt[0] + self.equation[1]*pt[1] + self.equation[2]*pt[2] + self.equation[3]
    }

    pub fn is_point_clipped(&self, pt: &[f64; 3]) -> bool {
        self.is_on && self.distance_to_point(pt) < 0.0
    }

    pub fn with_capping_color(mut self, c: [f32; 4]) -> Self { self.capping_color = c; self }
    pub fn with_capping(mut self) -> Self { self.is_capping = true; self }

    pub fn flip(&mut self) {
        for i in 0..4 { self.equation[i] = -self.equation[i]; }
    }

    pub fn flip_copy(&self) -> Self {
        let mut c = self.clone();
        c.flip();
        c
    }
}

// occt: Graphic3d_SequenceOfHClipPlane
#[derive(Clone, Debug, Default)]
pub struct ClipPlaneSet {
    pub planes: Vec<ClipPlane>,
    pub max_planes: usize,
}

impl ClipPlaneSet {
    pub fn new() -> Self { Self { planes: Vec::new(), max_planes: 8 } }

    pub fn add(&mut self, plane: ClipPlane) -> bool {
        if self.planes.len() >= self.max_planes { return false; }
        self.planes.push(plane);
        true
    }

    pub fn remove(&mut self, idx: usize) -> Option<ClipPlane> {
        if idx < self.planes.len() { Some(self.planes.remove(idx)) } else { None }
    }

    pub fn nb_active(&self) -> usize { self.planes.iter().filter(|p| p.is_on).count() }
    pub fn nb_planes(&self) -> usize { self.planes.len() }
    pub fn is_empty(&self) -> bool { self.planes.is_empty() }

    pub fn is_point_clipped(&self, pt: [f64; 3]) -> bool {
        self.planes.iter().any(|p| p.is_point_clipped(&pt))
    }

    pub fn capping_planes(&self) -> Vec<&ClipPlane> {
        self.planes.iter().filter(|p| p.is_on && p.is_capping).collect()
    }

    pub fn set_all_on(&mut self, v: bool) {
        for p in &mut self.planes { p.is_on = v; }
    }
}

/// Axis-aligned clipping box (6 planes).
#[derive(Clone, Debug)]
pub struct ClipBox {
    pub min: [f64; 3],
    pub max: [f64; 3],
}

impl ClipBox {
    pub fn new(mn: [f64; 3], mx: [f64; 3]) -> Self { Self { min: mn, max: mx } }

    pub fn to_clip_planes(&self) -> Vec<ClipPlane> {
        vec![
            ClipPlane::new( 1.0,  0.0,  0.0, -self.min[0]),  // x >= min
            ClipPlane::new(-1.0,  0.0,  0.0,  self.max[0]),  // x <= max
            ClipPlane::new( 0.0,  1.0,  0.0, -self.min[1]),
            ClipPlane::new( 0.0, -1.0,  0.0,  self.max[1]),
            ClipPlane::new( 0.0,  0.0,  1.0, -self.min[2]),
            ClipPlane::new( 0.0,  0.0, -1.0,  self.max[2]),
        ]
    }

    pub fn contains(&self, pt: &[f64; 3]) -> bool {
        (0..3).all(|i| pt[i] >= self.min[i] && pt[i] <= self.max[i])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clip_plane_distance() {
        let p = ClipPlane::from_point_normal([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        assert!((p.distance_to_point(&[0.0, 0.0, 1.0]) - 1.0).abs() < 1e-10);
        assert!((p.distance_to_point(&[0.0, 0.0, -1.0]) + 1.0).abs() < 1e-10);
        assert!(!p.is_point_clipped(&[0.0, 0.0, 1.0]));
        assert!(p.is_point_clipped(&[0.0, 0.0, -0.1]));
    }

    #[test]
    fn clip_plane_set() {
        let mut cs = ClipPlaneSet::new();
        cs.add(ClipPlane::new(0.0, 0.0, 1.0, 0.0));
        cs.add(ClipPlane::new(0.0, 1.0, 0.0, 0.0));
        assert_eq!(cs.nb_planes(), 2);
        assert_eq!(cs.nb_active(), 2);
        assert!(cs.is_point_clipped([0.0, 0.0, -1.0]));
        assert!(!cs.is_point_clipped([1.0, 1.0, 1.0]));
    }

    #[test]
    fn clip_box() {
        let cb = ClipBox::new([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
        assert!(cb.contains(&[0.5, 0.5, 0.5]));
        assert!(!cb.contains(&[1.5, 0.5, 0.5]));
        let planes = cb.to_clip_planes();
        assert_eq!(planes.len(), 6);
    }

    #[test]
    fn plane_flip() {
        let p = ClipPlane::new(0.0, 0.0, 1.0, 0.0);
        let flipped = p.flip_copy();
        assert!((flipped.equation[2] + 1.0).abs() < 1e-10);
    }
}
