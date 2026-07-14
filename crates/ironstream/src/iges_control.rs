// FILE: iges_control.rs

// occt-ref: IGESControl

/// Status returned by [`IgesControlReader::read_file`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IgesReadStatus {
    Done,
    Error,
    Void,
    Fail,
}

// occt-ref: IGESControl_WriteMode
/// Write mode controlling how shapes are encoded in the output IGES file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IgesWriteMode {
    BRep,
    BrewithCurves,
    NoBrep,
}

// occt-ref: IGESControl_Reader
/// Stub IGES file reader.
pub struct IgesControlReader {
    nb_roots: u32,
    loaded: bool,
    filename: Option<String>,
    failed_entities: u32,
}

impl IgesControlReader {
    pub fn new() -> Self {
        Self {
            nb_roots: 0,
            loaded: false,
            filename: None,
            failed_entities: 0,
        }
    }

    /// Stub: marks the reader as loaded and records the path.
    pub fn read_file(&mut self, path: &str) -> IgesReadStatus {
        self.filename = Some(path.to_string());
        self.loaded = true;
        self.nb_roots = 0;
        self.failed_entities = 0;
        IgesReadStatus::Done
    }

    /// Reset the set of transferred shapes without unloading the file.
    pub fn clear_shapes(&mut self) {
        self.nb_roots = 0;
    }

    /// Stub: transfers all roots and returns their count.
    pub fn transfer_roots(&mut self) -> u32 {
        self.nb_roots
    }

    /// Number of entities available for transfer.
    pub fn nb_roots_for_transfer(&self) -> u32 {
        self.nb_roots
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    pub fn nb_failed_entities(&self) -> u32 {
        self.failed_entities
    }

    pub fn filename(&self) -> Option<&str> {
        self.filename.as_deref()
    }
}

impl Default for IgesControlReader {
    fn default() -> Self {
        Self::new()
    }
}

// occt-ref: IGESControl_Writer
/// Stub IGES file writer.
pub struct IgesControlWriter {
    write_mode: IgesWriteMode,
    nb_shapes: u32,
}

impl IgesControlWriter {
    pub fn new() -> Self {
        Self {
            write_mode: IgesWriteMode::BRep,
            nb_shapes: 0,
        }
    }

    pub fn new_with_mode(mode: IgesWriteMode) -> Self {
        Self {
            write_mode: mode,
            nb_shapes: 0,
        }
    }

    pub fn write_mode(&self) -> IgesWriteMode {
        self.write_mode
    }

    /// Registers one shape to be written; increments the internal counter.
    pub fn add_shape(&mut self) {
        self.nb_shapes += 1;
    }

    /// Stub: always succeeds.
    pub fn write(&self, _path: &str) -> bool {
        true
    }

    pub fn nb_shapes(&self) -> u32 {
        self.nb_shapes
    }
}

impl Default for IgesControlWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reader_new_is_not_loaded() {
        let r = IgesControlReader::new();
        assert!(!r.is_loaded());
        assert!(r.filename().is_none());
    }

    #[test]
    fn reader_read_file_returns_done() {
        let mut r = IgesControlReader::new();
        assert_eq!(r.read_file("part.igs"), IgesReadStatus::Done);
    }

    #[test]
    fn reader_is_loaded_after_read() {
        let mut r = IgesControlReader::new();
        r.read_file("part.igs");
        assert!(r.is_loaded());
    }

    #[test]
    fn reader_filename_stored_after_read() {
        let mut r = IgesControlReader::new();
        r.read_file("assembly.igs");
        assert_eq!(r.filename(), Some("assembly.igs"));
    }

    #[test]
    fn reader_nb_failed_entities_zero_after_read() {
        let mut r = IgesControlReader::new();
        r.read_file("part.igs");
        assert_eq!(r.nb_failed_entities(), 0);
    }

    #[test]
    fn reader_clear_shapes_resets_nb_roots() {
        let mut r = IgesControlReader::new();
        r.read_file("part.igs");
        r.clear_shapes();
        assert_eq!(r.nb_roots_for_transfer(), 0);
    }

    #[test]
    fn reader_transfer_roots_returns_zero_for_stub() {
        let mut r = IgesControlReader::new();
        r.read_file("part.igs");
        assert_eq!(r.transfer_roots(), 0);
    }

    #[test]
    fn writer_new_defaults_to_brep_mode() {
        let w = IgesControlWriter::new();
        assert_eq!(w.write_mode(), IgesWriteMode::BRep);
    }

    #[test]
    fn writer_new_with_mode_stores_mode() {
        let w = IgesControlWriter::new_with_mode(IgesWriteMode::NoBrep);
        assert_eq!(w.write_mode(), IgesWriteMode::NoBrep);
    }

    #[test]
    fn writer_add_shape_increments_count() {
        let mut w = IgesControlWriter::new();
        assert_eq!(w.nb_shapes(), 0);
        w.add_shape();
        assert_eq!(w.nb_shapes(), 1);
        w.add_shape();
        assert_eq!(w.nb_shapes(), 2);
    }

    #[test]
    fn writer_write_returns_true() {
        let w = IgesControlWriter::new();
        assert!(w.write("output.igs"));
    }

    #[test]
    fn write_mode_enum_all_variants_distinct() {
        assert_ne!(IgesWriteMode::BRep, IgesWriteMode::BrewithCurves);
        assert_ne!(IgesWriteMode::BRep, IgesWriteMode::NoBrep);
        assert_ne!(IgesWriteMode::BrewithCurves, IgesWriteMode::NoBrep);
    }

    #[test]
    fn read_status_done_differs_from_others() {
        assert_ne!(IgesReadStatus::Done, IgesReadStatus::Error);
        assert_ne!(IgesReadStatus::Done, IgesReadStatus::Void);
        assert_ne!(IgesReadStatus::Done, IgesReadStatus::Fail);
    }
}
