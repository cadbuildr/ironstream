// FILE: iges_select_work_library.rs
// occt: IGESSelect_WorkLibrary

//! Work library for reading and writing IGES files.
//!
//! Provides read/write operations for IGES models and defines protocol
//! encompassing all IGES norms including IGESSolid and IGESAppli.

use std::io::{self, Write, Read};
use std::fs::{File, OpenOptions};
use std::path::Path;

/// Represents an IGES file model
pub struct IGESModel {
    entities: Vec<String>,
    nb_entities: usize,
}

impl IGESModel {
    pub fn new() -> Self {
        IGESModel {
            entities: Vec::new(),
            nb_entities: 0,
        }
    }

    pub fn add_entity(&mut self, entity: String) {
        self.entities.push(entity);
        self.nb_entities += 1;
    }

    pub fn nb_entities(&self) -> usize {
        self.nb_entities
    }

    pub fn entities(&self) -> &[String] {
        &self.entities
    }
}

/// IGES Protocol definition
pub struct IGESDataProtocol {
    name: String,
}

impl IGESDataProtocol {
    pub fn new(name: &str) -> Self {
        IGESDataProtocol {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Interface model base type
pub struct InterfaceModel {
    // Placeholder for interface model
}

/// Context for file write operations
pub struct IFSelectContextWrite {
    model: Option<IGESModel>,
    protocol: Option<IGESDataProtocol>,
    file_name: String,
    checks: Vec<String>,
    nb_modifiers: usize,
    nb_entities: usize,
}

impl IFSelectContextWrite {
    pub fn new(file_name: &str) -> Self {
        IFSelectContextWrite {
            model: None,
            protocol: None,
            file_name: file_name.to_string(),
            checks: Vec::new(),
            nb_modifiers: 0,
            nb_entities: 0,
        }
    }

    pub fn set_model(&mut self, model: IGESModel) {
        self.nb_entities = model.nb_entities();
        self.model = Some(model);
    }

    pub fn set_protocol(&mut self, protocol: IGESDataProtocol) {
        self.protocol = Some(protocol);
    }

    pub fn model(&self) -> Option<&IGESModel> {
        self.model.as_ref()
    }

    pub fn protocol(&self) -> Option<&IGESDataProtocol> {
        self.protocol.as_ref()
    }

    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    pub fn add_check(&mut self, message: String) {
        self.checks.push(message);
    }

    pub fn checks(&self) -> &[String] {
        &self.checks
    }

    pub fn set_nb_modifiers(&mut self, nb: usize) {
        self.nb_modifiers = nb;
    }

    pub fn nb_modifiers(&self) -> usize {
        self.nb_modifiers
    }

    pub fn nb_entities(&self) -> usize {
        self.nb_entities
    }
}

/// IGES Writer for serializing models
pub struct IGESWriter {
    model: IGESModel,
}

impl IGESWriter {
    pub fn new(model: IGESModel) -> Self {
        IGESWriter { model }
    }

    pub fn model(&self) -> &IGESModel {
        &self.model
    }

    /// Sends the model to protocol for processing
    pub fn send_model(&self, _protocol: &IGESDataProtocol) {
        // Placeholder: in real implementation, validates model against protocol
    }

    /// Prints/writes the model to an output stream
    pub fn print(&self, output: &mut dyn Write) -> io::Result<bool> {
        // Write IGES header
        writeln!(output, "IGES File")?;
        writeln!(output, "         {:80}", "S      1")?;

        // Write entities
        for entity in self.model.entities() {
            writeln!(output, "{}", entity)?;
        }

        // Write trailer
        writeln!(output, "S      1G      0D      0P      1")?;

        Ok(true)
    }
}

/// File protocol combining multiple IGES protocols
pub struct IGESFileProtocol {
    protocols: Vec<IGESDataProtocol>,
}

impl IGESFileProtocol {
    pub fn new() -> Self {
        IGESFileProtocol {
            protocols: Vec::new(),
        }
    }

    pub fn add_protocol(&mut self, protocol: IGESDataProtocol) {
        self.protocols.push(protocol);
    }

    pub fn protocols(&self) -> &[IGESDataProtocol] {
        &self.protocols
    }
}

/// Work library for IGES file operations
pub struct IGESSelectWorkLibrary {
    mode_fnes: bool,
    protocol: Option<IGESFileProtocol>,
}

impl IGESSelectWorkLibrary {
    /// Creates a new IGES work library
    pub fn new(mode_fnes: bool) -> Self {
        IGESSelectWorkLibrary {
            mode_fnes,
            protocol: None,
        }
    }

    /// Reads an IGES file and returns a model
    ///
    /// Returns:
    /// - 0 if OK
    /// - 1 if read error
    /// - -1 if file not opened
    pub fn read_file(
        &self,
        name: &str,
        _protocol: &IGESDataProtocol,
    ) -> (i32, Option<IGESModel>) {
        let path = Path::new(name);

        if !path.exists() {
            return (-1, None);
        }

        match File::open(path) {
            Ok(mut file) => {
                let mut contents = String::new();
                match file.read_to_string(&mut contents) {
                    Ok(_) => {
                        let mut model = IGESModel::new();
                        // Parse basic IGES structure (simplified)
                        for line in contents.lines() {
                            if !line.trim().is_empty() && !line.starts_with('S') {
                                model.add_entity(line.to_string());
                            }
                        }
                        (0, Some(model))
                    }
                    Err(_) => (1, None),
                }
            }
            Err(_) => (1, None),
        }
    }

    /// Writes an IGES file from a context
    pub fn write_file(&self, ctx: &IFSelectContextWrite) -> bool {
        if ctx.model().is_none() || ctx.protocol().is_none() {
            return false;
        }

        match OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(ctx.file_name())
        {
            Ok(mut file) => {
                if let Some(model) = ctx.model() {
                    let writer = IGESWriter::new(IGESModel {
                        entities: model.entities().to_vec(),
                        nb_entities: model.nb_entities(),
                    });

                    if let Some(protocol) = ctx.protocol() {
                        writer.send_model(protocol);
                    }

                    match writer.print(&mut file) {
                        Ok(status) => status,
                        Err(_) => false,
                    }
                } else {
                    false
                }
            }
            Err(_) => false,
        }
    }

    /// Defines a protocol encompassing all IGES norms
    pub fn define_protocol() -> IGESFileProtocol {
        let mut proto = IGESFileProtocol::new();
        proto.add_protocol(IGESDataProtocol::new("IGESSolid"));
        proto.add_protocol(IGESDataProtocol::new("IGESAppli"));
        proto
    }

    /// Returns the FNES mode setting
    pub fn mode_fnes(&self) -> bool {
        self.mode_fnes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_iges_model_creation() {
        let model = IGESModel::new();
        assert_eq!(model.nb_entities(), 0);
    }

    #[test]
    fn test_add_entity() {
        let mut model = IGESModel::new();
        model.add_entity("Entity 1".to_string());
        model.add_entity("Entity 2".to_string());

        assert_eq!(model.nb_entities(), 2);
        assert_eq!(model.entities().len(), 2);
    }

    #[test]
    fn test_iges_protocol() {
        let proto = IGESDataProtocol::new("TestProto");
        assert_eq!(proto.name(), "TestProto");
    }

    #[test]
    fn test_context_write_creation() {
        let ctx = IFSelectContextWrite::new("output.iges");
        assert_eq!(ctx.file_name(), "output.iges");
        assert!(ctx.model().is_none());
    }

    #[test]
    fn test_context_set_model() {
        let mut ctx = IFSelectContextWrite::new("output.iges");
        let mut model = IGESModel::new();
        model.add_entity("E1".to_string());
        model.add_entity("E2".to_string());

        ctx.set_model(model);
        assert_eq!(ctx.nb_entities(), 2);
    }

    #[test]
    fn test_context_set_protocol() {
        let mut ctx = IFSelectContextWrite::new("output.iges");
        let proto = IGESDataProtocol::new("IGES");
        ctx.set_protocol(proto);

        assert!(ctx.protocol().is_some());
        assert_eq!(ctx.protocol().unwrap().name(), "IGES");
    }

    #[test]
    fn test_context_checks() {
        let mut ctx = IFSelectContextWrite::new("output.iges");
        ctx.add_check("Check 1".to_string());
        ctx.add_check("Check 2".to_string());

        assert_eq!(ctx.checks().len(), 2);
    }

    #[test]
    fn test_iges_writer() {
        let model = IGESModel::new();
        let writer = IGESWriter::new(model);

        assert_eq!(writer.model().nb_entities(), 0);
    }

    #[test]
    fn test_iges_writer_print() {
        let mut model = IGESModel::new();
        model.add_entity("ENTITY".to_string());

        let writer = IGESWriter::new(model);
        let mut buffer = Vec::new();

        let result = writer.print(&mut buffer);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_work_library_creation() {
        let lib = IGESSelectWorkLibrary::new(false);
        assert!(!lib.mode_fnes());
    }

    #[test]
    fn test_work_library_fnes_mode() {
        let lib = IGESSelectWorkLibrary::new(true);
        assert!(lib.mode_fnes());
    }

    #[test]
    fn test_define_protocol() {
        let proto = IGESSelectWorkLibrary::define_protocol();
        assert_eq!(proto.protocols().len(), 2);
        assert_eq!(proto.protocols()[0].name(), "IGESSolid");
        assert_eq!(proto.protocols()[1].name(), "IGESAppli");
    }

    #[test]
    fn test_write_file_missing_model() {
        let lib = IGESSelectWorkLibrary::new(false);
        let ctx = IFSelectContextWrite::new("/tmp/test.iges");

        let result = lib.write_file(&ctx);
        assert!(!result);
    }

    #[test]
    fn test_file_protocol() {
        let mut fp = IGESFileProtocol::new();
        fp.add_protocol(IGESDataProtocol::new("Protocol1"));
        fp.add_protocol(IGESDataProtocol::new("Protocol2"));

        assert_eq!(fp.protocols().len(), 2);
    }

    #[test]
    fn test_context_modifiers() {
        let mut ctx = IFSelectContextWrite::new("output.iges");
        ctx.set_nb_modifiers(3);

        assert_eq!(ctx.nb_modifiers(), 3);
    }
}
