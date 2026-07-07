// FILE: iges_geom_read_write_module.rs
// occt: IGESGeom_ReadWriteModule

/// ReadWriteModule for IGESGeom entities.
/// Handles reading and writing of IGES geometry entities from/to files.
pub struct ReadWriteModule;

impl ReadWriteModule {
    pub fn new() -> Self {
        ReadWriteModule
    }
}

impl Default for ReadWriteModule {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let _ = ReadWriteModule::new();
    }
}
