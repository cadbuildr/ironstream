// FILE: prs3d_o.rs
// occt: Prs3d

pub mod prs3d {
    pub const DEFAULT_DEFLECTION_COEFFICIENT: f64 = 0.001;
}

#[cfg(test)]
mod tests {
    use super::prs3d::*;
    #[test]
    fn test_const() { assert!(DEFAULT_DEFLECTION_COEFFICIENT > 0.0); }
}
