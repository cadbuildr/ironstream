// FILE: bin_m_data_xtd_pattern_std_driver.rs
// occt: BinMDataXtd_PatternStdDriver

use std::collections::HashMap;

/// Binary serialization/deserialization driver for pattern standard attributes.
/// Handles persistence of pattern data including signature, axes, values, instances, and mirror.
pub struct BinMDataXtdPatternStdDriver {
    _message_driver: Option<String>,
}

impl BinMDataXtdPatternStdDriver {
    pub fn new(_message_driver: Option<String>) -> Self {
        BinMDataXtdPatternStdDriver {
            _message_driver,
        }
    }

    /// Create a new empty pattern standard attribute.
    pub fn new_empty(&self) -> MockPatternStd {
        MockPatternStd::new()
    }

    /// Deserialize pattern from binary source.
    pub fn paste_read(
        &self,
        source: &[u8],
        reloc_table: &mut HashMap<i32, MockAttribute>,
    ) -> Result<(MockPatternStd, usize), String> {
        let mut offset = 0;

        // Read signature
        let (signature, next_offset) = read_i32(source, offset)?;
        offset = next_offset;

        let mut pattern = MockPatternStd::new();

        if signature == 0 {
            return Ok((pattern, offset));
        }

        if signature < 1 || signature > 5 {
            return Err("Invalid signature".to_string());
        }

        pattern.signature = signature;

        // Read reversed flags
        let (rev_flags, next_offset) = read_i32(source, offset)?;
        offset = next_offset;

        pattern.axis1_reversed = (rev_flags & 1) != 0;
        pattern.axis2_reversed = (rev_flags & 2) != 0;

        if signature == 5 {
            // mirror
            let (plane_nb, next_offset) = read_i32(source, offset)?;
            offset = next_offset;

            if plane_nb > 0 {
                if !reloc_table.contains_key(&plane_nb) {
                    reloc_table.insert(plane_nb, MockAttribute::NamedShape);
                }
                pattern.mirror = Some(plane_nb);
            }
        } else {
            // axis 1
            let (axis1_nb, next_offset) = read_i32(source, offset)?;
            offset = next_offset;

            if axis1_nb > 0 {
                if !reloc_table.contains_key(&axis1_nb) {
                    reloc_table.insert(axis1_nb, MockAttribute::NamedShape);
                }
                pattern.axis1 = Some(axis1_nb);
            }

            // value 1
            let (value1_nb, next_offset) = read_i32(source, offset)?;
            offset = next_offset;

            if value1_nb > 0 {
                if !reloc_table.contains_key(&value1_nb) {
                    reloc_table.insert(value1_nb, MockAttribute::Real(0));
                }
                pattern.value1 = Some(value1_nb);
            }

            // number of instances 1
            let (nb_inst1_nb, next_offset) = read_i32(source, offset)?;
            offset = next_offset;

            if nb_inst1_nb > 0 {
                if !reloc_table.contains_key(&nb_inst1_nb) {
                    reloc_table.insert(nb_inst1_nb, MockAttribute::Integer(0));
                }
                pattern.nb_instances1 = Some(nb_inst1_nb);
            }

            if signature > 2 {
                // axis 2
                let (axis2_nb, next_offset) = read_i32(source, offset)?;
                offset = next_offset;

                if axis2_nb > 0 {
                    if !reloc_table.contains_key(&axis2_nb) {
                        reloc_table.insert(axis2_nb, MockAttribute::NamedShape);
                    }
                    pattern.axis2 = Some(axis2_nb);
                }

                // value 2
                let (value2_nb, next_offset) = read_i32(source, offset)?;
                offset = next_offset;

                if value2_nb > 0 {
                    if !reloc_table.contains_key(&value2_nb) {
                        reloc_table.insert(value2_nb, MockAttribute::Real(0));
                    }
                    pattern.value2 = Some(value2_nb);
                }

                // number of instances 2
                let (nb_inst2_nb, next_offset) = read_i32(source, offset)?;
                offset = next_offset;

                if nb_inst2_nb > 0 {
                    if !reloc_table.contains_key(&nb_inst2_nb) {
                        reloc_table.insert(nb_inst2_nb, MockAttribute::Integer(0));
                    }
                    pattern.nb_instances2 = Some(nb_inst2_nb);
                }
            }
        }

        Ok((pattern, offset))
    }

