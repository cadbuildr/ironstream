// FILE: xml_t_obj_drivers_int_sparse_array_driver.rs
// occt: XmlTObjDrivers_IntSparseArrayDriver
//
// Port of OCCT XmlTObjDrivers_IntSparseArrayDriver
// (DataExchange/TKTObjDRAW.. TObj XML drivers). Serializes a
// TObj_TIntSparseArray as XML attribute pairs
//   itemId_1 / itemValue_1, itemId_2 / itemValue_2, ...
// terminated by a pair with itemId_N = 0.
// TDF/XmlObjMgt plumbing is modeled locally.

use std::collections::{BTreeMap, HashMap};

const ITEM_ID: &str = "itemId_";
const ITEM_VALUE: &str = "itemValue_";

/// Local model of an XmlObjMgt_Element: XML attributes by name.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct XmlElement {
    attributes: HashMap<String, String>,
}

impl XmlElement {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_attribute(&self, name: &str) -> Option<&str> {
        self.attributes.get(name).map(|s| s.as_str())
    }

    pub fn set_attribute_int(&mut self, name: &str, value: i32) {
        self.attributes.insert(name.to_string(), value.to_string());
    }
}

/// Local model of TObj_TIntSparseArray: sparse map of positive IDs to
/// integer values; absent IDs read as 0.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TObjTIntSparseArray {
    data: BTreeMap<usize, i32>,
}

impl TObjTIntSparseArray {
    pub fn new() -> Self {
        Self::default()
    }

    /// TObj_TIntSparseArray::SetValue (id must be positive).
    pub fn set_value(&mut self, id: usize, value: i32) {
        assert!(id > 0, "sparse array IDs are positive");
        if value == 0 {
            self.data.remove(&id);
        } else {
            self.data.insert(id, value);
        }
    }

    /// Value at id; 0 when unset.
    pub fn value(&self, id: usize) -> i32 {
        *self.data.get(&id).unwrap_or(&0)
    }

    /// Iterator over stored (id, value) pairs in increasing id order.
    pub fn iter(&self) -> impl Iterator<Item = (usize, i32)> + '_ {
        self.data.iter().map(|(&k, &v)| (k, v))
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

/// XmlMDF_ADriver for TObj_TIntSparseArray.
#[derive(Debug, Default)]
pub struct XmlTObjDriversIntSparseArrayDriver;

impl XmlTObjDriversIntSparseArrayDriver {
    pub fn new() -> Self {
        Self
    }

    /// OCCT NewEmpty.
    pub fn new_empty(&self) -> TObjTIntSparseArray {
        TObjTIntSparseArray::new()
    }

    /// OCCT Paste (persistent -> transient): reads pairs
    /// (itemId_i, itemValue_i) while itemId_i is an integer != 0.
    pub fn paste_from_xml(&self, source: &XmlElement, target: &mut TObjTIntSparseArray) -> bool {
        let mut i = 1usize;
        loop {
            let id_attr = format!("{}{}", ITEM_ID, i);
            let id = match source
                .get_attribute(&id_attr)
                .and_then(|s| s.parse::<i32>().ok())
            {
                Some(v) => v,
                None => break, // not an integer value -> stop
            };
            if id == 0 {
                break; // terminator
            }
            let val_attr = format!("{}{}", ITEM_VALUE, i);
            if let Some(value) = source
                .get_attribute(&val_attr)
                .and_then(|s| s.parse::<i32>().ok())
            {
                target.set_value(id as usize, value);
            }
            i += 1;
        }
        true
    }

