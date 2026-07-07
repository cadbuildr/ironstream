// FILE: bin_m_naming_naming_driver.rs
// occt: BinMNaming_NamingDriver

/// Binary serialization/deserialization driver for naming attributes.
/// Handles persistence of naming information including type, shape type, arguments, and context.
pub struct BinMNamingNamingDriver {
    _message_driver: Option<String>,
}

impl BinMNamingNamingDriver {
    pub fn new(_message_driver: Option<String>) -> Self {
        BinMNamingNamingDriver {
            _message_driver,
        }
    }

    /// Create a new empty naming attribute.
    pub fn new_empty(&self) -> MockNaming {
        MockNaming::new()
    }

    /// Deserialize naming from binary source.
    pub fn paste_read(
        &self,
        source: &[u8],
        offset: usize,
    ) -> Result<(MockNaming, usize), String> {
        let mut current_offset = offset;

        // Read format flag
        let (format_char, next_offset) = read_char(source, current_offset)?;
        current_offset = next_offset;
        let is_new_format = format_char == 'Z';

        // Read name type
        let (name_type_char, next_offset) = read_char(source, current_offset)?;
        current_offset = next_offset;
        let name_type = char_to_name_type(name_type_char)?;

        // Read shape type
        let (shape_type_char, next_offset) = read_char(source, current_offset)?;
        current_offset = next_offset;
        let shape_type = char_to_shape_type(shape_type_char)?;

        // Read number of arguments
        let (nb_args, next_offset) = read_i32(source, current_offset)?;
        current_offset = next_offset;

        let mut arguments = Vec::new();
        if nb_args > 0 {
            for _ in 0..nb_args {
                let (arg_idx, next_offset) = read_i32(source, current_offset)?;
                current_offset = next_offset;
                arguments.push(arg_idx);
            }
        }

        // Read stop named shape
        let (stop_ns_idx, next_offset) = read_i32(source, current_offset)?;
        current_offset = next_offset;

        // Read index
        let (index, next_offset) = read_i32(source, current_offset)?;
        current_offset = next_offset;

        // Read context label entry
        let (entry_str, next_offset) = read_string(source, current_offset)?;
        current_offset = next_offset;

        // Read orientation (if format version supports it)
        let mut orientation = 0;
        if is_new_format {
            let (orient, next_offset) = read_i32(source, current_offset)?;
            current_offset = next_offset;
            orientation = orient;
        }

        let naming = MockNaming {
            name_type,
            shape_type,
            arguments,
            stop_ns_index: if stop_ns_idx > 0 { Some(stop_ns_idx) } else { None },
            index,
            context_label: entry_str,
            orientation,
            is_new_format,
        };

        Ok((naming, current_offset))
    }

    /// Serialize naming to binary target.
    pub fn paste_write(&self, naming: &MockNaming, target: &mut Vec<u8>) {
        // Write format flag for new format
        write_char(target, 'Z');

        // Write name type
        write_char(target, name_type_to_char(naming.name_type));

        // Write shape type
        write_char(target, shape_type_to_char(naming.shape_type));

        // Write arguments
        write_i32(target, naming.arguments.len() as i32);
        for &arg_idx in &naming.arguments {
            write_i32(target, arg_idx);
        }

        // Write stop named shape
        let stop_ns = naming.stop_ns_index.unwrap_or(0);
        write_i32(target, stop_ns);

        // Write index
        write_i32(target, naming.index);

        // Write context label
        write_string(target, &naming.context_label);

        // Write orientation
        write_i32(target, naming.orientation);
    }
}

/// Naming types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NameType {
    Unknown = 0,
    Identity = 1,
    ModifUntil = 2,
    Generation = 3,
    Intersection = 4,
    Union = 5,
    Substraction = 6,
    ConstShape = 7,
    FilterByNeighbours = 8,
    Orientation = 9,
    WireIn = 10,
    ShellIn = 11,
}

fn name_type_to_char(nt: NameType) -> char {
    match nt {
        NameType::Unknown => 'N',
        NameType::Identity => 'I',
        NameType::ModifUntil => 'M',
        NameType::Generation => 'G',
        NameType::Intersection => 'S',
        NameType::Union => 'U',
        NameType::Substraction => 'B',
        NameType::ConstShape => 'C',
        NameType::FilterByNeighbours => 'F',
        NameType::Orientation => 'O',
        NameType::WireIn => 'W',
        NameType::ShellIn => 'H',
    }
}

