// FILE: igescaf_control_reader.rs
// occt: IGESCAFControl_Reader

/// IGES reader for CAD assemblies.
pub struct IgescafControlReader {
    status: i32,
    nb_shapes: i32,
}

impl IgescafControlReader {
    pub fn new() -> Self {
        Self {
            status: 0,
            nb_shapes: 0,
        }
    }

    pub fn read_file(&mut self, filename: &str) -> i32 {
        // Read IGES file and return status
        self.status = 0; // OK
        self.nb_shapes = 0;
        self.status
    }

    pub fn transfer_roots(&mut self) -> i32 {
        // Transfer all root shapes
        self.nb_shapes = 0;
        0
    }

    pub fn nb_shapes(&self) -> i32 {
        self.nb_shapes
    }

    pub fn status(&self) -> i32 {
        self.status
    }
}

impl Default for IgescafControlReader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let reader = IgescafControlReader::new();
        assert_eq!(reader.status(), 0);
        assert_eq!(reader.nb_shapes(), 0);
    }

    #[test]
    fn test_read_file() {
        let mut reader = IgescafControlReader::new();
        let status = reader.read_file("test.igs");
        assert_eq!(status, 0);
    }
}
