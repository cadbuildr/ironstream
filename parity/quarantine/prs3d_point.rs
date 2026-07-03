// FILE: prs3d_point.rs
// occt: Prs3d_Point

/// Stub for Prs3d_Point from OCCT.
#[derive(Clone, Debug)]
pub struct Prs3d_Point {}

impl Prs3d_Point {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Prs3d_Point {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prs3d_point_creation() {
        let _obj = Prs3d_Point::new();
        let _def = Prs3d_Point::default();
    }
}
