// FILE: iges_to_b_rep_iges_boundary.rs
// occt: IGESToBRep_IGESBoundary

#[derive(Default, Clone, Debug)]
pub struct IgesToBRepIgesBoundary;

impl IgesToBRepIgesBoundary {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _boundary = IgesToBRepIgesBoundary::new();
    }
}
