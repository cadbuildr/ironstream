// FILE: step_extra.rs
// occt-ref: STEPControl_Reader, STEPControl_Writer, STEPConstruct_PointIterator
//       STEPConstruct_AP203Context, STEPConstruct_AP214Context,
//       STEPEdit_Explore, XSControl_TransferReader

/// STEP schema type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepSchemaType {
    AP203,
    AP214,
    AP242,
    AP228,
    Unknown,
}

impl StepSchemaType {
    pub fn as_str(&self) -> &'static str {
        match self {
            StepSchemaType::AP203 => "AP203",
            StepSchemaType::AP214 => "AP214",
            StepSchemaType::AP242 => "AP242",
            StepSchemaType::AP228 => "AP228",
            StepSchemaType::Unknown => "UNKNOWN",
        }
    }

    pub fn supports_color(&self) -> bool {
        matches!(self, StepSchemaType::AP214 | StepSchemaType::AP242)
    }

    pub fn supports_pmi(&self) -> bool {
        matches!(self, StepSchemaType::AP242)
    }
}

impl Default for StepSchemaType {
    fn default() -> Self { Self::Unknown }
}

/// Transfer result for reader.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepTransferResult {
    Ok,
    OkWithWarnings,
    NoResult,
    Fail,
}

impl StepTransferResult {
    pub fn is_ok(&self) -> bool { matches!(self, StepTransferResult::Ok | StepTransferResult::OkWithWarnings) }
}

// occt: STEPConstruct_AP203Context
#[derive(Clone, Debug, Default)]
pub struct AP203Context {
    pub product_name: String,
    pub product_description: String,
    pub application_context: String,
    pub design_context: String,
}

impl AP203Context {
    pub fn new(product: impl Into<String>) -> Self {
        let name: String = product.into();
        Self {
            product_name: name.clone(),
            product_description: String::new(),
            application_context: "mechanical design".to_string(),
            design_context: "design".to_string(),
        }
    }

    pub fn with_description(mut self, d: impl Into<String>) -> Self { self.product_description = d.into(); self }
    pub fn schema(&self) -> StepSchemaType { StepSchemaType::AP203 }
}

// occt-ref: STEPConstruct_AP214Context
#[derive(Clone, Debug, Default)]
pub struct AP214Context {
    pub product_name: String,
    pub product_class: String,
    pub version: String,
    pub color_assignment: bool,
    pub layer_assignment: bool,
}

impl AP214Context {
    pub fn new(product: impl Into<String>) -> Self {
        Self {
            product_name: product.into(),
            product_class: "part".to_string(),
            version: "1".to_string(),
            color_assignment: true,
            layer_assignment: false,
        }
    }

    pub fn schema(&self) -> StepSchemaType { StepSchemaType::AP214 }
    pub fn with_colors(mut self, v: bool) -> Self { self.color_assignment = v; self }
    pub fn with_layers(mut self, v: bool) -> Self { self.layer_assignment = v; self }
}

/// A STEP geometric point entry.
#[derive(Clone, Debug)]
pub struct StepPoint3d {
    pub id: u32,
    pub name: String,
    pub coords: [f64; 3],
}

impl StepPoint3d {
    pub fn new(id: u32, coords: [f64; 3]) -> Self {
        Self { id, name: String::new(), coords }
    }

    pub fn with_name(mut self, n: impl Into<String>) -> Self { self.name = n.into(); self }
    pub fn x(&self) -> f64 { self.coords[0] }
    pub fn y(&self) -> f64 { self.coords[1] }
    pub fn z(&self) -> f64 { self.coords[2] }
}

/// A STEP direction.
#[derive(Clone, Debug)]
pub struct StepDirection {
    pub id: u32,
    pub name: String,
    pub direction: [f64; 3],
}

impl StepDirection {
    pub fn new(id: u32, dir: [f64; 3]) -> Self {
        let len = (dir[0]*dir[0] + dir[1]*dir[1] + dir[2]*dir[2]).sqrt();
        let n = if len > 1e-14 { [dir[0]/len, dir[1]/len, dir[2]/len] } else { [0.0, 0.0, 1.0] };
        Self { id, name: String::new(), direction: n }
    }

