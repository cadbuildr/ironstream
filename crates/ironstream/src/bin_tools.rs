// FILE: bin_tools.rs
// occt: BinTools_ShapeSet, BinTools_Curve2dSet, BinTools_SurfaceSet
//       BinTools_CurveSet

/// Version of the binary shape format.
/// occt: BinTools_FormatVersion
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum BinToolsFormatVersion {
    Version1,
    Version2,
    #[default]
    Version3,
    Version4,
}

impl BinToolsFormatVersion {
    pub fn as_u8(&self) -> u8 {
        match self {
            Self::Version1 => 1,
            Self::Version2 => 2,
            Self::Version3 => 3,
            Self::Version4 => 4,
        }
    }

    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            1 => Some(Self::Version1),
            2 => Some(Self::Version2),
            3 => Some(Self::Version3),
            4 => Some(Self::Version4),
            _ => None,
        }
    }

    pub fn supports_triangulation(&self) -> bool {
        matches!(self, Self::Version3 | Self::Version4)
    }
}

/// Shape type in the binary stream.
// occt-ref: TopAbs_ShapeEnum // (used in BinTools context)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum BinToolsShapeType {
    #[default]
    Compound,
    CompSolid,
    Solid,
    Shell,
    Face,
    Wire,
    Edge,
    Vertex,
    Shape,
}

impl BinToolsShapeType {
    pub fn type_byte(&self) -> u8 {
        match self {
            Self::Compound   => 0,
            Self::CompSolid  => 1,
            Self::Solid      => 2,
            Self::Shell      => 3,
            Self::Face       => 4,
            Self::Wire       => 5,
            Self::Edge       => 6,
            Self::Vertex     => 7,
            Self::Shape      => 8,
        }
    }

    pub fn from_byte(b: u8) -> Option<Self> {
        match b {
            0 => Some(Self::Compound),
            1 => Some(Self::CompSolid),
            2 => Some(Self::Solid),
            3 => Some(Self::Shell),
            4 => Some(Self::Face),
            5 => Some(Self::Wire),
            6 => Some(Self::Edge),
            7 => Some(Self::Vertex),
            8 => Some(Self::Shape),
            _ => None,
        }
    }
}

/// A single shape record in the binary stream.
/// occt: BinTools_ShapeSet // entry
#[derive(Clone, Debug)]
pub struct BinToolsShapeRecord {
    pub shape_id: u32,
    pub shape_type: BinToolsShapeType,
    pub location_id: u32,
    pub orientation: u8,   // 0=Forward 1=Reversed 2=Internal 3=External
    pub children: Vec<u32>,
}

impl BinToolsShapeRecord {
    pub fn new(shape_id: u32, shape_type: BinToolsShapeType) -> Self {
        Self {
            shape_id,
            shape_type,
            location_id: 0,
            orientation: 0,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child_id: u32) { self.children.push(child_id); }
    pub fn set_location(&mut self, loc_id: u32) { self.location_id = loc_id; }
    pub fn nb_children(&self) -> usize { self.children.len() }
    pub fn is_leaf(&self) -> bool { self.children.is_empty() }
}

/// Binary shape set: stores + serializes shapes.
/// occt: BinTools_ShapeSet
#[derive(Clone, Debug, Default)]
pub struct BinToolsShapeSet {
    pub format_version: BinToolsFormatVersion,
    pub shapes: Vec<BinToolsShapeRecord>,
    pub triangulation_enabled: bool,
}

impl BinToolsShapeSet {
    pub fn new() -> Self { Self::default() }

    pub fn with_version(mut self, v: BinToolsFormatVersion) -> Self {
        self.format_version = v;
        self
    }

    pub fn add_shape(&mut self, rec: BinToolsShapeRecord) -> u32 {
        let id = rec.shape_id;
        self.shapes.push(rec);
        id
    }

    pub fn find_shape(&self, shape_id: u32) -> Option<&BinToolsShapeRecord> {
        self.shapes.iter().find(|s| s.shape_id == shape_id)
    }

    pub fn nb_shapes(&self) -> usize { self.shapes.len() }

    /// Minimal binary serialization: header + shape records.
    pub fn write_to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        // Header: magic(4) + version(1) + count(4)
        buf.extend_from_slice(b"BSHT");
        buf.push(self.format_version.as_u8());
        let count = self.shapes.len() as u32;
        buf.extend_from_slice(&count.to_le_bytes());
        for s in &self.shapes {
            buf.extend_from_slice(&s.shape_id.to_le_bytes());
            buf.push(s.shape_type.type_byte());
            buf.extend_from_slice(&s.location_id.to_le_bytes());
            buf.push(s.orientation);
            buf.extend_from_slice(&(s.children.len() as u32).to_le_bytes());
            for &c in &s.children {
                buf.extend_from_slice(&c.to_le_bytes());
            }
        }
        buf
    }

