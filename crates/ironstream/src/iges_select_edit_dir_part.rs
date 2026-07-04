// FILE: iges_select_edit_dir_part.rs
// occt: IGESSelect_EditDirPart

pub struct IGESSelectEditDirPart;

impl IGESSelectEditDirPart {
    pub fn new() -> Self {
        IGESSelectEditDirPart
    }
}

impl Default for IGESSelectEditDirPart {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = IGESSelectEditDirPart::new();
    }
}
