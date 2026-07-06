// FILE: xml_t_obj_drivers_xyz_driver.rs
// occt: XmlTObjDrivers_XYZDriver

/// XML driver for XYZ coordinate data in TObj persistence.
/// Handles serialization/deserialization of 3D point and vector data.
pub struct XmlTObjDriversXYZDriver {
    version: i32,
    precision: usize,
}

impl XmlTObjDriversXYZDriver {
    /// Create a new XYZ driver with default precision.
    pub fn new() -> Self {
        XmlTObjDriversXYZDriver {
            version: 1,
            precision: 6,
        }
    }

    /// Create an XYZ driver with specified decimal precision.
    pub fn with_precision(precision: usize) -> Self {
        XmlTObjDriversXYZDriver {
            version: 1,
            precision,
        }
    }

    /// Get the driver version.
    pub fn version(&self) -> i32 {
        self.version
    }

    /// Get the coordinate precision (decimal places).
    pub fn precision(&self) -> usize {
        self.precision
    }

    /// Write a 3D point (x, y, z) to XML.
    pub fn write_point(&self, x: f64, y: f64, z: f64) -> String {
        let prec = self.precision;
        format!(
            "<Point x=\"{:.prec$}\" y=\"{:.prec$}\" z=\"{:.prec$}\"/>",
            x, y, z,
            prec = prec
        )
    }

    /// Read a 3D point from XML.
    /// Returns (x, y, z) or error.
    pub fn read_point(&self, xml: &str) -> Result<(f64, f64, f64), String> {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        let mut count = 0;

        for part in xml.split_whitespace() {
            if let Some(xval) = part.strip_prefix("x=\"").and_then(|s| s.strip_suffix("\"")) {
                x = xval.parse::<f64>().map_err(|_| "Invalid x coordinate".to_string())?;
                count += 1;
            } else if let Some(yval) = part.strip_prefix("y=\"").and_then(|s| s.strip_suffix("\"")) {
                y = yval.parse::<f64>().map_err(|_| "Invalid y coordinate".to_string())?;
                count += 1;
            } else if let Some(zval) = part.strip_prefix("z=\"").and_then(|s| s.strip_suffix("\"")) {
                z = zval.parse::<f64>().map_err(|_| "Invalid z coordinate".to_string())?;
                count += 1;
            }
        }

        if count != 3 {
            return Err("Missing coordinates".to_string());
        }

        Ok((x, y, z))
    }

    /// Write a 3D vector (dx, dy, dz) to XML.
    pub fn write_vector(&self, dx: f64, dy: f64, dz: f64) -> String {
        let prec = self.precision;
        format!(
            "<Vector dx=\"{:.prec$}\" dy=\"{:.prec$}\" dz=\"{:.prec$}\"/>",
            dx, dy, dz,
            prec = prec
        )
    }

    /// Read a 3D vector from XML.
    /// Returns (dx, dy, dz) or error.
    pub fn read_vector(&self, xml: &str) -> Result<(f64, f64, f64), String> {
        let mut dx = 0.0;
        let mut dy = 0.0;
        let mut dz = 0.0;
        let mut count = 0;

        for part in xml.split_whitespace() {
            if let Some(dxval) = part.strip_prefix("dx=\"").and_then(|s| s.strip_suffix("\"")) {
                dx = dxval.parse::<f64>().map_err(|_| "Invalid dx coordinate".to_string())?;
                count += 1;
            } else if let Some(dyval) = part.strip_prefix("dy=\"").and_then(|s| s.strip_suffix("\"")) {
                dy = dyval.parse::<f64>().map_err(|_| "Invalid dy coordinate".to_string())?;
                count += 1;
            } else if let Some(dzval) = part.strip_prefix("dz=\"").and_then(|s| s.strip_suffix("\"")) {
                dz = dzval.parse::<f64>().map_err(|_| "Invalid dz coordinate".to_string())?;
                count += 1;
            }
        }

        if count != 3 {
            return Err("Missing vector components".to_string());
        }

        Ok((dx, dy, dz))
    }

