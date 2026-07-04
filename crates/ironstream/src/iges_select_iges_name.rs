// FILE: iges_select_iges_name.rs
// occt: IGESSelect_IGESName

pub struct IGESSelectIGESName;

impl IGESSelectIGESName {
    pub fn new() -> Self {
        IGESSelectIGESName
    }
}

impl Default for IGESSelectIGESName {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = IGESSelectIGESName::new();
    }
}
