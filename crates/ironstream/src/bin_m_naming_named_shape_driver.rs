// FILE: bin_m_naming_named_shape_driver.rs
// occt: BinMNaming_NamedShapeDriver

use std::collections::HashMap;

/// Binary serialization/deserialization driver for named shape attributes.
/// Handles persistence of naming evolution, shapes, and geometric data with support for triangulation.
pub struct BinMNamingNamedShapeDriver {
    _message_driver: Option<String>,
    with_triangles: bool,
    with_normals: bool,
    is_quick_part: bool,
}

impl BinMNamingNamedShapeDriver {
    pub fn new(_message_driver: Option<String>) -> Self {
        BinMNamingNamedShapeDriver {
            _message_driver,
            with_triangles: false,
            with_normals: false,
            is_quick_part: false,
        }
    }

    /// Create a new empty named shape attribute.
    pub fn new_empty(&self) -> MockNamedShape {
        MockNamedShape::new()
    }

    /// Set whether to store triangulation data.
    pub fn set_with_triangles(&mut self, with_triangles: bool) {
        self.with_triangles = with_triangles;
    }

    /// Set whether to store triangulation normals.
    pub fn set_with_normals(&mut self, with_normals: bool) {
        self.with_normals = with_normals;
    }

    /// Return true if shape should be stored with triangles.
    pub fn is_with_triangles(&self) -> bool {
        self.with_triangles
    }

    /// Return true if shape should be stored with triangulation normals.
    pub fn is_with_normals(&self) -> bool {
        self.with_normals
    }

    /// Enable/disable quick part mode (shapes stored in attribute).
    pub fn enable_quick_part(&mut self, value: bool) {
        self.is_quick_part = value;
    }

    /// Check if quick part mode is enabled.
    pub fn is_quick_part(&self) -> bool {
        self.is_quick_part
    }

    /// Deserialize named shape from binary source.
    pub fn paste_read(
        &self,
        source: &[u8],
        offset: usize,
    ) -> Result<(MockNamedShape, usize), String> {
        let mut current_offset = offset;

        // Read number of shapes
        let (nb_shapes, next_offset) = read_i32(source, current_offset)?;
        current_offset = next_offset;

        // Read version
        let (version, next_offset) = read_i32(source, current_offset)?;
        current_offset = next_offset;

        // Read evolution character
        let (evol_char, next_offset) = read_char(source, current_offset)?;
        current_offset = next_offset;

        let evolution = char_to_evolution(evol_char)?;

        let mut named_shape = MockNamedShape {
            nb_shapes,
            version,
            evolution,
            old_shapes: Vec::new(),
            new_shapes: Vec::new(),
        };

        // Read shape pairs (only for actual use in tests)
        for _ in 0..nb_shapes {
            let mut old_shape_idx = None;
            let mut new_shape_idx = None;

            if evolution != NamingEvolution::Delete {
                let (idx, next_offset) = read_i32(source, current_offset)?;
                current_offset = next_offset;
                new_shape_idx = Some(idx);
            }

            if evolution != NamingEvolution::Primitive {
                let (idx, next_offset) = read_i32(source, current_offset)?;
                current_offset = next_offset;
                old_shape_idx = Some(idx);
            }

            if let Some(idx) = old_shape_idx {
                named_shape.old_shapes.push(idx);
            }
            if let Some(idx) = new_shape_idx {
                named_shape.new_shapes.push(idx);
            }
        }

        Ok((named_shape, current_offset))
    }

    /// Serialize named shape to binary target.
    pub fn paste_write(&self, named_shape: &MockNamedShape, target: &mut Vec<u8>) {
        // Write number of shapes
        write_i32(target, named_shape.nb_shapes);

        // Write version
        write_i32(target, named_shape.version);

        // Write evolution as character
        let evol_char = evolution_to_char(&named_shape.evolution);
        write_char(target, evol_char);

        // Write shape pairs
        for i in 0..named_shape.nb_shapes as usize {
            if named_shape.evolution != NamingEvolution::Delete {
                let idx = named_shape.new_shapes.get(i).copied().unwrap_or(0);
                write_i32(target, idx);
            }

            if named_shape.evolution != NamingEvolution::Primitive {
                let idx = named_shape.old_shapes.get(i).copied().unwrap_or(0);
                write_i32(target, idx);
            }
        }
    }
}