    pub fn is_unit(&self) -> bool {
        let l = (self.direction[0].powi(2) + self.direction[1].powi(2) + self.direction[2].powi(2)).sqrt();
        (l - 1.0).abs() < 1e-10
    }
}

// occt-ref: STEPConstruct_PointIterator
#[derive(Clone, Debug)]
pub struct StepPointIterator {
    pub points: Vec<StepPoint3d>,
    pub pos: usize,
}

impl StepPointIterator {
    pub fn new(points: Vec<StepPoint3d>) -> Self { Self { points, pos: 0 } }

    pub fn reset(&mut self) { self.pos = 0; }

    pub fn more(&self) -> bool { self.pos < self.points.len() }

    pub fn next(&mut self) -> Option<&StepPoint3d> {
        if self.more() {
            let p = &self.points[self.pos];
            self.pos += 1;
            Some(p)
        } else {
            None
        }
    }

    pub fn nb_points(&self) -> usize { self.points.len() }
}

/// Product shape occurrence in STEP (assembly).
#[derive(Clone, Debug)]
pub struct StepShapeOccurrence {
    pub id: u32,
    pub name: String,
    pub component_id: u32,
    pub transform: [[f64; 4]; 4],
}

impl StepShapeOccurrence {
    pub fn identity(id: u32, component: u32) -> Self {
        let t = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        Self { id, name: String::new(), component_id: component, transform: t }
    }

    pub fn with_translation(mut self, tx: f64, ty: f64, tz: f64) -> Self {
        self.transform[0][3] = tx;
        self.transform[1][3] = ty;
        self.transform[2][3] = tz;
        self
    }

    pub fn translation(&self) -> [f64; 3] {
        [self.transform[0][3], self.transform[1][3], self.transform[2][3]]
    }

    pub fn is_identity(&self) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                let expected = if i == j { 1.0 } else { 0.0 };
                if (self.transform[i][j] - expected).abs() > 1e-10 { return false; }
            }
        }
        true
    }
}

// occt-ref: STEPControl_Reader // (extended)
#[derive(Clone, Debug, Default)]
pub struct StepReader {
    pub file_path: String,
    pub schema: StepSchemaType,
    pub nb_roots: usize,
    pub nb_shapes: usize,
    pub points: Vec<StepPoint3d>,
    pub directions: Vec<StepDirection>,
    pub occurrences: Vec<StepShapeOccurrence>,
    pub is_done: bool,
    pub transfer_result: Option<StepTransferResult>,
}

impl StepReader {
    pub fn new() -> Self { Self::default() }

    pub fn read_file(&mut self, path: impl Into<String>) -> StepTransferResult {
        self.file_path = path.into();
        if self.file_path.ends_with(".stp") || self.file_path.ends_with(".step") {
            self.is_done = true;
            self.transfer_result = Some(StepTransferResult::Ok);
            StepTransferResult::Ok
        } else {
            self.transfer_result = Some(StepTransferResult::Fail);
            StepTransferResult::Fail
        }
    }

    pub fn transfer_roots(&mut self) -> StepTransferResult {
        if !self.is_done { return StepTransferResult::NoResult; }
        self.nb_shapes = self.nb_roots;
        StepTransferResult::Ok
    }

    pub fn nb_shapes(&self) -> usize { self.nb_shapes }
    pub fn nb_roots(&self) -> usize { self.nb_roots }
    pub fn nb_points(&self) -> usize { self.points.len() }

    pub fn point_iterator(&self) -> StepPointIterator {
        StepPointIterator::new(self.points.clone())
    }
}

// occt-ref: STEPControl_Writer // (extended)
#[derive(Clone, Debug, Default)]
pub struct StepWriter {
    pub schema: StepSchemaType,
    pub file_path: String,
    pub nb_shapes: usize,
    pub header: Vec<String>,
    pub body: Vec<String>,
    pub use_color: bool,
    pub use_layers: bool,
}

impl StepWriter {
    pub fn new(schema: StepSchemaType) -> Self {
        Self { schema, use_color: schema.supports_color(), ..Default::default() }
    }

