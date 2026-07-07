// FILE: iges_to_b_rep_basic_surface.rs
// occt: IGESToBRep_BasicSurface

#[derive(Default, Clone, Debug)]
pub struct IgesToBRepBasicSurface;

impl IgesToBRepBasicSurface {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _surface = IgesToBRepBasicSurface::new();
    }
}
