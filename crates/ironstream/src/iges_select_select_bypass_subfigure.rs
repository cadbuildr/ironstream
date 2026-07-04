// FILE: iges_select_select_bypass_subfigure.rs
// occt: IGESSelect_SelectBypassSubfigure

pub struct IGESSelectSelectBypassSubfigure;

impl IGESSelectSelectBypassSubfigure {
    pub fn new() -> Self {
        IGESSelectSelectBypassSubfigure
    }
}

impl Default for IGESSelectSelectBypassSubfigure {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = IGESSelectSelectBypassSubfigure::new();
    }
}