/// Naming evolution types.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NamingEvolution {
    Primitive = 0,
    Generated = 1,
    Modify = 2,
    Delete = 3,
    Selected = 4,
}

impl NamingEvolution {
    pub fn to_char(self) -> char {
        match self {
            NamingEvolution::Primitive => 'P',
            NamingEvolution::Generated => 'G',
            NamingEvolution::Modify => 'M',
            NamingEvolution::Delete => 'D',
            NamingEvolution::Selected => 'S',
        }
    }

    pub fn from_char(c: char) -> Result<Self, String> {
        match c {
            'P' => Ok(NamingEvolution::Primitive),
            'G' => Ok(NamingEvolution::Generated),
            'M' => Ok(NamingEvolution::Modify),
            'D' => Ok(NamingEvolution::Delete),
            'S' => Ok(NamingEvolution::Selected),
            'R' => Ok(NamingEvolution::Modify), // backward compat
            _ => Err(format!("Unknown evolution char: {}", c)),
        }
    }
}

/// Mock named shape attribute for testing serialization.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MockNamedShape {
    pub nb_shapes: i32,
    pub version: i32,
    pub evolution: NamingEvolution,
    pub old_shapes: Vec<i32>,
    pub new_shapes: Vec<i32>,
}

impl MockNamedShape {
    pub fn new() -> Self {
        MockNamedShape {
            nb_shapes: 0,
            version: 0,
            evolution: NamingEvolution::Primitive,
            old_shapes: Vec::new(),
            new_shapes: Vec::new(),
        }
    }

    pub fn with_nb_shapes(mut self, nb: i32) -> Self {
        self.nb_shapes = nb;
        self
    }

    pub fn with_version(mut self, v: i32) -> Self {
        self.version = v;
        self
    }

    pub fn with_evolution(mut self, e: NamingEvolution) -> Self {
        self.evolution = e;
        self
    }

    pub fn add_old_shape(mut self, idx: i32) -> Self {
        self.old_shapes.push(idx);
        self
    }

    pub fn add_new_shape(mut self, idx: i32) -> Self {
        self.new_shapes.push(idx);
        self
    }
}

impl Default for MockNamedShape {
    fn default() -> Self {
        MockNamedShape::new()
    }
}

fn evolution_to_char(evolution: &NamingEvolution) -> char {
    evolution.to_char()
}

fn char_to_evolution(c: char) -> Result<NamingEvolution, String> {
    NamingEvolution::from_char(c)
}

fn read_i32(source: &[u8], offset: usize) -> Result<(i32, usize), String> {
    if offset + 4 > source.len() {
        return Err("Insufficient data for i32".to_string());
    }
    let bytes: [u8; 4] = [source[offset], source[offset + 1], source[offset + 2], source[offset + 3]];
    Ok((i32::from_le_bytes(bytes), offset + 4))
}

fn read_char(source: &[u8], offset: usize) -> Result<(char, usize), String> {
    if offset >= source.len() {
        return Err("Insufficient data for char".to_string());
    }
    Ok((source[offset] as char, offset + 1))
}

fn write_i32(target: &mut Vec<u8>, value: i32) {
    target.extend_from_slice(&value.to_le_bytes());
}