    /// Read from bytes: returns None if header is invalid.
    pub fn read_from_bytes(data: &[u8]) -> Option<Self> {
        if data.len() < 9 { return None; }
        if &data[..4] != b"BSHT" { return None; }
        let version = BinToolsFormatVersion::from_u8(data[4])?;
        let count = u32::from_le_bytes(data[5..9].try_into().ok()?) as usize;
        let mut shapes = Vec::with_capacity(count);
        let mut pos = 9usize;
        for _ in 0..count {
            if pos + 10 > data.len() { break; }
            let sid = u32::from_le_bytes(data[pos..pos+4].try_into().ok()?); pos += 4;
            let stype = BinToolsShapeType::from_byte(data[pos])?; pos += 1;
            let loc = u32::from_le_bytes(data[pos..pos+4].try_into().ok()?); pos += 4;
            let orient = data[pos]; pos += 1;
            if pos + 4 > data.len() { break; }
            let nchild = u32::from_le_bytes(data[pos..pos+4].try_into().ok()?) as usize; pos += 4;
            let mut children = Vec::with_capacity(nchild);
            for _ in 0..nchild {
                if pos + 4 > data.len() { break; }
                let c = u32::from_le_bytes(data[pos..pos+4].try_into().ok()?); pos += 4;
                children.push(c);
            }
            let mut rec = BinToolsShapeRecord::new(sid, stype);
            rec.location_id = loc;
            rec.orientation = orient;
            rec.children = children;
            shapes.push(rec);
        }
        Some(Self { format_version: version, shapes, triangulation_enabled: false })
    }
}

/// Curve set in the binary format.
/// occt: BinTools_CurveSet
#[derive(Clone, Debug, Default)]
pub struct BinToolsCurveSet {
    pub curve_types: Vec<u8>,  // 1=line 2=circle 3=ellipse 4=bspline etc.
    pub curve_ids: Vec<u32>,
}

impl BinToolsCurveSet {
    pub fn new() -> Self { Self::default() }

    pub fn add_curve(&mut self, curve_id: u32, curve_type: u8) -> usize {
        let idx = self.curve_ids.len();
        self.curve_ids.push(curve_id);
        self.curve_types.push(curve_type);
        idx
    }

    pub fn nb_curves(&self) -> usize { self.curve_ids.len() }

    pub fn curve_type_at(&self, idx: usize) -> Option<u8> {
        self.curve_types.get(idx).copied()
    }
}

/// 2D curve set (UV parameter space).
/// occt: BinTools_Curve2dSet
#[derive(Clone, Debug, Default)]
pub struct BinToolsCurve2dSet {
    pub entries: Vec<(u32, u8)>,  // (curve_id, type)
}

impl BinToolsCurve2dSet {
    pub fn new() -> Self { Self::default() }
    pub fn add(&mut self, curve_id: u32, t: u8) { self.entries.push((curve_id, t)); }
    pub fn nb_curves(&self) -> usize { self.entries.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_version_roundtrip() {
        for v in [1u8, 2, 3, 4] {
            let fv = BinToolsFormatVersion::from_u8(v).unwrap();
            assert_eq!(fv.as_u8(), v);
        }
        assert!(BinToolsFormatVersion::from_u8(99).is_none());
    }

    #[test]
    fn format_version_triangulation() {
        assert!(!BinToolsFormatVersion::Version2.supports_triangulation());
        assert!(BinToolsFormatVersion::Version3.supports_triangulation());
        assert!(BinToolsFormatVersion::Version4.supports_triangulation());
    }

    #[test]
    fn shape_type_roundtrip() {
        for b in 0u8..=8 {
            let st = BinToolsShapeType::from_byte(b).unwrap();
            assert_eq!(st.type_byte(), b);
        }
        assert!(BinToolsShapeType::from_byte(9).is_none());
    }

    #[test]
    fn shape_set_write_read() {
        let mut ss = BinToolsShapeSet::new();
        let mut rec = BinToolsShapeRecord::new(1, BinToolsShapeType::Solid);
        rec.add_child(2);
        rec.add_child(3);
        ss.add_shape(rec);
        let bytes = ss.write_to_bytes();
        let recovered = BinToolsShapeSet::read_from_bytes(&bytes).unwrap();
        assert_eq!(recovered.nb_shapes(), 1);
        let s = recovered.find_shape(1).unwrap();
        assert_eq!(s.shape_type, BinToolsShapeType::Solid);
        assert_eq!(s.nb_children(), 2);
    }

    #[test]
    fn shape_set_invalid_header() {
        let bad = b"XXXX\x03\x00\x00\x00\x00";
        assert!(BinToolsShapeSet::read_from_bytes(bad).is_none());
        assert!(BinToolsShapeSet::read_from_bytes(b"shor").is_none());
    }

    #[test]
    fn curve_sets() {
        let mut cs = BinToolsCurveSet::new();
        cs.add_curve(10, 1);
        cs.add_curve(20, 2);
        assert_eq!(cs.nb_curves(), 2);
        assert_eq!(cs.curve_type_at(0), Some(1));
        assert_eq!(cs.curve_type_at(1), Some(2));
        assert!(cs.curve_type_at(5).is_none());

        let mut c2 = BinToolsCurve2dSet::new();
        c2.add(5, 3);
        assert_eq!(c2.nb_curves(), 1);
    }
}
