// FILE: bin_m_data_xtd_constraint_driver.rs
// occt: BinMDataXtd_ConstraintDriver

use std::collections::HashMap;

/// Binary serialization/deserialization driver for constraint attributes.
/// Handles persistence of constraint metadata including value, geometries, plane, type, and flags.
pub struct BinMDataXtdConstraintDriver {
    _message_driver: Option<String>,
}

impl BinMDataXtdConstraintDriver {
    pub fn new(_message_driver: Option<String>) -> Self {
        BinMDataXtdConstraintDriver {
            _message_driver,
        }
    }

    /// Create a new empty constraint attribute.
    pub fn new_empty(&self) -> MockConstraint {
        MockConstraint::new()
    }

    /// Deserialize constraint from binary source.
    /// Reads: value index, geometries count and indices, plane index, constraint type, flags.
    pub fn paste_read(
        &self,
        source: &[u8],
        reloc_table: &mut HashMap<i32, MockAttribute>,
    ) -> Result<(MockConstraint, usize), String> {
        let mut offset = 0;

        // Read value index
        let (value_nb, next_offset) = read_i32(source, offset)?;
        offset = next_offset;

        let mut constraint = MockConstraint::new();

        if value_nb > 0 {
            if !reloc_table.contains_key(&value_nb) {
                reloc_table.insert(value_nb, MockAttribute::Real(0.0));
            }
        } else if value_nb > 0 {
            constraint.value_index = Some(value_nb);
        }

        // Read geometries count
        let (nb_geom, next_offset) = read_i32(source, offset)?;
        offset = next_offset;

        for _ in 1..=nb_geom {
            let (geom_nb, next_offset) = read_i32(source, offset)?;
            offset = next_offset;

            if geom_nb > 0 {
                if !reloc_table.contains_key(&geom_nb) {
                    reloc_table.insert(geom_nb, MockAttribute::NamedShape);
                }
                constraint.geometries.push(geom_nb);
            }
        }

        // Read plane index
        let (plane_nb, next_offset) = read_i32(source, offset)?;
        offset = next_offset;

        if plane_nb > 0 {
            if !reloc_table.contains_key(&plane_nb) {
                reloc_table.insert(plane_nb, MockAttribute::NamedShape);
            }
            constraint.plane_index = Some(plane_nb);
        }

        // Read constraint type
        let (constraint_type, next_offset) = read_i32(source, offset)?;
        offset = next_offset;
        constraint.constraint_type = constraint_type;

        // Read flags
        let (flags, next_offset) = read_i32(source, offset)?;
        offset = next_offset;

        constraint.verified = (flags & 1) != 0;
        constraint.inverted = (flags & 2) != 0;
        constraint.reversed = (flags & 4) != 0;

        Ok((constraint, offset))
    }

    /// Serialize constraint to binary target.
    /// Writes: value index, geometries count and indices, plane index, constraint type, flags.
    pub fn paste_write(
        &self,
        constraint: &MockConstraint,
        target: &mut Vec<u8>,
        reloc_table: &mut HashMap<MockAttribute, i32>,
    ) {
        // Write value index
        let value_nb = if let Some(idx) = constraint.value_index {
            idx
        } else {
            -1
        };
        write_i32(target, value_nb);

        // Write geometries
        write_i32(target, constraint.geometries.len() as i32);
        for &geom_idx in &constraint.geometries {
            write_i32(target, geom_idx);
        }

        // Write plane index
        let plane_nb = if let Some(idx) = constraint.plane_index {
            idx
        } else {
            -1
        };
        write_i32(target, plane_nb);

        // Write constraint type
        write_i32(target, constraint.constraint_type);

        // Write flags
        let mut flags = 0;
        if constraint.verified {
            flags |= 1;
        }
        if constraint.inverted {
            flags |= 2;
        }
        if constraint.reversed {
            flags |= 4;
        }
        write_i32(target, flags);
    }
}

/// Mock constraint attribute for testing serialization.
#[derive(Clone, Debug, PartialEq)]
pub struct MockConstraint {
    pub value_index: Option<i32>,
    pub geometries: Vec<i32>,
    pub plane_index: Option<i32>,
    pub constraint_type: i32,
    pub verified: bool,
    pub inverted: bool,
    pub reversed: bool,
}

impl MockConstraint {
    pub fn new() -> Self {
        MockConstraint {
            value_index: None,
            geometries: Vec::new(),
            plane_index: None,
            constraint_type: 0,
            verified: false,
            inverted: false,
            reversed: false,
        }
    }

    pub fn with_value(mut self, idx: i32) -> Self {
        self.value_index = Some(idx);
        self
    }

    pub fn with_geometry(mut self, idx: i32) -> Self {
        self.geometries.push(idx);
        self
    }

    pub fn with_plane(mut self, idx: i32) -> Self {
        self.plane_index = Some(idx);
        self
    }

