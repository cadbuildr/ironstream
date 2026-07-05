// FILE: iges_to_b_rep_reader.rs
// occt: IGESToBRep_Reader

/// Reader for converting IGES files to B-Rep format
#[derive(Default, Clone, Debug)]
pub struct IgesToBRepReader;

impl IgesToBRepReader {
    /// Creates a new Reader
    pub fn new() -> Self {
        Self
    }

    /// Reads an IGES file
    pub fn read_file(_filename: &str) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let _reader = IgesToBRepReader::new();
    }
}
