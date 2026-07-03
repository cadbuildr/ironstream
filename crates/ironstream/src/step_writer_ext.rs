// FILE: step_writer_ext.rs
// occt: StepData_Writer, StepData_StepWriter, StepData_WriterContext,
//       StepData_FileRecord

/// STEP file header information.
/// occt: StepData_FileHeaderMeta
#[derive(Clone, Debug, Default)]
pub struct StepFileHeader {
    pub description: String,
    pub implementation_level: String,
    pub authorization: String,
    pub organization: String,
    pub schema_version: String,
}

impl StepFileHeader {
    pub fn new() -> Self {
        Self {
            description: "IronStream STEP Writer".into(),
            implementation_level: "2".into(),
            authorization: "".into(),
            organization: "".into(),
            schema_version: "AP214 CD".into(),
        }
    }

    pub fn set_description(&mut self, d: impl Into<String>) { self.description = d.into(); }
    pub fn set_schema(&mut self, s: impl Into<String>) { self.schema_version = s.into(); }
}

/// A single entity record in the STEP file.
/// occt: StepData_FileRecord (simplified)
#[derive(Clone, Debug)]
pub struct StepFileRecord {
    pub record_id: u32,
    pub entity_type: String,
    pub attributes: Vec<String>,
}

impl StepFileRecord {
    pub fn new(id: u32, entity_type: impl Into<String>) -> Self {
        Self {
            record_id: id,
            entity_type: entity_type.into(),
            attributes: Vec::new(),
        }
    }

    pub fn add_attribute(&mut self, attr: impl Into<String>) { self.attributes.push(attr.into()); }
    pub fn nb_attributes(&self) -> usize { self.attributes.len() }

    /// Serialize to STEP line format: #123 = ENTITY_TYPE(...);
    pub fn to_step_line(&self) -> String {
        let attrs = self.attributes.join(",");
        format!("#{} = {}({});", self.record_id, self.entity_type, attrs)
    }
}

/// STEP writer: builds and outputs STEP files.
/// occt: StepData_Writer / StepData_StepWriter
#[derive(Clone, Debug, Default)]
pub struct StepWriter {
    pub header: StepFileHeader,
    pub records: Vec<StepFileRecord>,
    pub data_section_offset: usize,
}

impl StepWriter {
    pub fn new() -> Self { Self { header: StepFileHeader::new(), ..Self::default() } }

    pub fn add_record(&mut self, r: StepFileRecord) { self.records.push(r); }

    pub fn add_shape(&mut self, shape_id: u32, shape_type: &str) -> u32 {
        let rid = (self.records.len() + 1) as u32;
        let mut rec = StepFileRecord::new(rid, shape_type);
        rec.add_attribute(shape_id.to_string());
        self.records.push(rec);
        rid
    }

    pub fn nb_records(&self) -> usize { self.records.len() }

    pub fn to_step_string(&self) -> String {
        let mut out = String::new();
        out.push_str("ISO-10303-21;\n");
        out.push_str("HEADER;\n");
        out.push_str(&format!("FILE_DESCRIPTION(('{}'),{:?});\n", self.header.description, self.header.implementation_level));
        out.push_str(&format!("FILE_NAME('output.stp','{}','{}',('{}'),('{}'),{:?},{:?});\n",
                             chrono_stub::now_str(), "", self.header.organization,
                             self.header.authorization, "IronStream", "2.0"));
        out.push_str(&format!("FILE_SCHEMA(('{}'));\n", self.header.schema_version));
        out.push_str("ENDSEC;\n");
        out.push_str("DATA;\n");
        for rec in &self.records {
            out.push_str(&rec.to_step_line());
            out.push('\n');
        }
        out.push_str("ENDSEC;\nEND-ISO-10303-21;\n");
        out
    }

    pub fn write_to_file(&self, path: &str) -> std::io::Result<()> {
        std::fs::write(path, self.to_step_string())
    }
}

/// Stub for time/date generation.
mod chrono_stub {
    pub fn now_str() -> String { "2024-01-01T00:00:00".into() }
}

/// Writer context: manages entity references during conversion.
/// occt: StepData_WriterContext (stub)
#[derive(Clone, Debug, Default)]
pub struct StepWriterContext {
    pub entity_map: Vec<(u32, u32)>,  // (src_id, step_record_id)
    pub next_record_id: u32,
}

impl StepWriterContext {
    pub fn new() -> Self { Self { next_record_id: 1, ..Self::default() } }

    pub fn map_entity(&mut self, src: u32, step_id: u32) {
        if !self.entity_map.iter().any(|(s, _)| *s == src) {
            self.entity_map.push((src, step_id));
        }
    }

    pub fn find_step_id(&self, src: u32) -> Option<u32> {
        self.entity_map.iter().find(|(s, _)| *s == src).map(|(_, id)| *id)
    }

    pub fn next_id(&mut self) -> u32 {
        let id = self.next_record_id;
        self.next_record_id += 1;
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_header() {
        let mut h = StepFileHeader::new();
        h.set_description("Test CAD Model");
        h.set_schema("AP203");
        assert_eq!(h.description, "Test CAD Model");
        assert_eq!(h.schema_version, "AP203");
    }

    #[test]
    fn file_record() {
        let mut r = StepFileRecord::new(1, "CARTESIAN_POINT");
        r.add_attribute("0.0");
        r.add_attribute("0.0");
        r.add_attribute("0.0");
        assert_eq!(r.nb_attributes(), 3);
        let line = r.to_step_line();
        assert!(line.contains("#1 ="));
        assert!(line.contains("CARTESIAN_POINT"));
    }

    #[test]
    fn step_writer() {
        let mut w = StepWriter::new();
        let _r1 = w.add_shape(10, "SHAPE");
        let _r2 = w.add_shape(11, "SOLID");
        assert_eq!(w.nb_records(), 2);
        let step_str = w.to_step_string();
        assert!(step_str.contains("ISO-10303-21"));
        assert!(step_str.contains("DATA;"));
        assert!(step_str.contains("SHAPE"));
    }

    #[test]
    fn writer_context() {
        let mut ctx = StepWriterContext::new();
        ctx.map_entity(100, 1);
        ctx.map_entity(101, 2);
        assert_eq!(ctx.find_step_id(100), Some(1));
        let nid = ctx.next_id();
        assert_eq!(nid, 1);
        assert_eq!(ctx.next_id(), 2);
    }
}
