// FILE: iges_data_writer_lib.rs
// occt: IGESData_WriterLib

//! Library for IGES entity writers.

#[derive(Clone, Debug)]
pub struct WriterLib {
    writers: Vec<String>,
}

impl WriterLib {
    pub fn new() -> Self {
        WriterLib {
            writers: Vec::new(),
        }
    }

    pub fn add_writer(&mut self, name: &str) {
        self.writers.push(name.to_string());
    }

    pub fn writer_count(&self) -> usize {
        self.writers.len()
    }

    pub fn writers(&self) -> &[String] {
        &self.writers
    }
}

impl Default for WriterLib {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let lib = WriterLib::new();
        assert_eq!(lib.writer_count(), 0);
    }

    #[test]
    fn test_add_writer() {
        let mut lib = WriterLib::new();
        lib.add_writer("writer1");
        lib.add_writer("writer2");
        assert_eq!(lib.writer_count(), 2);
    }

    #[test]
    fn test_writers() {
        let mut lib = WriterLib::new();
        lib.add_writer("w1");
        lib.add_writer("w2");
        let writers = lib.writers();
        assert_eq!(writers.len(), 2);
    }
}