    /// Serialize pattern to binary target.
    pub fn paste_write(
        &self,
        pattern: &MockPatternStd,
        target: &mut Vec<u8>,
        _reloc_table: &mut HashMap<MockAttribute, i32>,
    ) {
        // Write signature
        let mut sig = pattern.signature;
        if sig < 1 || sig > 5 {
            sig = 0;
        }
        write_i32(target, sig);

        if sig == 0 {
            return;
        }

        // Write reversed flags
        let mut rev_flags = 0;
        if pattern.axis1_reversed {
            rev_flags |= 1;
        }
        if pattern.axis2_reversed {
            rev_flags |= 2;
        }
        write_i32(target, rev_flags);

        if sig == 5 {
            // mirror
            let mirror_nb = pattern.mirror.unwrap_or(-1);
            write_i32(target, mirror_nb);
        } else {
            // axis 1
            let axis1_nb = pattern.axis1.unwrap_or(-1);
            write_i32(target, axis1_nb);

            // value 1
            let value1_nb = pattern.value1.unwrap_or(-1);
            write_i32(target, value1_nb);

            // instances 1
            let nb_inst1_nb = pattern.nb_instances1.unwrap_or(-1);
            write_i32(target, nb_inst1_nb);

            if sig > 2 {
                // axis 2
                let axis2_nb = pattern.axis2.unwrap_or(-1);
                write_i32(target, axis2_nb);

                // value 2
                let value2_nb = pattern.value2.unwrap_or(-1);
                write_i32(target, value2_nb);

                // instances 2
                let nb_inst2_nb = pattern.nb_instances2.unwrap_or(-1);
                write_i32(target, nb_inst2_nb);
            }
        }
    }
}

/// Mock pattern standard attribute for testing serialization.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MockPatternStd {
    pub signature: i32,
    pub axis1_reversed: bool,
    pub axis2_reversed: bool,
    pub mirror: Option<i32>,
    pub axis1: Option<i32>,
    pub value1: Option<i32>,
    pub nb_instances1: Option<i32>,
    pub axis2: Option<i32>,
    pub value2: Option<i32>,
    pub nb_instances2: Option<i32>,
}

impl MockPatternStd {
    pub fn new() -> Self {
        MockPatternStd {
            signature: 0,
            axis1_reversed: false,
            axis2_reversed: false,
            mirror: None,
            axis1: None,
            value1: None,
            nb_instances1: None,
            axis2: None,
            value2: None,
            nb_instances2: None,
        }
    }

    pub fn with_signature(mut self, sig: i32) -> Self {
        self.signature = sig;
        self
    }

    pub fn with_reversed_flags(mut self, axis1: bool, axis2: bool) -> Self {
        self.axis1_reversed = axis1;
        self.axis2_reversed = axis2;
        self
    }

    pub fn with_mirror(mut self, idx: i32) -> Self {
        self.mirror = Some(idx);
        self
    }

    pub fn with_axis1_pattern(mut self, axis: i32, value: i32, instances: i32) -> Self {
        self.axis1 = Some(axis);
        self.value1 = Some(value);
        self.nb_instances1 = Some(instances);
        self
    }

    pub fn with_axis2_pattern(mut self, axis: i32, value: i32, instances: i32) -> Self {
        self.axis2 = Some(axis);
        self.value2 = Some(value);
        self.nb_instances2 = Some(instances);
        self
    }
}

impl Default for MockPatternStd {
    fn default() -> Self {
        MockPatternStd::new()
    }
}

