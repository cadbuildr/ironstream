// FILE: iges_to_b_rep_basic_curve.rs
// occt: IGESToBRep_BasicCurve

#[derive(Default, Clone, Debug)]
pub struct IgesToBRepBasicCurve;

impl IgesToBRepBasicCurve {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _curve = IgesToBRepBasicCurve::new();
    }
}
