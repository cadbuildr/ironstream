// FILE: rw_dxf.rs
// occt: RWMesh_CafReader (DXF variant), DXF entities (DxfLine, DxfArc, DxfCircle, DxfPolyline)

// occt: DXF entity types
#[derive(Clone, Debug, PartialEq)]
pub enum DxfEntity {
    Line(DxfLine),
    Arc(DxfArc),
    Circle(DxfCircle),
    Polyline(DxfPolyline),
    Text(DxfText),
    Insert(DxfInsert),
}

#[derive(Clone, Debug, PartialEq)]
pub struct DxfLine {
    pub start: [f64; 3],
    pub end: [f64; 3],
    pub layer: String,
    pub color: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DxfArc {
    pub center: [f64; 3],
    pub radius: f64,
    pub start_angle: f64,
    pub end_angle: f64,
    pub layer: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DxfCircle {
    pub center: [f64; 3],
    pub radius: f64,
    pub layer: String,
    pub color: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DxfPolyline {
    pub vertices: Vec<[f64; 3]>,
    pub is_closed: bool,
    pub layer: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DxfText {
    pub position: [f64; 3],
    pub text: String,
    pub height: f64,
    pub layer: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DxfInsert {
    pub block_name: String,
    pub position: [f64; 3],
    pub scale: [f64; 3],
    pub rotation: f64,
    pub layer: String,
}

// occt: DXF reader
#[derive(Clone, Debug, Default)]
pub struct DxfReader {
    pub entities: Vec<DxfEntity>,
    pub layers: Vec<String>,
    pub blocks: Vec<DxfBlock>,
    pub done: bool,
}

#[derive(Clone, Debug)]
pub struct DxfBlock {
    pub name: String,
    pub base_point: [f64; 3],
    pub entities: Vec<DxfEntity>,
}

impl DxfReader {
    pub fn new() -> Self { Self::default() }

    pub fn parse(&mut self, content: &str) {
        // Minimal DXF parser: the file is a sequence of (group code, value)
        // line pairs. An entity starts at group code 0 whose value is the
        // entity type name; subsequent group codes fill in its fields until
        // the next group code 0.
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;
        let mut current: Option<DxfEntity> = None;
        while i + 1 < lines.len() {
            let code = lines[i].trim();
            let val = lines[i + 1].trim();
            if code == "0" {
                // Flush the entity under construction, then possibly start a new one.
                if let Some(e) = current.take() {
                    self.record_layer(&e);
                    self.entities.push(e);
                }
                current = match val {
                    "LINE" => Some(DxfEntity::Line(DxfLine {
                        start: [0.0; 3], end: [0.0; 3],
                        layer: "0".to_string(), color: 7,
                    })),
                    "CIRCLE" => Some(DxfEntity::Circle(DxfCircle {
                        center: [0.0; 3], radius: 0.0,
                        layer: "0".to_string(), color: 7,
                    })),
                    "ARC" => Some(DxfEntity::Arc(DxfArc {
                        center: [0.0; 3], radius: 0.0,
                        start_angle: 0.0, end_angle: 0.0,
                        layer: "0".to_string(),
                    })),
                    "POLYLINE" | "LWPOLYLINE" => Some(DxfEntity::Polyline(DxfPolyline {
                        vertices: Vec::new(), is_closed: false,
                        layer: "0".to_string(),
                    })),
                    "TEXT" | "MTEXT" => Some(DxfEntity::Text(DxfText {
                        position: [0.0; 3], text: String::new(),
                        height: 0.0, layer: "0".to_string(),
                    })),
                    "INSERT" => Some(DxfEntity::Insert(DxfInsert {
                        block_name: String::new(), position: [0.0; 3],
                        scale: [1.0; 3], rotation: 0.0,
                        layer: "0".to_string(),
                    })),
                    _ => None, // SECTION, ENDSEC, EOF, VERTEX markers, ...
                };
            } else if let Some(ref mut e) = current {
                Self::apply_group(e, code, val);
            } else if code == "8" && !val.is_empty() {
                // Layer reference outside an entity (e.g. layer table).
                if !self.layers.contains(&val.to_string()) {
                    self.layers.push(val.to_string());
                }
            }
            i += 2;
        }
        if let Some(e) = current.take() {
            self.record_layer(&e);
            self.entities.push(e);
        }
        self.done = true;
    }

    fn record_layer(&mut self, e: &DxfEntity) {
        let layer = match e {
            DxfEntity::Line(l) => &l.layer,
            DxfEntity::Arc(a) => &a.layer,
            DxfEntity::Circle(c) => &c.layer,
            DxfEntity::Polyline(p) => &p.layer,
            DxfEntity::Text(t) => &t.layer,
            DxfEntity::Insert(ins) => &ins.layer,
        };
        if !self.layers.contains(layer) { self.layers.push(layer.clone()); }
    }

    fn apply_group(e: &mut DxfEntity, code: &str, val: &str) {
        let f = || val.parse::<f64>().unwrap_or(0.0);
        match e {
            DxfEntity::Line(l) => match code {
                "8" => l.layer = val.to_string(),
                "62" => l.color = val.parse().unwrap_or(7),
                "10" => l.start[0] = f(),
                "20" => l.start[1] = f(),
                "30" => l.start[2] = f(),
                "11" => l.end[0] = f(),
                "21" => l.end[1] = f(),
                "31" => l.end[2] = f(),
                _ => {}
            },
            DxfEntity::Circle(c) => match code {
                "8" => c.layer = val.to_string(),
                "62" => c.color = val.parse().unwrap_or(7),
                "10" => c.center[0] = f(),
                "20" => c.center[1] = f(),
                "30" => c.center[2] = f(),
                "40" => c.radius = f(),
                _ => {}
            },
            DxfEntity::Arc(a) => match code {
                "8" => a.layer = val.to_string(),
                "10" => a.center[0] = f(),
                "20" => a.center[1] = f(),
                "30" => a.center[2] = f(),
                "40" => a.radius = f(),
                "50" => a.start_angle = f(),
                "51" => a.end_angle = f(),
                _ => {}
            },
            DxfEntity::Polyline(p) => match code {
                "8" => p.layer = val.to_string(),
                "70" => p.is_closed = val.parse::<i32>().unwrap_or(0) & 1 != 0,
                "10" => p.vertices.push([f(), 0.0, 0.0]),
                "20" => { if let Some(v) = p.vertices.last_mut() { v[1] = f(); } },
                "30" => { if let Some(v) = p.vertices.last_mut() { v[2] = f(); } },
                _ => {}
            },
            DxfEntity::Text(t) => match code {
                "8" => t.layer = val.to_string(),
                "1" => t.text = val.to_string(),
                "40" => t.height = f(),
                "10" => t.position[0] = f(),
                "20" => t.position[1] = f(),
                "30" => t.position[2] = f(),
                _ => {}
            },
            DxfEntity::Insert(ins) => match code {
                "8" => ins.layer = val.to_string(),
                "2" => ins.block_name = val.to_string(),
                "10" => ins.position[0] = f(),
                "20" => ins.position[1] = f(),
                "30" => ins.position[2] = f(),
                "41" => ins.scale[0] = f(),
                "42" => ins.scale[1] = f(),
                "43" => ins.scale[2] = f(),
                "50" => ins.rotation = f(),
                _ => {}
            },
        }
    }

    pub fn is_done(&self) -> bool { self.done }
    pub fn nb_entities(&self) -> usize { self.entities.len() }
    pub fn nb_layers(&self) -> usize { self.layers.len() }

    pub fn entities_of_type(&self, t: &str) -> Vec<&DxfEntity> {
        self.entities.iter().filter(|e| match (t, e) {
            ("LINE", DxfEntity::Line(_)) => true,
            ("ARC", DxfEntity::Arc(_)) => true,
            ("CIRCLE", DxfEntity::Circle(_)) => true,
            _ => false,
        }).collect()
    }
}

// DXF writer
#[derive(Clone, Debug, Default)]
pub struct DxfWriter {
    pub content: String,
}

impl DxfWriter {
    pub fn new() -> Self { Self::default() }

    pub fn write_header(&mut self) {
        self.content.push_str("0\nSECTION\n2\nHEADER\n0\nENDSEC\n");
        self.content.push_str("0\nSECTION\n2\nENTITIES\n");
    }

    pub fn write_line(&mut self, line: &DxfLine) {
        self.content.push_str(&format!(
            "0\nLINE\n8\n{}\n10\n{}\n20\n{}\n30\n{}\n11\n{}\n21\n{}\n31\n{}\n",
            line.layer,
            line.start[0], line.start[1], line.start[2],
            line.end[0], line.end[1], line.end[2]
        ));
    }

    pub fn write_circle(&mut self, circle: &DxfCircle) {
        self.content.push_str(&format!(
            "0\nCIRCLE\n8\n{}\n10\n{}\n20\n{}\n30\n{}\n40\n{}\n",
            circle.layer,
            circle.center[0], circle.center[1], circle.center[2],
            circle.radius
        ));
    }

    pub fn finish(&mut self) {
        self.content.push_str("0\nENDSEC\n0\nEOF\n");
    }

    pub fn content_str(&self) -> &str { &self.content }
    pub fn byte_count(&self) -> usize { self.content.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dxf_reader_parse() {
        let dxf = "0\nSECTION\n8\nLayer1\n0\nLINE\n8\nLayer1\n0\nCIRCLE\n8\nLayer1\n";
        let mut r = DxfReader::new();
        r.parse(dxf);
        assert!(r.is_done());
        assert_eq!(r.nb_entities(), 2);
    }

    #[test]
    fn dxf_entities_of_type() {
        let mut r = DxfReader::new();
        r.parse("0\nLINE\n0\nARC\n0\nLINE\n");
        let lines = r.entities_of_type("LINE");
        assert_eq!(lines.len(), 2);
    }

    #[test]
    fn dxf_writer_line() {
        let mut w = DxfWriter::new();
        w.write_header();
        w.write_line(&DxfLine { start: [0.0;3], end: [1.0,0.0,0.0], layer: "0".into(), color: 7 });
        w.finish();
        let s = w.content_str();
        assert!(s.contains("LINE"));
        assert!(s.contains("EOF"));
    }

    #[test]
    fn dxf_writer_circle() {
        let mut w = DxfWriter::new();
        w.write_header();
        w.write_circle(&DxfCircle { center: [0.0;3], radius: 5.0, layer: "0".into(), color: 1 });
        w.finish();
        assert!(w.content_str().contains("CIRCLE"));
        assert!(w.byte_count() > 0);
    }

    #[test]
    fn dxf_reader_nb_layers() {
        let mut r = DxfReader::new();
        r.parse("8\nALayer\n0\nLINE\n");
        assert!(r.nb_layers() >= 1);
    }
}
