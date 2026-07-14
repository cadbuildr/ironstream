// FILE: rw_step_ext.rs
// occt-ref: STEPControl_Reader, STEPControl_Writer, RWStepGeom_RWAxis2Placement3d
//       STEPConstruct_Styles, STEPEdit_Exploration

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepReadStatus {
    IsDone,
    OpenError,
    NotDone,
    BadSchema,
    FailedWithWarnings,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepTransferStatus {
    Done,
    NoTransfer,
    Error,
}

/// STEP entity type codes (subset).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepEntityType {
    AdvancedBrepShapeRepresentation,
    ManifoldSolidBrep,
    ClosedShell,
    FaceSurface,
    AdvancedFace,
    EdgeLoop,
    OrientedEdge,
    EdgeCurve,
    VertexPoint,
    CartesianPoint,
    Direction,
    Axis2Placement3d,
    Plane,
    CylindricalSurface,
    ConicalSurface,
    SphericalSurface,
    ToroidalSurface,
    BSplineSurface,
    BSplineCurve,
    Circle,
    Line,
    Other,
}

/// A minimal STEP entity record.
#[derive(Clone, Debug)]
pub struct StepEntity {
    pub id: u32,
    pub entity_type: StepEntityType,
    pub params: Vec<String>,
}

impl StepEntity {
    pub fn new(id: u32, entity_type: StepEntityType) -> Self {
        Self { id, entity_type, params: Vec::new() }
    }

    pub fn with_param(mut self, p: impl Into<String>) -> Self { self.params.push(p.into()); self }
    pub fn nb_params(&self) -> usize { self.params.len() }
}

// occt-ref: STEPControl_Reader
#[derive(Clone, Debug, Default)]
pub struct StepReader {
    pub entities: Vec<StepEntity>,
    pub warnings: Vec<String>,
    pub status: Option<StepReadStatus>,
    pub schema: String,
}

impl StepReader {
    pub fn new() -> Self { Self { schema: "AP214".into(), ..Default::default() } }

    pub fn read_string(&mut self, content: &str) {
        // Parse minimal STEP header + data
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("FILE_SCHEMA") {
                if let Some(s) = line.split('\'').nth(1) { self.schema = s.to_string(); }
            } else if line.starts_with('#') {
                if let Some(eq) = line.find('=') {
                    let id_str = line[1..eq].trim();
                    if let Ok(id) = id_str.parse::<u32>() {
                        let rest = &line[eq+1..].trim().to_uppercase();
                        let et = classify_entity(rest);
                        self.entities.push(StepEntity::new(id, et));
                    }
                }
            }
        }
        self.status = Some(StepReadStatus::IsDone);
    }

    pub fn nb_roots_for_transfer(&self) -> usize { self.entities.len() }
    pub fn is_done(&self) -> bool { self.status == Some(StepReadStatus::IsDone) }

    pub fn entities_of_type(&self, t: StepEntityType) -> Vec<&StepEntity> {
        self.entities.iter().filter(|e| e.entity_type == t).collect()
    }
}

// occt-ref: STEPControl_Writer
#[derive(Clone, Debug, Default)]
pub struct StepWriter {
    pub content: String,
    pub entity_counter: u32,
}

impl StepWriter {
    pub fn new() -> Self { Self::default() }

    pub fn write_header(&mut self, schema: &str) {
        self.content.push_str("ISO-10303-21;\nHEADER;\n");
        self.content.push_str(&format!("FILE_SCHEMA(('{}'));\n", schema));
        self.content.push_str("ENDSEC;\nDATA;\n");
    }

    pub fn write_cartesian_point(&mut self, x: f64, y: f64, z: f64) -> u32 {
        self.entity_counter += 1;
        let id = self.entity_counter;
        self.content.push_str(&format!("#{} = CARTESIAN_POINT('',({}.,{}.,{}.));\n", id, x, y, z));
        id
    }

    pub fn write_direction(&mut self, x: f64, y: f64, z: f64) -> u32 {
        self.entity_counter += 1;
        let id = self.entity_counter;
        self.content.push_str(&format!("#{} = DIRECTION('',({}.,{}.,{}.));\n", id, x, y, z));
        id
    }

    pub fn finish(&mut self) {
        self.content.push_str("ENDSEC;\nEND-ISO-10303-21;\n");
    }

    pub fn content_str(&self) -> &str { &self.content }
    pub fn byte_count(&self) -> usize { self.content.len() }
}

// occt-note: STEPEdit_Exploration entity count by type
pub struct StepExplorer;

impl StepExplorer {
    pub fn count_by_type(entities: &[StepEntity]) -> Vec<(StepEntityType, usize)> {
        use StepEntityType::*;
        let types = [CartesianPoint, Direction, Axis2Placement3d, Plane, ManifoldSolidBrep, ClosedShell, FaceSurface];
        types.iter().map(|&t| (t, entities.iter().filter(|e| e.entity_type == t).count())).collect()
    }
}

fn classify_entity(s: &str) -> StepEntityType {
    if s.contains("CARTESIAN_POINT") { StepEntityType::CartesianPoint }
    else if s.contains("DIRECTION") { StepEntityType::Direction }
    else if s.contains("AXIS2_PLACEMENT_3D") { StepEntityType::Axis2Placement3d }
    else if s.contains("PLANE") { StepEntityType::Plane }
    else if s.contains("MANIFOLD_SOLID_BREP") { StepEntityType::ManifoldSolidBrep }
    else if s.contains("CLOSED_SHELL") { StepEntityType::ClosedShell }
    else if s.contains("ADVANCED_FACE") { StepEntityType::AdvancedFace }
    else if s.contains("CIRCLE") { StepEntityType::Circle }
    else if s.contains("LINE") { StepEntityType::Line }
    else { StepEntityType::Other }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STEP_CONTENT: &str = r"ISO-10303-21;
HEADER;
FILE_SCHEMA(('AP203'));
ENDSEC;
DATA;
#1 = CARTESIAN_POINT('',(0.,0.,0.));
#2 = CARTESIAN_POINT('',(1.,0.,0.));
#3 = DIRECTION('',(0.,0.,1.));
ENDSEC;
END-ISO-10303-21;
";

    #[test]
    fn step_reader_parse() {
        let mut r = StepReader::new();
        r.read_string(STEP_CONTENT);
        assert!(r.is_done());
        assert_eq!(r.nb_roots_for_transfer(), 3);
        assert_eq!(r.schema, "AP203");
    }

    #[test]
    fn step_reader_entities_of_type() {
        let mut r = StepReader::new();
        r.read_string(STEP_CONTENT);
        let pts = r.entities_of_type(StepEntityType::CartesianPoint);
        assert_eq!(pts.len(), 2);
    }

    #[test]
    fn step_writer_basic() {
        let mut w = StepWriter::new();
        w.write_header("AP214");
        let id1 = w.write_cartesian_point(0.0, 0.0, 0.0);
        let id2 = w.write_direction(0.0, 0.0, 1.0);
        w.finish();
        assert!(w.content_str().contains("CARTESIAN_POINT"));
        assert!(w.content_str().contains("END-ISO-10303-21"));
        assert!(id1 < id2);
    }
}
