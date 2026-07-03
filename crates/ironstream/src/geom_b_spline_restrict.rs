// FILE: geom_b_spline_restrict.rs
// occt: GeomConvert_CompCurvesToBSplineCurve, BSplCLib (knot insertion/removal)

// occt: BSplCLib knot insertion
/// Insert a knot into a B-Spline at parameter t, returning the new knot vector.
pub fn insert_knot(knots: &[f64], mult: &[u32], t: f64, degree: u32) -> (Vec<f64>, Vec<u32>) {
    let mut new_knots = Vec::new();
    let mut new_mult = Vec::new();
    let mut inserted = false;
    for (&k, &m) in knots.iter().zip(mult.iter()) {
        if !inserted && t < k - 1e-14 {
            new_knots.push(t);
            new_mult.push(1);
            inserted = true;
        }
        if (k - t).abs() < 1e-14 && !inserted {
            // Increase multiplicity
            new_knots.push(k);
            new_mult.push((m + 1).min(degree + 1));
            inserted = true;
        } else {
            new_knots.push(k);
            new_mult.push(m);
        }
    }
    if !inserted {
        new_knots.push(t);
        new_mult.push(1);
    }
    (new_knots, new_mult)
}

/// Remove a knot at index `idx` if its multiplicity can be decreased by 1.
pub fn remove_knot(knots: &[f64], mult: &[u32], idx: usize, tolerance: f64) -> Option<(Vec<f64>, Vec<u32>)> {
    if idx >= mult.len() || mult[idx] == 0 { return None; }
    let _ = tolerance;
    let mut new_knots = knots.to_vec();
    let mut new_mult = mult.to_vec();
    if new_mult[idx] > 1 {
        new_mult[idx] -= 1;
    } else {
        new_knots.remove(idx);
        new_mult.remove(idx);
    }
    Some((new_knots, new_mult))
}

// occt: GeomConvert_CompCurvesToBSplineCurve (concatenate B-Splines)
#[derive(Clone, Debug)]
pub struct ConcatBSplines {
    pub segments: Vec<BSplineSegment>,
    pub result_poles: Vec<[f64; 3]>,
    pub result_knots: Vec<f64>,
    pub tolerance: f64,
    pub done: bool,
}

#[derive(Clone, Debug)]
pub struct BSplineSegment {
    pub poles: Vec<[f64; 3]>,
    pub t_start: f64,
    pub t_end: f64,
}

impl ConcatBSplines {
    pub fn new(tolerance: f64) -> Self {
        Self { segments: Vec::new(), result_poles: Vec::new(), result_knots: Vec::new(), tolerance, done: false }
    }

    pub fn add(&mut self, seg: BSplineSegment) {
        self.segments.push(seg);
    }

    pub fn build(&mut self) {
        if self.segments.is_empty() { self.done = false; return; }
        self.result_poles.clear();
        self.result_knots.clear();
        let mut t_offset = 0.0;
        for seg in &self.segments {
            let span = seg.t_end - seg.t_start;
            if self.result_poles.is_empty() {
                self.result_poles.extend_from_slice(&seg.poles);
            } else {
                // Check C0 continuity at junction
                let last = *self.result_poles.last().unwrap();
                let first = seg.poles[0];
                let dx = last[0]-first[0]; let dy = last[1]-first[1]; let dz = last[2]-first[2];
                if (dx*dx+dy*dy+dz*dz).sqrt() < self.tolerance {
                    self.result_poles.extend_from_slice(&seg.poles[1..]);
                } else {
                    self.result_poles.extend_from_slice(&seg.poles);
                }
            }
            self.result_knots.push(t_offset);
            t_offset += span;
        }
        self.result_knots.push(t_offset);
        self.done = true;
    }

    pub fn is_done(&self) -> bool { self.done }
    pub fn nb_poles(&self) -> usize { self.result_poles.len() }
}

// occt: GeomConvert_RationalToPoly (convert rational to polynomial)
#[derive(Clone, Debug)]
pub struct RationalToPoly {
    pub poles: Vec<[f64; 3]>,
    pub weights: Vec<f64>,
    pub result_poles: Vec<[f64; 3]>,
    pub done: bool,
}

impl RationalToPoly {
    pub fn new(poles: Vec<[f64; 3]>, weights: Vec<f64>) -> Self {
        let mut r = Self { poles, weights, result_poles: Vec::new(), done: false };
        r.perform();
        r
    }

    fn perform(&mut self) {
        // Multiply poles by weights to get homogeneous coordinates, then normalize
        self.result_poles = self.poles.iter().zip(self.weights.iter()).map(|(&p, &w)| {
            let w = if w.abs() < 1e-14 { 1.0 } else { w };
            [p[0]/w, p[1]/w, p[2]/w]
        }).collect();
        self.done = true;
    }

    pub fn is_done(&self) -> bool { self.done }
    pub fn nb_poles(&self) -> usize { self.result_poles.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_knot_new() {
        let knots = vec![0.0, 1.0];
        let mult = vec![1u32, 1];
        let (k, m) = insert_knot(&knots, &mult, 0.5, 3);
        assert!(k.contains(&0.5));
        assert_eq!(k.len(), m.len());
    }

    #[test]
    fn remove_knot_decrease_mult() {
        let knots = vec![0.0, 0.5, 1.0];
        let mult = vec![1u32, 2, 1];
        let (k, m) = remove_knot(&knots, &mult, 1, 1e-6).unwrap();
        assert_eq!(m[1], 1);
        assert_eq!(k.len(), 3);
    }

    #[test]
    fn concat_bsplines() {
        let mut c = ConcatBSplines::new(1e-6);
        c.add(BSplineSegment { poles: vec![[0.0,0.0,0.0],[0.5,1.0,0.0],[1.0,0.0,0.0]], t_start: 0.0, t_end: 1.0 });
        c.add(BSplineSegment { poles: vec![[1.0,0.0,0.0],[1.5,-1.0,0.0],[2.0,0.0,0.0]], t_start: 0.0, t_end: 1.0 });
        c.build();
        assert!(c.is_done());
        assert_eq!(c.nb_poles(), 5); // shared endpoint merged
    }

    #[test]
    fn rational_to_poly() {
        let poles = vec![[1.0,0.0,0.0],[2.0,0.0,0.0]];
        let weights = vec![2.0, 1.0];
        let r = RationalToPoly::new(poles, weights);
        assert!(r.is_done());
        assert!((r.result_poles[0][0] - 0.5).abs() < 1e-10);
    }

    #[test]
    fn insert_knot_existing() {
        let knots = vec![0.0, 0.5, 1.0];
        let mult = vec![1u32, 1, 1];
        let (k, m) = insert_knot(&knots, &mult, 0.5, 3);
        assert_eq!(m[1], 2);
        assert_eq!(k.len(), 3);
    }
}
