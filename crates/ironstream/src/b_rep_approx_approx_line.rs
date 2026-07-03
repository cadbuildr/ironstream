// FILE: b_rep_approx_approx_line.rs
// occt: BRepApprox_ApproxLine

pub struct BRepApproxApproxLine {
    tolerance: f64,
}

impl BRepApproxApproxLine {
    pub fn new(tol: f64) -> Self {
        BRepApproxApproxLine { tolerance: tol }
    }

    pub fn tolerance(&self) -> f64 {
        self.tolerance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let al = BRepApproxApproxLine::new(1e-7);
        assert_eq!(al.tolerance(), 1e-7);
    }
}