/// Mock attribute types for relocation table.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum MockAttribute {
    Real(i32),
    Integer(i32),
    NamedShape,
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
    fn test_pattern_creation() {
        let driver = BinMDataXtdPatternStdDriver::new(None);
        let pattern = driver.new_empty();
        assert_eq!(pattern.signature, 0);
        assert!(!pattern.axis1_reversed);
    }

    #[test]
    fn test_empty_pattern_serialize() {
        let driver = BinMDataXtdPatternStdDriver::new(None);
        let mut reloc_table = HashMap::new();

        let pattern = MockPatternStd::new();

        let mut buffer = Vec::new();
        driver.paste_write(&pattern, &mut buffer, &mut reloc_table);

        // Should only write signature (0)
        let mut reloc_table2 = HashMap::new();
        let (deserialized, _) = driver.paste_read(&buffer, &mut reloc_table2).unwrap();

        assert_eq!(deserialized.signature, 0);
    }

    #[test]
    fn test_mirror_pattern_serialize() {
        let driver = BinMDataXtdPatternStdDriver::new(None);
        let mut reloc_table = HashMap::new();

        let pattern = MockPatternStd::new()
            .with_signature(5)
            .with_mirror(10);

        let mut buffer = Vec::new();
        driver.paste_write(&pattern, &mut buffer, &mut reloc_table);

        let mut reloc_table2 = HashMap::new();
        let (deserialized, _) = driver.paste_read(&buffer, &mut reloc_table2).unwrap();

        assert_eq!(deserialized.signature, 5);
        assert_eq!(deserialized.mirror, Some(10));
    }

    #[test]
    fn test_linear_pattern_serialize() {
        let driver = BinMDataXtdPatternStdDriver::new(None);
        let mut reloc_table = HashMap::new();

        let pattern = MockPatternStd::new()
            .with_signature(1)
            .with_axis1_pattern(5, 15, 25);

        let mut buffer = Vec::new();
        driver.paste_write(&pattern, &mut buffer, &mut reloc_table);

        let mut reloc_table2 = HashMap::new();
        let (deserialized, _) = driver.paste_read(&buffer, &mut reloc_table2).unwrap();

        assert_eq!(deserialized.signature, 1);
        assert_eq!(deserialized.axis1, Some(5));
        assert_eq!(deserialized.value1, Some(15));
        assert_eq!(deserialized.nb_instances1, Some(25));
    }

    #[test]
    fn test_rectangular_pattern_serialize() {
        let driver = BinMDataXtdPatternStdDriver::new(None);
        let mut reloc_table = HashMap::new();

        let pattern = MockPatternStd::new()
            .with_signature(3)
            .with_axis1_pattern(5, 15, 25)
            .with_axis2_pattern(10, 20, 30);

        let mut buffer = Vec::new();
        driver.paste_write(&pattern, &mut buffer, &mut reloc_table);

        let mut reloc_table2 = HashMap::new();
        let (deserialized, _) = driver.paste_read(&buffer, &mut reloc_table2).unwrap();

        assert_eq!(deserialized.signature, 3);
        assert_eq!(deserialized.axis1, Some(5));
        assert_eq!(deserialized.axis2, Some(10));
        assert_eq!(deserialized.nb_instances1, Some(25));
        assert_eq!(deserialized.nb_instances2, Some(30));
    }

    #[test]
    fn test_reversed_flags() {
        let driver = BinMDataXtdPatternStdDriver::new(None);
        let mut reloc_table = HashMap::new();

        let pattern = MockPatternStd::new()
            .with_signature(1)
            .with_reversed_flags(true, false)
            .with_axis1_pattern(5, 15, 25);

        let mut buffer = Vec::new();
        driver.paste_write(&pattern, &mut buffer, &mut reloc_table);

        let mut reloc_table2 = HashMap::new();
        let (deserialized, _) = driver.paste_read(&buffer, &mut reloc_table2).unwrap();

        assert!(deserialized.axis1_reversed);
        assert!(!deserialized.axis2_reversed);
    }
}