    /// Calculate distance between two points.
    pub fn distance(&self, x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
        let dx = x2 - x1;
        let dy = y2 - y1;
        let dz = z2 - z1;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Calculate vector magnitude.
    pub fn magnitude(&self, dx: f64, dy: f64, dz: f64) -> f64 {
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

impl Default for XmlTObjDriversXYZDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_version() {
        let driver = XmlTObjDriversXYZDriver::new();
        assert_eq!(driver.version(), 1);
    }

    #[test]
    fn test_default_precision() {
        let driver = XmlTObjDriversXYZDriver::new();
        assert_eq!(driver.precision(), 6);
    }

    #[test]
    fn test_with_precision() {
        let driver = XmlTObjDriversXYZDriver::with_precision(3);
        assert_eq!(driver.precision(), 3);
    }

    #[test]
    fn test_write_point() {
        let driver = XmlTObjDriversXYZDriver::new();
        let xml = driver.write_point(1.5, 2.5, 3.5);
        assert!(xml.contains("x=\"1.5"));
        assert!(xml.contains("y=\"2.5"));
        assert!(xml.contains("z=\"3.5"));
    }

    #[test]
    fn test_write_point_precision() {
        let driver = XmlTObjDriversXYZDriver::with_precision(2);
        let xml = driver.write_point(1.234, 2.567, 3.891);
        assert!(xml.contains("x=\"1.23"));
        assert!(xml.contains("y=\"2.57"));
    }

    #[test]
    fn test_read_point_valid() {
        let driver = XmlTObjDriversXYZDriver::new();
        let xml = "<Point x=\"1.0\" y=\"2.0\" z=\"3.0\"/>";
        let result = driver.read_point(xml);
        assert!(result.is_ok());

        let (x, y, z) = result.unwrap();
        assert_eq!(x, 1.0);
        assert_eq!(y, 2.0);
        assert_eq!(z, 3.0);
    }

    #[test]
    fn test_read_point_missing_coordinate() {
        let driver = XmlTObjDriversXYZDriver::new();
        let xml = "<Point x=\"1.0\" y=\"2.0\"/>";
        let result = driver.read_point(xml);
        assert!(result.is_err());
    }

    #[test]
    fn test_read_point_invalid_value() {
        let driver = XmlTObjDriversXYZDriver::new();
        let xml = "<Point x=\"invalid\" y=\"2.0\" z=\"3.0\"/>";
        let result = driver.read_point(xml);
        assert!(result.is_err());
    }

    #[test]
    fn test_write_vector() {
        let driver = XmlTObjDriversXYZDriver::new();
        let xml = driver.write_vector(0.5, 1.5, 2.5);
        assert!(xml.contains("dx=\"0.5"));
        assert!(xml.contains("dy=\"1.5"));
        assert!(xml.contains("dz=\"2.5"));
    }

    #[test]
    fn test_read_vector_valid() {
        let driver = XmlTObjDriversXYZDriver::new();
        let xml = "<Vector dx=\"1.0\" dy=\"2.0\" dz=\"3.0\"/>";
        let result = driver.read_vector(xml);
        assert!(result.is_ok());

        let (dx, dy, dz) = result.unwrap();
        assert_eq!(dx, 1.0);
        assert_eq!(dy, 2.0);
        assert_eq!(dz, 3.0);
    }

    #[test]
    fn test_read_vector_missing_component() {
        let driver = XmlTObjDriversXYZDriver::new();
        let xml = "<Vector dx=\"1.0\" dy=\"2.0\"/>";
        let result = driver.read_vector(xml);
        assert!(result.is_err());
    }

    #[test]
    fn test_distance_zero() {
        let driver = XmlTObjDriversXYZDriver::new();
        let dist = driver.distance(1.0, 2.0, 3.0, 1.0, 2.0, 3.0);
        assert!(dist < 0.0001);
    }

    #[test]
    fn test_distance_unit() {
        let driver = XmlTObjDriversXYZDriver::new();
        let dist = driver.distance(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        assert!((dist - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_distance_3_4_5_triangle() {
        let driver = XmlTObjDriversXYZDriver::new();
        let dist = driver.distance(0.0, 0.0, 0.0, 3.0, 4.0, 0.0);
        assert!((dist - 5.0).abs() < 0.0001);
    }

    #[test]
    fn test_magnitude_unit() {
        let driver = XmlTObjDriversXYZDriver::new();
        let mag = driver.magnitude(1.0, 0.0, 0.0);
        assert!((mag - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_magnitude_zero() {
        let driver = XmlTObjDriversXYZDriver::new();
        let mag = driver.magnitude(0.0, 0.0, 0.0);
        assert!(mag < 0.0001);
    }

    #[test]
    fn test_roundtrip_point() {
        let driver = XmlTObjDriversXYZDriver::new();
        let xml = driver.write_point(1.23456, 2.34567, 3.45678);
        let (x, y, z) = driver.read_point(&xml).unwrap();
        assert!((x - 1.23456).abs() < 0.0001);
        assert!((y - 2.34567).abs() < 0.0001);
        assert!((z - 3.45678).abs() < 0.0001);
    }

    #[test]
    fn test_roundtrip_vector() {
        let driver = XmlTObjDriversXYZDriver::new();
        let xml = driver.write_vector(0.5, 1.5, 2.5);
        let (dx, dy, dz) = driver.read_vector(&xml).unwrap();
        assert_eq!(dx, 0.5);
        assert_eq!(dy, 1.5);
        assert_eq!(dz, 2.5);
    }
}
