// FILE: b_rep_approx_the_multi_line_of_approx.rs
// occt: BRepApprox_TheMultiLineOfApprox

pub struct BrepapproxThemultilineofapprox;

impl BrepapproxThemultilineofapprox {
    pub fn new() -> Self {
        BrepapproxThemultilineofapprox
    }
}

impl Default for BrepapproxThemultilineofapprox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BrepapproxThemultilineofapprox::new();
    }
}
