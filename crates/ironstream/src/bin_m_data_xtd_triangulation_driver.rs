// FILE: bin_m_data_xtd_triangulation_driver.rs
// occt: BinMDataXtd_TriangulationDriver

/// Binary serialization/deserialization driver for triangulation attributes.
/// Handles persistence of triangulation mesh data: nodes, UV nodes, triangles, and deflection.
pub struct BinMDataXtdTriangulationDriver {
    _message_driver: Option<String>,
}

impl BinMDataXtdTriangulationDriver {
    pub fn new(_message_driver: Option<String>) -> Self {
        BinMDataXtdTriangulationDriver {
            _message_driver,
        }
    }

    /// Create a new empty triangulation attribute.
    pub fn new_empty(&self) -> MockTriangulation {
        MockTriangulation::new()
    }

    /// Deserialize triangulation from binary source.
    pub fn paste_read(
        &self,
        source: &[u8],
        offset: usize,
    ) -> Result<(MockTriangulation, usize), String> {
        let mut current_offset = offset;

        // Read number of nodes
        let (nb_nodes, next_offset) = read_i32(source, current_offset)?;
        current_offset = next_offset;

        // Read number of triangles
        let (nb_triangles, next_offset) = read_i32(source, current_offset)?;
        current_offset = next_offset;

        // Read has UV flag
        let (has_uv_int, next_offset) = read_i32(source, current_offset)?;
        current_offset = next_offset;
        let has_uv = has_uv_int != 0;

        // Read deflection
        let (deflection, next_offset) = read_f64(source, current_offset)?;
        current_offset = next_offset;

        if nb_nodes <= 0 || nb_triangles <= 0 {
            return Err("Invalid node or triangle count".to_string());
        }

        let mut triangulation = MockTriangulation {
            nodes: Vec::new(),
            uv_nodes: Vec::new(),
            triangles: Vec::new(),
            has_uv,
            deflection,
        };

        // Read 3D nodes
        for _ in 0..nb_nodes {
            let (x, next_offset) = read_f64(source, current_offset)?;
            current_offset = next_offset;

            let (y, next_offset) = read_f64(source, current_offset)?;
            current_offset = next_offset;

            let (z, next_offset) = read_f64(source, current_offset)?;
            current_offset = next_offset;

            triangulation.nodes.push([x, y, z]);
        }

        // Read 2D nodes if present
        if has_uv {
            for _ in 0..nb_nodes {
                let (x, next_offset) = read_f64(source, current_offset)?;
                current_offset = next_offset;

                let (y, next_offset) = read_f64(source, current_offset)?;
                current_offset = next_offset;

                triangulation.uv_nodes.push([x, y]);
            }
        }

        // Read triangles
        for _ in 0..nb_triangles {
            let (n1, next_offset) = read_i32(source, current_offset)?;
            current_offset = next_offset;

            let (n2, next_offset) = read_i32(source, current_offset)?;
            current_offset = next_offset;

            let (n3, next_offset) = read_i32(source, current_offset)?;
            current_offset = next_offset;

            triangulation.triangles.push([n1, n2, n3]);
        }

        Ok((triangulation, current_offset))
    }

    /// Serialize triangulation to binary target.
    pub fn paste_write(&self, triangulation: &MockTriangulation, target: &mut Vec<u8>) {
        // Write number of nodes
        write_i32(target, triangulation.nodes.len() as i32);

        // Write number of triangles
        write_i32(target, triangulation.triangles.len() as i32);

        // Write has UV flag
        write_i32(target, if triangulation.has_uv { 1 } else { 0 });

        // Write deflection
        write_f64(target, triangulation.deflection);

        // Write 3D nodes
        for node in &triangulation.nodes {
            write_f64(target, node[0]);
            write_f64(target, node[1]);
            write_f64(target, node[2]);
        }

        // Write 2D nodes if present
        if triangulation.has_uv {
            for uv_node in &triangulation.uv_nodes {
                write_f64(target, uv_node[0]);
                write_f64(target, uv_node[1]);
            }
        }

        // Write triangles
        for triangle in &triangulation.triangles {
            write_i32(target, triangle[0]);
            write_i32(target, triangle[1]);
            write_i32(target, triangle[2]);
        }
    }
}

/// Mock triangulation attribute for testing serialization.
#[derive(Clone, Debug, PartialEq)]
pub struct MockTriangulation {
    pub nodes: Vec<[f64; 3]>,
    pub uv_nodes: Vec<[f64; 2]>,
    pub triangles: Vec<[i32; 3]>,
    pub has_uv: bool,
    pub deflection: f64,
}

impl MockTriangulation {
    pub fn new() -> Self {
        MockTriangulation {
            nodes: Vec::new(),
            uv_nodes: Vec::new(),
            triangles: Vec::new(),
            has_uv: false,
            deflection: 0.0,
        }
    }

    pub fn with_node(mut self, x: f64, y: f64, z: f64) -> Self {
        self.nodes.push([x, y, z]);
        self
    }

    pub fn with_uv_node(mut self, u: f64, v: f64) -> Self {
        self.uv_nodes.push([u, v]);
        self.has_uv = true;
        self
    }

    pub fn with_triangle(mut self, n1: i32, n2: i32, n3: i32) -> Self {
        self.triangles.push([n1, n2, n3]);
        self
    }

    pub fn with_deflection(mut self, deflection: f64) -> Self {
        self.deflection = deflection;
        self
    }
}

impl Default for MockTriangulation {
    fn default() -> Self {
        MockTriangulation::new()
    }
}

