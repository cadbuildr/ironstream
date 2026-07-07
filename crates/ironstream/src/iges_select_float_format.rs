// FILE: iges_select_float_format.rs
// occt: IGESSelect_FloatFormat

pub struct IGESSelectFloatFormat;

impl IGESSelectFloatFormat {
    pub fn new() -> Self {
        IGESSelectFloatFormat
    }
}

impl Default for IGESSelectFloatFormat {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = IGESSelectFloatFormat::new();
    }
}
