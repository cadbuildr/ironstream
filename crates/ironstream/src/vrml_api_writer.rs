// FILE: vrml_api_writer.rs
// occt: VrmlAPI_Writer

#[derive(Clone, Debug)]
pub struct VrmlApiWriter {
    output_file: String,
}

impl VrmlApiWriter {
    pub fn new(output_file: &str) -> Self {
        VrmlApiWriter {
            output_file: output_file.to_string(),
        }
    }

    pub fn output_file(&self) -> &str {
        &self.output_file
    }

    pub fn write(&self) -> bool {
        !self.output_file.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let writer = VrmlApiWriter::new("output.wrl");
        assert_eq!(writer.output_file(), "output.wrl");
    }

    #[test]
    fn test_write() {
        let writer = VrmlApiWriter::new("output.wrl");
        assert!(writer.write());
    }
}