fn char_to_name_type(c: char) -> Result<NameType, String> {
    match c {
        'N' => Ok(NameType::Unknown),
        'I' => Ok(NameType::Identity),
        'M' => Ok(NameType::ModifUntil),
        'G' => Ok(NameType::Generation),
        'S' => Ok(NameType::Intersection),
        'U' => Ok(NameType::Union),
        'B' => Ok(NameType::Substraction),
        'C' => Ok(NameType::ConstShape),
        'F' => Ok(NameType::FilterByNeighbours),
        'O' => Ok(NameType::Orientation),
        'W' => Ok(NameType::WireIn),
        'H' => Ok(NameType::ShellIn),
        _ => Err(format!("Unknown name type: {}", c)),
    }
}

/// Shape types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShapeType {
    Compound = 0,
    CompSolid = 1,
    Solid = 2,
    Shell = 3,
    Face = 4,
    Wire = 5,
    Edge = 6,
    Vertex = 7,
    Shape = 8,
}

fn shape_type_to_char(st: ShapeType) -> char {
    match st {
        ShapeType::Compound => 'C',
        ShapeType::CompSolid => 'O',
        ShapeType::Solid => 'S',
        ShapeType::Shell => 'H',
        ShapeType::Face => 'F',
        ShapeType::Wire => 'W',
        ShapeType::Edge => 'E',
        ShapeType::Vertex => 'V',
        ShapeType::Shape => 'A',
    }
}

fn char_to_shape_type(c: char) -> Result<ShapeType, String> {
    match c {
        'C' => Ok(ShapeType::Compound),
        'O' => Ok(ShapeType::CompSolid),
        'S' => Ok(ShapeType::Solid),
        'H' => Ok(ShapeType::Shell),
        'F' => Ok(ShapeType::Face),
        'W' => Ok(ShapeType::Wire),
        'E' => Ok(ShapeType::Edge),
        'V' => Ok(ShapeType::Vertex),
        'A' => Ok(ShapeType::Shape),
        _ => Err(format!("Unknown shape type: {}", c)),
    }
}

/// Mock naming attribute for testing serialization.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MockNaming {
    pub name_type: NameType,
    pub shape_type: ShapeType,
    pub arguments: Vec<i32>,
    pub stop_ns_index: Option<i32>,
    pub index: i32,
    pub context_label: String,
    pub orientation: i32,
    pub is_new_format: bool,
}

impl MockNaming {
    pub fn new() -> Self {
        MockNaming {
            name_type: NameType::Unknown,
            shape_type: ShapeType::Shape,
            arguments: Vec::new(),
            stop_ns_index: None,
            index: 0,
            context_label: "0:0".to_string(),
            orientation: 0,
            is_new_format: true,
        }
    }

    pub fn with_name_type(mut self, nt: NameType) -> Self {
        self.name_type = nt;
        self
    }

    pub fn with_shape_type(mut self, st: ShapeType) -> Self {
        self.shape_type = st;
        self
    }

    pub fn add_argument(mut self, idx: i32) -> Self {
        self.arguments.push(idx);
        self
    }

    pub fn with_stop_ns(mut self, idx: i32) -> Self {
        self.stop_ns_index = Some(idx);
        self
    }

    pub fn with_index(mut self, i: i32) -> Self {
        self.index = i;
        self
    }

    pub fn with_context_label(mut self, label: String) -> Self {
        self.context_label = label;
        self
    }

    pub fn with_orientation(mut self, o: i32) -> Self {
        self.orientation = o;
        self
    }
}

impl Default for MockNaming {
    fn default() -> Self {
        MockNaming::new()
    }
}

fn read_char(source: &[u8], offset: usize) -> Result<(char, usize), String> {
    if offset >= source.len() {
        return Err("Insufficient data for char".to_string());
    }
    Ok((source[offset] as char, offset + 1))
}

fn read_i32(source: &[u8], offset: usize) -> Result<(i32, usize), String> {
    if offset + 4 > source.len() {
        return Err("Insufficient data for i32".to_string());
    }
    let bytes: [u8; 4] = [source[offset], source[offset + 1], source[offset + 2], source[offset + 3]];
    Ok((i32::from_le_bytes(bytes), offset + 4))
}

fn read_string(source: &[u8], offset: usize) -> Result<(String, usize), String> {
    if offset + 4 > source.len() {
        return Err("Insufficient data for string length".to_string());
    }
    let len_bytes: [u8; 4] = [source[offset], source[offset + 1], source[offset + 2], source[offset + 3]];
    let len = i32::from_le_bytes(len_bytes) as usize;
    let str_offset = offset + 4;

    if str_offset + len > source.len() {
        return Err("Insufficient data for string content".to_string());
    }

    let s = String::from_utf8(source[str_offset..str_offset + len].to_vec())
        .map_err(|_| "Invalid UTF-8 in string".to_string())?;

    Ok((s, str_offset + len))
}

fn write_char(target: &mut Vec<u8>, c: char) {
    target.push(c as u8);
}

fn write_i32(target: &mut Vec<u8>, value: i32) {
    target.extend_from_slice(&value.to_le_bytes());
}

