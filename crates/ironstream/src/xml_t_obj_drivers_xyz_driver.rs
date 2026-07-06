// FILE: xml_t_obj_drivers_xyz_driver.rs
// occt: XmlTObjDrivers_XYZDriver
//
// Port of OCCT XmlTObjDrivers_XYZDriver (TObj XML drivers).
// Stores a TObj_TXYZ attribute (a gp_XYZ triple) as the XML attributes
// "X", "Y" and "Z"; retrieval fails if any coordinate is missing or
// not a valid real. XmlObjMgt plumbing is modeled locally.

use std::collections::HashMap;

/// DOM attribute names (IMPLEMENT_DOMSTRING in OCCT).
pub const ATTR_COORD_X: &str = "X";
pub const ATTR_COORD_Y: &str = "Y";
pub const ATTR_COORD_Z: &str = "Z";

/// Local model of an XmlObjMgt_Element.
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

    pub fn set_attribute(&mut self, name: &str, value: &str) {
        self.attributes.insert(name.to_string(), value.to_string());
    }
}

/// XmlObjMgt::GetReal analogue: parses a real, false on failure.
pub fn get_real(s: &str) -> Option<f64> {
    s.trim().parse::<f64>().ok()
}

/// Local model of gp_XYZ.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct GpXyz {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl GpXyz {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        GpXyz { x, y, z }
    }
}

/// Local model of the TObj_TXYZ attribute.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct TObjTXyz {
    xyz: GpXyz,
}

impl TObjTXyz {
    pub fn new() -> Self {
        Self::default()
    }

    /// TObj_TXYZ::Set.
    pub fn set(&mut self, xyz: GpXyz) {
        self.xyz = xyz;
    }

    /// TObj_TXYZ::Get.
    pub fn get(&self) -> GpXyz {
        self.xyz
    }
}

/// XmlMDF_ADriver for TObj_TXYZ.
#[derive(Debug, Default)]
pub struct XmlTObjDriversXYZDriver;

impl XmlTObjDriversXYZDriver {
    pub fn new() -> Self {
        Self
    }

    /// OCCT NewEmpty.
    pub fn new_empty(&self) -> TObjTXyz {
        TObjTXyz::new()
    }

    /// OCCT Paste (persistent -> transient): reads X, Y, Z attributes;
    /// false when any of them is missing or not a real.
    pub fn paste_from_xml(&self, source: &XmlElement, target: &mut TObjTXyz) -> bool {
        let x = match source.get_attribute(ATTR_COORD_X).and_then(get_real) {
            Some(v) => v,
            None => return false,
        };
        let y = match source.get_attribute(ATTR_COORD_Y).and_then(get_real) {
            Some(v) => v,
            None => return false,
        };
        let z = match source.get_attribute(ATTR_COORD_Z).and_then(get_real) {
            Some(v) => v,
            None => return false,
        };
        target.set(GpXyz::new(x, y, z));
        true
    }

    /// OCCT Paste (transient -> persistent): writes X, Y, Z attributes
    /// with full round-trip precision.
    pub fn paste_to_xml(&self, source: &TObjTXyz, target: &mut XmlElement) {
        let xyz = source.get();
        target.set_attribute(ATTR_COORD_X, &xyz.x.to_string());
        target.set_attribute(ATTR_COORD_Y, &xyz.y.to_string());
        target.set_attribute(ATTR_COORD_Z, &xyz.z.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_empty() {
        let driver = XmlTObjDriversXYZDriver::new();
        let t = driver.new_empty();
        assert_eq!(t.get(), GpXyz::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_write_point() {
        let driver = XmlTObjDriversXYZDriver::new();
        let mut t = TObjTXyz::new();
        t.set(GpXyz::new(1.5, -2.25, 3.0));

        let mut el = XmlElement::new();
        driver.paste_to_xml(&t, &mut el);
        assert_eq!(el.get_attribute("X"), Some("1.5"));
        assert_eq!(el.get_attribute("Y"), Some("-2.25"));
        assert_eq!(el.get_attribute("Z"), Some("3"));
    }

    #[test]
    fn test_read_point_valid() {
        let driver = XmlTObjDriversXYZDriver::new();
        let mut el = XmlElement::new();
        el.set_attribute("X", "10.5");
        el.set_attribute("Y", "20.25");
        el.set_attribute("Z", "-30.125");

        let mut t = TObjTXyz::new();
        assert!(driver.paste_from_xml(&el, &mut t));
        assert_eq!(t.get(), GpXyz::new(10.5, 20.25, -30.125));
    }

    #[test]
    fn test_read_missing_coordinate_fails() {
        let driver = XmlTObjDriversXYZDriver::new();
        let mut el = XmlElement::new();
        el.set_attribute("X", "1.0");
        el.set_attribute("Y", "2.0");
        // Z missing.

        let mut t = TObjTXyz::new();
        assert!(!driver.paste_from_xml(&el, &mut t));
        // Target untouched on failure.
        assert_eq!(t.get(), GpXyz::default());
    }

    #[test]
    fn test_read_invalid_real_fails() {
        let driver = XmlTObjDriversXYZDriver::new();
        let mut el = XmlElement::new();
        el.set_attribute("X", "1.0");
        el.set_attribute("Y", "not-a-number");
        el.set_attribute("Z", "3.0");

        let mut t = TObjTXyz::new();
        assert!(!driver.paste_from_xml(&el, &mut t));
    }

    #[test]
    fn test_read_scientific_notation() {
        let driver = XmlTObjDriversXYZDriver::new();
        let mut el = XmlElement::new();
        el.set_attribute("X", "1e-3");
        el.set_attribute("Y", "-2.5E2");
        el.set_attribute("Z", "0");

        let mut t = TObjTXyz::new();
        assert!(driver.paste_from_xml(&el, &mut t));
        assert_eq!(t.get(), GpXyz::new(0.001, -250.0, 0.0));
    }

    #[test]
    fn test_roundtrip_exact() {
        let driver = XmlTObjDriversXYZDriver::new();
        let mut src = TObjTXyz::new();
        src.set(GpXyz::new(
            1.2345678901234567,
            -9.876543210987654e-10,
            3.141592653589793,
        ));

        let mut el = XmlElement::new();
        driver.paste_to_xml(&src, &mut el);

        let mut dst = TObjTXyz::new();
        assert!(driver.paste_from_xml(&el, &mut dst));
        // Shortest round-trip formatting restores the exact values.
        assert_eq!(dst.get(), src.get());
    }
}
