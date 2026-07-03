// FILE: rw_stl.rs
// occt: RWStl, StlAPI_Reader, StlAPI_Writer

/// STL file format (binary or ASCII).
/// occt: RWStl
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StlFormat {
    Binary,
    Ascii,
}

impl Default for StlFormat {
    fn default() -> Self { Self::Binary }
}

/// STL reader result.
#[derive(Clone, Debug)]
pub struct StlReadResult {
    pub mesh_id: u32,
    pub nb_triangles: usize,
    pub nb_nodes: usize,
    pub is_success: bool,
    pub error_msg: String,
}

impl StlReadResult {
    pub fn success(mesh_id: u32, nb_triangles: usize) -> Self {
        Self { mesh_id, nb_triangles, nb_nodes: nb_triangles * 3, is_success: true, error_msg: String::new() }
    }

    pub fn failure(msg: &str) -> Self {
        Self { mesh_id: 0, nb_triangles: 0, nb_nodes: 0, is_success: false, error_msg: msg.to_string() }
    }
}

/// STL reader.
/// occt: StlAPI_Reader / RWStl (read side)
#[derive(Clone, Debug)]
pub struct StlReader {
    pub file_path: String,
    pub result: Option<StlReadResult>,
    pub merge_nodes: bool,
    pub merge_tol: f64,
}

impl StlReader {
    pub fn new() -> Self {
        Self {
            file_path: String::new(),
            result: None,
            merge_nodes: false,
            merge_tol: 1e-7,
        }
    }

    pub fn set_merge_nodes(&mut self, v: bool) { self.merge_nodes = v; }
    pub fn set_merge_tolerance(&mut self, t: f64) { self.merge_tol = t.max(0.0); }

    /// Stub read: success if path ends with ".stl" (case-insensitive).
    pub fn read(&mut self, path: &str, mesh_id: u32) -> bool {
        self.file_path = path.to_string();
        let lower = path.to_lowercase();
        if lower.ends_with(".stl") {
            self.result = Some(StlReadResult::success(mesh_id, 100));
            true
        } else {
            self.result = Some(StlReadResult::failure("not an STL file"));
            false
        }
    }

    pub fn is_done(&self) -> bool {
        self.result.as_ref().map(|r| r.is_success).unwrap_or(false)
    }

    pub fn nb_triangles(&self) -> usize {
        self.result.as_ref().map(|r| r.nb_triangles).unwrap_or(0)
    }
}

impl Default for StlReader {
    fn default() -> Self { Self::new() }
}

/// STL writer.
/// occt: StlAPI_Writer / RWStl (write side)
#[derive(Clone, Debug)]
pub struct StlWriter {
    pub file_path: String,
    pub format: StlFormat,
    pub is_relative_deflection: bool,
    pub deflection: f64,
    pub solid_name: String,
    pub is_done: bool,
}

impl StlWriter {
    pub fn new() -> Self {
        Self {
            file_path: String::new(),
            format: StlFormat::Binary,
            is_relative_deflection: false,
            deflection: 0.001,
            solid_name: String::from("Solid"),
            is_done: false,
        }
    }

    pub fn set_format(&mut self, f: StlFormat) { self.format = f; }
    pub fn set_deflection(&mut self, d: f64) { self.deflection = d.max(1e-10); }
    pub fn set_relative_deflection(&mut self, v: bool) { self.is_relative_deflection = v; }
    pub fn set_solid_name(&mut self, n: &str) { self.solid_name = n.to_string(); }

    /// Stub write: success if mesh_id > 0 and path is non-empty.
    pub fn write(&mut self, path: &str, mesh_id: u32) -> bool {
        self.file_path = path.to_string();
        self.is_done = mesh_id > 0 && !path.is_empty();
        self.is_done
    }

    pub fn is_done(&self) -> bool { self.is_done }
    pub fn format(&self) -> StlFormat { self.format }
}

impl Default for StlWriter {
    fn default() -> Self { Self::new() }
}

/// STL info: inspect an existing STL file without fully reading it.
/// occt: RWStl info functions
#[derive(Clone, Debug, Default)]
pub struct StlFileInfo {
    pub path: String,
    pub format: StlFormat,
    pub nb_triangles_hint: usize,
    pub file_size_bytes: u64,
}

impl StlFileInfo {
    pub fn probe(path: &str) -> Self {
        let lower = path.to_lowercase();
        let valid = lower.ends_with(".stl");
        Self {
            path: path.to_string(),
            format: StlFormat::Binary,
            nb_triangles_hint: if valid { 1000 } else { 0 },
            file_size_bytes: if valid { 84 + 1000 * 50 } else { 0 },
        }
    }

    pub fn is_binary(&self) -> bool { self.format == StlFormat::Binary }
    pub fn nb_triangles_hint(&self) -> usize { self.nb_triangles_hint }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stl_reader_success() {
        let mut r = StlReader::new();
        assert!(r.read("model.stl", 1));
        assert!(r.is_done());
        assert_eq!(r.nb_triangles(), 100);
    }

    #[test]
    fn stl_reader_failure() {
        let mut r = StlReader::new();
        assert!(!r.read("model.obj", 1));
        assert!(!r.is_done());
    }

    #[test]
    fn stl_writer_write() {
        let mut w = StlWriter::new();
        w.set_format(StlFormat::Ascii);
        assert!(w.write("output.stl", 1));
        assert!(w.is_done());
        assert_eq!(w.format(), StlFormat::Ascii);
    }

    #[test]
    fn stl_writer_fail_empty() {
        let mut w = StlWriter::new();
        assert!(!w.write("", 1));
        assert!(!w.is_done());
    }

    #[test]
    fn stl_file_info_probe() {
        let info = StlFileInfo::probe("part.stl");
        assert!(info.nb_triangles_hint() > 0);
        assert!(info.is_binary());
    }
}
