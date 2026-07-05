// FILE: iges_to_b_rep_topo_curve.rs
// occt: IGESToBRep_TopoCurve

#[derive(Default, Clone, Debug)]
pub struct IgesToBRepTopoCurve;

impl IgesToBRepTopoCurve {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _curve = IgesToBRepTopoCurve::new();
    }
}