fn write_string(target: &mut Vec<u8>, s: &str) {
    let bytes = s.as_bytes();
    write_i32(target, bytes.len() as i32);
    target.extend_from_slice(bytes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naming_creation() {
        let driver = BinMNamingNamingDriver::new(None);
        let naming = driver.new_empty();
        assert_eq!(naming.name_type, NameType::Unknown);
        assert_eq!(naming.shape_type, ShapeType::Shape);
        assert_eq!(naming.arguments.len(), 0);
    }

    #[test]
    fn test_name_type_conversions() {
        assert_eq!(name_type_to_char(NameType::Unknown), 'N');
        assert_eq!(name_type_to_char(NameType::Identity), 'I');
        assert_eq!(name_type_to_char(NameType::Generation), 'G');
        assert_eq!(char_to_name_type('N').unwrap(), NameType::Unknown);
        assert_eq!(char_to_name_type('I').unwrap(), NameType::Identity);
    }

    #[test]
    fn test_shape_type_conversions() {
        assert_eq!(shape_type_to_char(ShapeType::Vertex), 'V');
        assert_eq!(shape_type_to_char(ShapeType::Edge), 'E');
        assert_eq!(char_to_shape_type('V').unwrap(), ShapeType::Vertex);
        assert_eq!(char_to_shape_type('E').unwrap(), ShapeType::Edge);
    }

    #[test]
    fn test_serialize_deserialize_simple() {
        let driver = BinMNamingNamingDriver::new(None);

        let original = MockNaming::new()
            .with_name_type(NameType::Identity)
            .with_shape_type(ShapeType::Vertex);

        let mut buffer = Vec::new();
        driver.paste_write(&original, &mut buffer);

        let (deserialized, _) = driver.paste_read(&buffer, 0).unwrap();

        assert_eq!(deserialized.name_type, NameType::Identity);
        assert_eq!(deserialized.shape_type, ShapeType::Vertex);
    }

    #[test]
    fn test_serialize_deserialize_with_arguments() {
        let driver = BinMNamingNamingDriver::new(None);

        let original = MockNaming::new()
            .with_name_type(NameType::Generation)
            .add_argument(1)
            .add_argument(2)
            .add_argument(3);

        let mut buffer = Vec::new();
        driver.paste_write(&original, &mut buffer);

        let (deserialized, _) = driver.paste_read(&buffer, 0).unwrap();

        assert_eq!(deserialized.arguments.len(), 3);
        assert_eq!(deserialized.arguments[0], 1);
    }

    #[test]
    fn test_serialize_deserialize_with_context() {
        let driver = BinMNamingNamingDriver::new(None);

        let original = MockNaming::new()
            .with_context_label("0:1:2".to_string())
            .with_orientation(1);

        let mut buffer = Vec::new();
        driver.paste_write(&original, &mut buffer);

        let (deserialized, _) = driver.paste_read(&buffer, 0).unwrap();

        assert_eq!(deserialized.context_label, "0:1:2");
        assert_eq!(deserialized.orientation, 1);
    }

    #[test]
    fn test_serialize_deserialize_with_stop_ns() {
        let driver = BinMNamingNamingDriver::new(None);

        let original = MockNaming::new()
            .with_stop_ns(42)
            .with_index(5);

        let mut buffer = Vec::new();
        driver.paste_write(&original, &mut buffer);

        let (deserialized, _) = driver.paste_read(&buffer, 0).unwrap();

        assert_eq!(deserialized.stop_ns_index, Some(42));
        assert_eq!(deserialized.index, 5);
    }

    #[test]
    fn test_all_name_types() {
        for name_type in &[
            NameType::Unknown,
            NameType::Identity,
            NameType::ModifUntil,
            NameType::Generation,
            NameType::Intersection,
            NameType::Union,
            NameType::Substraction,
            NameType::ConstShape,
            NameType::FilterByNeighbours,
            NameType::Orientation,
            NameType::WireIn,
            NameType::ShellIn,
        ] {
            let c = name_type_to_char(*name_type);
            let nt = char_to_name_type(c).unwrap();
            assert_eq!(nt, *name_type);
        }
    }

    #[test]
    fn test_all_shape_types() {
        for shape_type in &[
            ShapeType::Compound,
            ShapeType::CompSolid,
            ShapeType::Solid,
            ShapeType::Shell,
            ShapeType::Face,
            ShapeType::Wire,
            ShapeType::Edge,
            ShapeType::Vertex,
            ShapeType::Shape,
        ] {
            let c = shape_type_to_char(*shape_type);
            let st = char_to_shape_type(c).unwrap();
            assert_eq!(st, *shape_type);
        }
    }

    #[test]
    fn test_invalid_chars() {
        assert!(char_to_name_type('X').is_err());
        assert!(char_to_shape_type('X').is_err());
    }
}
