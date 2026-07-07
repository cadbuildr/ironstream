// FILE: iges_select_dumper.rs
// occt: IGESSelect_Dumper

pub struct IGESSelectDumper;

impl IGESSelectDumper {
    pub fn new() -> Self {
        IGESSelectDumper
    }
}

impl Default for IGESSelectDumper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = IGESSelectDumper::new();
    }
}