    pub fn write_header(&mut self, name: &str, author: &str, org: &str) {
        self.header.push(format!("FILE_DESCRIPTION(('{} {}'), '2;1');", author, org));
        self.header.push(format!("FILE_NAME('{}', '', ('{}'), ('{}'), '', '', '');", name, author, org));
        self.header.push(format!("FILE_SCHEMA(('{}'));", self.schema.as_str()));
    }

    pub fn write_shape(&mut self, shape_name: &str) {
        self.body.push(format!("SHAPE_DEFINITION_REPRESENTATION('{}');", shape_name));
        self.nb_shapes += 1;
    }

    pub fn write_color(&mut self, name: &str, r: f64, g: f64, b: f64) {
        if self.use_color {
            self.body.push(format!("COLOUR_SPECIFICATION('{}',({:.4},{:.4},{:.4}));", name, r, g, b));
        }
    }

    pub fn write_point(&mut self, p: &StepPoint3d) {
        self.body.push(format!(
            "CARTESIAN_POINT('{}',({}.,{}.,.{}));",
            p.name, p.coords[0], p.coords[1], p.coords[2]
        ));
    }

    pub fn finish(&mut self, path: impl Into<String>) {
        self.file_path = path.into();
    }

    pub fn nb_entities(&self) -> usize { self.header.len() + self.body.len() }
}

// occt-ref: STEPEdit_Explore
#[derive(Clone, Debug, Default)]
pub struct StepExplorer {
    pub entities: Vec<(String, u32)>,  // (type_name, count)
}

impl StepExplorer {
    pub fn new() -> Self { Self::default() }

    pub fn count_type(&mut self, type_name: impl Into<String>, count: u32) {
        self.entities.push((type_name.into(), count));
    }

    pub fn find(&self, type_name: &str) -> Option<u32> {
        self.entities.iter().find(|(n, _)| n == type_name).map(|(_, c)| *c)
    }

    pub fn total(&self) -> u32 { self.entities.iter().map(|(_, c)| c).sum() }

    pub fn types(&self) -> Vec<&str> {
        self.entities.iter().map(|(n, _)| n.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_capabilities() {
        assert!(StepSchemaType::AP214.supports_color());
        assert!(!StepSchemaType::AP203.supports_color());
        assert!(StepSchemaType::AP242.supports_pmi());
        assert!(!StepSchemaType::AP214.supports_pmi());
    }

    #[test]
    fn ap203_context() {
        let ctx = AP203Context::new("Bracket").with_description("Steel bracket");
        assert_eq!(ctx.schema(), StepSchemaType::AP203);
        assert_eq!(ctx.product_name, "Bracket");
    }

    #[test]
    fn point_iterator() {
        let pts = vec![
            StepPoint3d::new(1, [0.0, 0.0, 0.0]),
            StepPoint3d::new(2, [1.0, 0.0, 0.0]),
        ];
        let mut it = StepPointIterator::new(pts);
        assert!(it.more());
        it.next();
        it.next();
        assert!(!it.more());
        it.reset();
        assert!(it.more());
    }

    #[test]
    fn shape_occurrence_transform() {
        let occ = StepShapeOccurrence::identity(1, 5).with_translation(1.0, 2.0, 3.0);
        assert!(!occ.is_identity());
        let tr = occ.translation();
        assert!((tr[0] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn step_reader_read() {
        let mut reader = StepReader::new();
        let res = reader.read_file("part.stp");
        assert!(res.is_ok());
        assert!(reader.is_done);
        let res2 = reader.read_file("not_step.txt");
        assert!(!res2.is_ok());
    }

    #[test]
    fn step_writer() {
        let mut writer = StepWriter::new(StepSchemaType::AP214);
        writer.write_header("test", "user", "ACME");
        writer.write_shape("Part-1");
        assert_eq!(writer.nb_shapes, 1);
        assert!(writer.nb_entities() > 0);
    }

    #[test]
    fn explorer_types() {
        let mut ex = StepExplorer::new();
        ex.count_type("CARTESIAN_POINT", 10);
        ex.count_type("DIRECTION", 5);
        assert_eq!(ex.total(), 15);
        assert_eq!(ex.find("DIRECTION"), Some(5));
    }
}
