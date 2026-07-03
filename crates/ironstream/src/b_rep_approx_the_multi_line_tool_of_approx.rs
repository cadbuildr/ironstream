// FILE: b_rep_approx_the_multi_line_tool_of_approx.rs
// occt: BRepApprox_TheMultiLineToolOfApprox

pub struct BrepapproxThemultilinetoolofapprox;

impl BrepapproxThemultilinetoolofapprox {
    pub fn new() -> Self {
        BrepapproxThemultilinetoolofapprox
    }
}

impl Default for BrepapproxThemultilinetoolofapprox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = BrepapproxThemultilinetoolofapprox::new();
    }
}