    /// OCCT Paste (transient -> persistent): writes only non-null values
    /// as pairs (ID, value) and terminates the list with ID=0, value=0.
    pub fn paste_to_xml(&self, source: &TObjTIntSparseArray, target: &mut XmlElement) {
        let mut i = 1usize;
        for (id, value) in source.iter() {
            if value == 0 {
                continue;
            }
            target.set_attribute_int(&format!("{}{}", ITEM_ID, i), id as i32);
            target.set_attribute_int(&format!("{}{}", ITEM_VALUE, i), value);
            i += 1;
        }
        // write last (terminator) item
        target.set_attribute_int(&format!("{}{}", ITEM_ID, i), 0);
        target.set_attribute_int(&format!("{}{}", ITEM_VALUE, i), 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_empty() {
        let driver = XmlTObjDriversIntSparseArrayDriver::new();
        let arr = driver.new_empty();
        assert_eq!(arr.size(), 0);
        assert_eq!(arr.value(5), 0);
    }

    #[test]
    fn test_sparse_array_set_value() {
        let mut arr = TObjTIntSparseArray::new();
        arr.set_value(5, 42);
        assert_eq!(arr.value(5), 42);
        assert_eq!(arr.size(), 1);
        // Setting to 0 clears the slot.
        arr.set_value(5, 0);
        assert_eq!(arr.value(5), 0);
        assert_eq!(arr.size(), 0);
    }

    #[test]
    fn test_write_empty_array_writes_terminator_only() {
        let driver = XmlTObjDriversIntSparseArrayDriver::new();
        let arr = TObjTIntSparseArray::new();
        let mut el = XmlElement::new();
        driver.paste_to_xml(&arr, &mut el);
        assert_eq!(el.get_attribute("itemId_1"), Some("0"));
        assert_eq!(el.get_attribute("itemValue_1"), Some("0"));
        assert_eq!(el.get_attribute("itemId_2"), None);
    }

    #[test]
    fn test_write_single_entry() {
        let driver = XmlTObjDriversIntSparseArrayDriver::new();
        let mut arr = TObjTIntSparseArray::new();
        arr.set_value(5, 42);

        let mut el = XmlElement::new();
        driver.paste_to_xml(&arr, &mut el);
        assert_eq!(el.get_attribute("itemId_1"), Some("5"));
        assert_eq!(el.get_attribute("itemValue_1"), Some("42"));
        // Terminator follows.
        assert_eq!(el.get_attribute("itemId_2"), Some("0"));
        assert_eq!(el.get_attribute("itemValue_2"), Some("0"));
    }

    #[test]
    fn test_read_sparse_array_single_entry() {
        let driver = XmlTObjDriversIntSparseArrayDriver::new();
        let mut el = XmlElement::new();
        el.set_attribute_int("itemId_1", 5);
        el.set_attribute_int("itemValue_1", 42);
        el.set_attribute_int("itemId_2", 0);
        el.set_attribute_int("itemValue_2", 0);

        let mut arr = TObjTIntSparseArray::new();
        assert!(driver.paste_from_xml(&el, &mut arr));
        assert_eq!(arr.value(5), 42);
        assert_eq!(arr.size(), 1);
    }

    #[test]
    fn test_read_stops_at_terminator() {
        let driver = XmlTObjDriversIntSparseArrayDriver::new();
        let mut el = XmlElement::new();
        el.set_attribute_int("itemId_1", 3);
        el.set_attribute_int("itemValue_1", 30);
        el.set_attribute_int("itemId_2", 0);
        el.set_attribute_int("itemValue_2", 0);
        // Attributes after the terminator must be ignored.
        el.set_attribute_int("itemId_3", 9);
        el.set_attribute_int("itemValue_3", 90);

        let mut arr = TObjTIntSparseArray::new();
        assert!(driver.paste_from_xml(&el, &mut arr));
        assert_eq!(arr.value(3), 30);
        assert_eq!(arr.value(9), 0);
        assert_eq!(arr.size(), 1);
    }

    #[test]
    fn test_read_missing_id_attribute_stops() {
        let driver = XmlTObjDriversIntSparseArrayDriver::new();
        let el = XmlElement::new(); // no attributes at all
        let mut arr = TObjTIntSparseArray::new();
        assert!(driver.paste_from_xml(&el, &mut arr));
        assert_eq!(arr.size(), 0);
    }

    #[test]
    fn test_roundtrip_multiple_entries() {
        let driver = XmlTObjDriversIntSparseArrayDriver::new();
        let mut orig = TObjTIntSparseArray::new();
        orig.set_value(1, 100);
        orig.set_value(3, 200);
        orig.set_value(7, -300);

        let mut el = XmlElement::new();
        driver.paste_to_xml(&orig, &mut el);

        let mut restored = TObjTIntSparseArray::new();
        assert!(driver.paste_from_xml(&el, &mut restored));
        assert_eq!(restored, orig);
    }
}
