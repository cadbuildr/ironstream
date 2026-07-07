// FILE: if_select_disp_per_files.rs
// occt: IFSelect_DispPerFiles

#[derive(Clone, Debug)]
pub struct IfSelectDispPerFiles;

impl IfSelectDispPerFiles {
    pub fn new() -> Self {
        IfSelectDispPerFiles
    }
}

impl Default for IfSelectDispPerFiles {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _ = IfSelectDispPerFiles::new();
    }
}