fn read_i32(source: &[u8], offset: usize) -> Result<(i32, usize), String> {
    if offset + 4 > source.len() {
        return Err("Insufficient data for i32".to_string());
    }
    let bytes: [u8; 4] = [source[offset], source[offset + 1], source[offset + 2], source[offset + 3]];
    Ok((i32::from_le_bytes(bytes), offset + 4))
}

fn read_f64(source: &[u8], offset: usize) -> Result<(f64, usize), String> {
    if offset + 8 > source.len() {
        return Err("Insufficient data for f64".to_string());
    }
    let bytes: [u8; 8] = [
        source[offset],
        source[offset + 1],
        source[offset + 2],
        source[offset + 3],
        source[offset + 4],
        source[offset + 5],
        source[offset + 6],
        source[offset + 7],
    ];
    Ok((f64::from_le_bytes(bytes), offset + 8))
}

fn write_i32(target: &mut Vec<u8>, value: i32) {
    target.extend_from_slice(&value.to_le_bytes());
}

fn write_f64(target: &mut Vec<u8>, value: f64) {
    target.extend_from_slice(&value.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangulation_creation() {
        let driver = BinMDataXtdTriangulationDriver::new(None);
        let tri = driver.new_empty();
        assert_eq!(tri.nodes.len(), 0);
        assert_eq!(tri.triangles.len(), 0);
        assert!(!tri.has_uv);
    }

    #[test]
    fn test_triangulation_with_nodes() {
        let tri = MockTriangulation::new()
            .with_node(0.0, 0.0, 0.0)
            .with_node(1.0, 0.0, 0.0)
            .with_node(0.0, 1.0, 0.0);

        assert_eq!(tri.nodes.len(), 3);
        assert_eq!(tri.nodes[0], [0.0, 0.0, 0.0]);
        assert_eq!(tri.nodes[1], [1.0, 0.0, 0.0]);
        assert_eq!(tri.nodes[2], [0.0, 1.0, 0.0]);
    }

    #[test]
    fn test_triangulation_with_triangles() {
        let tri = MockTriangulation::new()
            .with_node(0.0, 0.0, 0.0)
            .with_node(1.0, 0.0, 0.0)
            .with_node(0.0, 1.0, 0.0)
            .with_triangle(1, 2, 3);

        assert_eq!(tri.triangles.len(), 1);
        assert_eq!(tri.triangles[0], [1, 2, 3]);
    }

    #[test]
    fn test_serialize_deserialize_simple_triangle() {
        let driver = BinMDataXtdTriangulationDriver::new(None);

        let original = MockTriangulation::new()
            .with_node(0.0, 0.0, 0.0)
            .with_node(1.0, 0.0, 0.0)
            .with_node(0.0, 1.0, 0.0)
            .with_triangle(1, 2, 3)
            .with_deflection(0.01);

        let mut buffer = Vec::new();
        driver.paste_write(&original, &mut buffer);

        let (deserialized, _) = driver.paste_read(&buffer, 0).unwrap();

        assert_eq!(deserialized.nodes.len(), 3);
        assert_eq!(deserialized.triangles.len(), 1);
        assert!((deserialized.deflection - 0.01).abs() < 1e-10);
    }

    #[test]
    fn test_serialize_deserialize_with_uv() {
        let driver = BinMDataXtdTriangulationDriver::new(None);

        let original = MockTriangulation::new()
            .with_node(0.0, 0.0, 0.0)
            .with_node(1.0, 0.0, 0.0)
            .with_node(0.0, 1.0, 0.0)
            .with_uv_node(0.0, 0.0)
            .with_uv_node(1.0, 0.0)
            .with_uv_node(0.0, 1.0)
            .with_triangle(1, 2, 3);

        let mut buffer = Vec::new();
        driver.paste_write(&original, &mut buffer);

        let (deserialized, _) = driver.paste_read(&buffer, 0).unwrap();

        assert!(deserialized.has_uv);
        assert_eq!(deserialized.uv_nodes.len(), 3);
        assert_eq!(deserialized.uv_nodes[0], [0.0, 0.0]);
        assert_eq!(deserialized.uv_nodes[1], [1.0, 0.0]);
    }

    #[test]
    fn test_serialize_deserialize_multiple_triangles() {
        let driver = BinMDataXtdTriangulationDriver::new(None);

        let original = MockTriangulation::new()
            .with_node(0.0, 0.0, 0.0)
            .with_node(1.0, 0.0, 0.0)
            .with_node(1.0, 1.0, 0.0)
            .with_node(0.0, 1.0, 0.0)
            .with_triangle(1, 2, 3)
            .with_triangle(1, 3, 4)
            .with_deflection(0.005);

        let mut buffer = Vec::new();
        driver.paste_write(&original, &mut buffer);

        let (deserialized, _) = driver.paste_read(&buffer, 0).unwrap();

        assert_eq!(deserialized.nodes.len(), 4);
        assert_eq!(deserialized.triangles.len(), 2);
        assert_eq!(deserialized.triangles[0], [1, 2, 3]);
        assert_eq!(deserialized.triangles[1], [1, 3, 4]);
    }

    #[test]
    fn test_coordinate_precision() {
        let driver = BinMDataXtdTriangulationDriver::new(None);

        let original = MockTriangulation::new()
            .with_node(1.23456789, 9.87654321, -5.55555555)
            .with_triangle(1, 1, 1);

        let mut buffer = Vec::new();
        driver.paste_write(&original, &mut buffer);

        let (deserialized, _) = driver.paste_read(&buffer, 0).unwrap();

        assert!((deserialized.nodes[0][0] - 1.23456789).abs() < 1e-10);
        assert!((deserialized.nodes[0][1] - 9.87654321).abs() < 1e-10);
        assert!((deserialized.nodes[0][2] - (-5.55555555)).abs() < 1e-10);
    }
}