    pub fn with_type(mut self, t: i32) -> Self {
        self.constraint_type = t;
        self
    }

    pub fn with_flags(mut self, verified: bool, inverted: bool, reversed: bool) -> Self {
        self.verified = verified;
        self.inverted = inverted;
        self.reversed = reversed;
        self
    }
}

/// Mock attribute types for relocation table.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum MockAttribute {
    Real(i32),
    NamedShape,
}

impl Default for MockAttribute {
    fn default() -> Self {
        MockAttribute::Real(0)
    }
}

fn read_i32(source: &[u8], offset: usize) -> Result<(i32, usize), String> {
    if offset + 4 > source.len() {
        return Err("Insufficient data".to_string());
    }
    let bytes: [u8; 4] = [source[offset], source[offset + 1], source[offset + 2], source[offset + 3]];
    Ok((i32::from_le_bytes(bytes), offset + 4))
}

fn write_i32(target: &mut Vec<u8>, value: i32) {
    target.extend_from_slice(&value.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constraint_creation() {
        let driver = BinMDataXtdConstraintDriver::new(None);
        let constraint = driver.new_empty();
        assert_eq!(constraint.value_index, None);
        assert_eq!(constraint.geometries.len(), 0);
        assert!(!constraint.verified);
    }

    #[test]
    fn test_constraint_with_value() {
        let constraint = MockConstraint::new().with_value(5);
        assert_eq!(constraint.value_index, Some(5));
    }

    #[test]
    fn test_constraint_with_geometries() {
        let constraint = MockConstraint::new()
            .with_geometry(10)
            .with_geometry(20);
        assert_eq!(constraint.geometries.len(), 2);
        assert_eq!(constraint.geometries[0], 10);
        assert_eq!(constraint.geometries[1], 20);
    }

    #[test]
    fn test_serialize_deserialize() {
        let driver = BinMDataXtdConstraintDriver::new(None);
        let mut reloc_table = HashMap::new();

        let original = MockConstraint::new()
            .with_value(5)
            .with_geometry(10)
            .with_plane(15)
            .with_type(2)
            .with_flags(true, false, true);

        let mut buffer = Vec::new();
        driver.paste_write(&original, &mut buffer, &mut reloc_table);

        let mut reloc_table2 = HashMap::new();
        let (deserialized, _) = driver.paste_read(&buffer, &mut reloc_table2).unwrap();

        assert_eq!(deserialized.value_index, original.value_index);
        assert_eq!(deserialized.geometries, original.geometries);
        assert_eq!(deserialized.plane_index, original.plane_index);
        assert_eq!(deserialized.constraint_type, original.constraint_type);
        assert_eq!(deserialized.verified, original.verified);
        assert_eq!(deserialized.inverted, original.inverted);
        assert_eq!(deserialized.reversed, original.reversed);
    }

    #[test]
    fn test_serialize_deserialize_multiple_geometries() {
        let driver = BinMDataXtdConstraintDriver::new(None);
        let mut reloc_table = HashMap::new();

        let original = MockConstraint::new()
            .with_geometry(1)
            .with_geometry(2)
            .with_geometry(3)
            .with_geometry(4);

        let mut buffer = Vec::new();
        driver.paste_write(&original, &mut buffer, &mut reloc_table);

        let mut reloc_table2 = HashMap::new();
        let (deserialized, _) = driver.paste_read(&buffer, &mut reloc_table2).unwrap();

        assert_eq!(deserialized.geometries.len(), 4);
        assert_eq!(deserialized.geometries, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_flags_encoding() {
        let driver = BinMDataXtdConstraintDriver::new(None);
        let mut reloc_table = HashMap::new();

        let constraint = MockConstraint::new()
            .with_flags(true, true, false);

        let mut buffer = Vec::new();
        driver.paste_write(&constraint, &mut buffer, &mut reloc_table);

        let mut reloc_table2 = HashMap::new();
        let (deserialized, _) = driver.paste_read(&buffer, &mut reloc_table2).unwrap();

        assert!(deserialized.verified);
        assert!(deserialized.inverted);
        assert!(!deserialized.reversed);
    }

    #[test]
    fn test_read_write_round_trip_empty() {
        let driver = BinMDataXtdConstraintDriver::new(None);
        let mut reloc_table = HashMap::new();

        let original = MockConstraint::new();

        let mut buffer = Vec::new();
        driver.paste_write(&original, &mut buffer, &mut reloc_table);

        let mut reloc_table2 = HashMap::new();
        let (deserialized, _) = driver.paste_read(&buffer, &mut reloc_table2).unwrap();

        assert_eq!(deserialized.value_index, None);
        assert_eq!(deserialized.geometries.len(), 0);
        assert_eq!(deserialized.plane_index, None);
    }
}