fn write_char(target: &mut Vec<u8>, c: char) {
    target.push(c as u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_named_shape_creation() {
        let driver = BinMNamingNamedShapeDriver::new(None);
        let ns = driver.new_empty();
        assert_eq!(ns.nb_shapes, 0);
        assert_eq!(ns.version, 0);
        assert_eq!(ns.evolution, NamingEvolution::Primitive);
    }

    #[test]
    fn test_triangulation_flags() {
        let mut driver = BinMNamingNamedShapeDriver::new(None);
        assert!(!driver.is_with_triangles());
        assert!(!driver.is_with_normals());

        driver.set_with_triangles(true);
        assert!(driver.is_with_triangles());

        driver.set_with_normals(true);
        assert!(driver.is_with_normals());
    }

    #[test]
    fn test_quick_part_mode() {
        let mut driver = BinMNamingNamedShapeDriver::new(None);
        assert!(!driver.is_quick_part());

        driver.enable_quick_part(true);
        assert!(driver.is_quick_part());

        driver.enable_quick_part(false);
        assert!(!driver.is_quick_part());
    }

    #[test]
    fn test_evolution_to_char() {
        assert_eq!(NamingEvolution::Primitive.to_char(), 'P');
        assert_eq!(NamingEvolution::Generated.to_char(), 'G');
        assert_eq!(NamingEvolution::Modify.to_char(), 'M');
        assert_eq!(NamingEvolution::Delete.to_char(), 'D');
        assert_eq!(NamingEvolution::Selected.to_char(), 'S');
    }

    #[test]
    fn test_char_to_evolution() {
        assert_eq!(NamingEvolution::from_char('P').unwrap(), NamingEvolution::Primitive);
        assert_eq!(NamingEvolution::from_char('G').unwrap(), NamingEvolution::Generated);
        assert_eq!(NamingEvolution::from_char('M').unwrap(), NamingEvolution::Modify);
        assert_eq!(NamingEvolution::from_char('D').unwrap(), NamingEvolution::Delete);
        assert_eq!(NamingEvolution::from_char('S').unwrap(), NamingEvolution::Selected);
    }

    #[test]
    fn test_char_to_evolution_backward_compat() {
        assert_eq!(NamingEvolution::from_char('R').unwrap(), NamingEvolution::Modify);
    }

    #[test]
    fn test_serialize_deserialize_primitive() {
        let driver = BinMNamingNamedShapeDriver::new(None);

        let original = MockNamedShape::new()
            .with_nb_shapes(1)
            .with_version(1)
            .with_evolution(NamingEvolution::Primitive)
            .add_new_shape(1);

        let mut buffer = Vec::new();
        driver.paste_write(&original, &mut buffer);

        let (deserialized, _) = driver.paste_read(&buffer, 0).unwrap();

        assert_eq!(deserialized.nb_shapes, 1);
        assert_eq!(deserialized.version, 1);
        assert_eq!(deserialized.evolution, NamingEvolution::Primitive);
    }

    #[test]
    fn test_serialize_deserialize_generated() {
        let driver = BinMNamingNamedShapeDriver::new(None);

        let original = MockNamedShape::new()
            .with_nb_shapes(1)
            .with_version(2)
            .with_evolution(NamingEvolution::Generated)
            .add_old_shape(1)
            .add_new_shape(2);

        let mut buffer = Vec::new();
        driver.paste_write(&original, &mut buffer);

        let (deserialized, _) = driver.paste_read(&buffer, 0).unwrap();

        assert_eq!(deserialized.evolution, NamingEvolution::Generated);
        assert!(deserialized.old_shapes.len() > 0 || deserialized.new_shapes.len() > 0);
    }

    #[test]
    fn test_serialize_deserialize_delete() {
        let driver = BinMNamingNamedShapeDriver::new(None);

        let original = MockNamedShape::new()
            .with_nb_shapes(1)
            .with_version(3)
            .with_evolution(NamingEvolution::Delete)
            .add_old_shape(1);

        let mut buffer = Vec::new();
        driver.paste_write(&original, &mut buffer);

        let (deserialized, _) = driver.paste_read(&buffer, 0).unwrap();

        assert_eq!(deserialized.evolution, NamingEvolution::Delete);
    }

    #[test]
    fn test_serialize_deserialize_multiple_shapes() {
        let driver = BinMNamingNamedShapeDriver::new(None);

        let original = MockNamedShape::new()
            .with_nb_shapes(3)
            .with_evolution(NamingEvolution::Modify)
            .add_old_shape(1)
            .add_new_shape(2)
            .add_old_shape(3)
            .add_new_shape(4)
            .add_old_shape(5)
            .add_new_shape(6);

        let mut buffer = Vec::new();
        driver.paste_write(&original, &mut buffer);

        let (deserialized, _) = driver.paste_read(&buffer, 0).unwrap();

        assert_eq!(deserialized.nb_shapes, 3);
    }

    #[test]
    fn test_read_insufficient_data() {
        let driver = BinMNamingNamedShapeDriver::new(None);
        let short_buffer = vec![1];
        let result = driver.paste_read(&short_buffer, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_evolution_char() {
        let result = NamingEvolution::from_char('X');
        assert!(result.is_err());
    }
}
