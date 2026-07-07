// FILE: iges_select_edit_header.rs
// occt: IGESSelect_EditHeader

pub struct IGESSelectEditHeader;

impl IGESSelectEditHeader {
    pub fn new() -> Self {
        IGESSelectEditHeader
    }
}

impl Default for IGESSelectEditHeader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = IGESSelectEditHeader::new();
    }
}
