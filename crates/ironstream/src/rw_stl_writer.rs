// FILE: rust/ironstream/crates/ironstream/src/rw_stl_writer.rs
//! STL file I/O — high-level writer and reader facades that mirror the OCCT
//! `RWStl` / `StlAPI` packages.
//!
//! | OCCT               | IronStream    |
//! |--------------------|---------------|
//! | `StlAPI_Writer`    | [`StlWriter`] |
//! | `RWStl_Reader`     | [`StlReader`] |
//!
//! Both types operate on the indexed representation preferred by the rest of
//! IronStream: a flat node array (`&[[f64; 3]]`) and a flat triangle index
//! array (`&[[u32; 3]]`), rather than the triangle-soup used by the lower-level
//! `stl_mesh` module.  This matches the calling conventions of the OpenCASCADE
//! `BRep_Builder` / `Poly_Triangulation` stack.
//!
//! Only `std` is used; there are no external crate dependencies.

use std::fs;
use std::io::Write as IoWrite;

// ─── normal computation helper ───────────────────────────────────────────────

/// Compute the unit-length face normal for a triangle given by three `f64`
/// vertex positions.  Returns `[0.0, 0.0, 0.0]` for degenerate triangles.
fn compute_normal(a: [f64; 3], b: [f64; 3], c: [f64; 3]) -> [f64; 3] {
    let ab = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let ac = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    let cross = [
        ab[1] * ac[2] - ab[2] * ac[1],
        ab[2] * ac[0] - ab[0] * ac[2],
        ab[0] * ac[1] - ab[1] * ac[0],
    ];
    let len = (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt();
    if len == 0.0 {
        [0.0, 0.0, 0.0]
    } else {
        [cross[0] / len, cross[1] / len, cross[2] / len]
    }
}

// ─── StlWriter ───────────────────────────────────────────────────────────────

/// High-level STL writer.  Produces either ASCII or binary STL output.
///
/// Set `binary = true` (the default via [`StlWriter::new`]) to write the
/// compact 80-byte-header + little-endian binary format, or use
/// [`StlWriter::ascii`] to write human-readable ASCII STL.
///
// occt-ref: StlAPI_Writer
#[derive(Clone, Debug)]
pub struct StlWriter {
    /// When `true` the writer emits binary STL; when `false` it emits ASCII.
    pub binary: bool,
}

impl StlWriter {
    /// Create a writer that produces **binary** STL (the OCCT default).
    pub fn new() -> Self {
        Self { binary: true }
    }

    /// Create a writer that produces **ASCII** STL.
    pub fn ascii() -> Self {
        Self { binary: false }
    }

    /// Write the mesh to `path`.
    ///
    /// `nodes`   — indexed vertex positions, one `[x, y, z]` per entry.
    /// `tris`    — triangle index triples referencing into `nodes`.
    /// `normals` — optional per-triangle face normals.  When absent, normals
    ///             are computed from the cross product of the triangle edges.
    ///
    /// Returns `Err(String)` on any I/O or formatting failure.
    pub fn write(
        &self,
        nodes: &[[f64; 3]],
        tris: &[[u32; 3]],
        normals: Option<&[[f64; 3]]>,
        path: &str,
    ) -> Result<(), String> {
        let bytes = if self.binary {
            encode_binary(nodes, tris, normals)
        } else {
            encode_ascii(nodes, tris, normals).into_bytes()
        };
        let mut file = fs::File::create(path)
            .map_err(|e| format!("StlWriter::write — cannot create '{}': {}", path, e))?;
        file.write_all(&bytes)
            .map_err(|e| format!("StlWriter::write — I/O error writing '{}': {}", path, e))?;
        Ok(())
    }

    /// Serialise the mesh to an in-memory `Vec<u8>` in binary STL format,
    /// ignoring the `binary` flag (always binary).
    ///
    /// This is the cheap, allocation-only path used when you do not want to
    /// touch the file system.
    pub fn to_bytes(nodes: &[[f64; 3]], tris: &[[u32; 3]]) -> Vec<u8> {
        encode_binary(nodes, tris, None)
    }
}

impl Default for StlWriter {
    fn default() -> Self {
        Self::new()
    }
}

// ─── StlReader ───────────────────────────────────────────────────────────────

/// High-level STL reader.  Accepts both ASCII and binary STL files and
/// exposes the parsed geometry as indexed arrays.
///
// occt: RWStl_Reader
#[derive(Clone, Debug, Default)]
pub struct StlReader {
    /// Vertex positions in `[x, y, z]` order.
    pub nodes: Vec<[f64; 3]>,
    /// Triangle index triples referencing into [`nodes`](StlReader::nodes).
    pub triangles: Vec<[u32; 3]>,
    /// Per-triangle face normals as stored in the file.
    pub normals: Vec<[f64; 3]>,
}

impl StlReader {
    /// Create an empty reader ready to accept geometry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Read and parse the file at `path`, auto-detecting ASCII vs binary.
    ///
    /// Clears any previously parsed data before reading.
    pub fn read_file(&mut self, path: &str) -> Result<(), String> {
        self.nodes.clear();
        self.triangles.clear();
        self.normals.clear();

        let data = fs::read(path)
            .map_err(|e| format!("StlReader::read_file — cannot read '{}': {}", path, e))?;

        // The STL spec says ASCII files start with "solid".  Binary files may
        // accidentally start with those bytes, so we also check the 4-byte
        // triangle count encoded at offset 80 against the actual file size.
        if looks_like_ascii(&data) {
            let text = std::str::from_utf8(&data)
                .map_err(|e| format!("StlReader::read_file — invalid UTF-8 in '{}': {}", path, e))?;
            self.parse_ascii(text)
        } else {
            self.parse_binary(&data)
        }
    }

    /// Parse an ASCII STL string into this reader, appending to any existing
    /// geometry.
    ///
    /// Returns `Err(String)` on any parse failure.
    pub fn parse_ascii(&mut self, data: &str) -> Result<(), String> {
        let mut tokens = data.split_ascii_whitespace().peekable();

        // "solid [name]"
        match tokens.next() {
            Some(t) if t.eq_ignore_ascii_case("solid") => {}
            Some(t) => return Err(format!("parse_ascii: expected 'solid', got '{}'", t)),
            None => return Err("parse_ascii: empty input".to_owned()),
        }
        // skip solid name tokens until "facet" or "endsolid"
        while let Some(&next) = tokens.peek() {
            if next.eq_ignore_ascii_case("facet") || next.eq_ignore_ascii_case("endsolid") {
                break;
            }
            tokens.next();
        }

        loop {
            match tokens.next().as_deref() {
                Some(t) if t.eq_ignore_ascii_case("endsolid") => break,
                Some(t) if t.eq_ignore_ascii_case("facet") => {
                    ascii_expect(&mut tokens, "normal")?;
                    let nx = ascii_f64(&mut tokens, "normal x")?;
                    let ny = ascii_f64(&mut tokens, "normal y")?;
                    let nz = ascii_f64(&mut tokens, "normal z")?;

                    ascii_expect(&mut tokens, "outer")?;
                    ascii_expect(&mut tokens, "loop")?;

                    let v0 = ascii_vertex(&mut tokens)?;
                    let v1 = ascii_vertex(&mut tokens)?;
                    let v2 = ascii_vertex(&mut tokens)?;

                    ascii_expect(&mut tokens, "endloop")?;
                    ascii_expect(&mut tokens, "endfacet")?;

                    let base = self.nodes.len() as u32;
                    self.nodes.push(v0);
                    self.nodes.push(v1);
                    self.nodes.push(v2);
                    self.triangles.push([base, base + 1, base + 2]);
                    self.normals.push([nx, ny, nz]);
                }
                Some(t) => return Err(format!("parse_ascii: unexpected token '{}'", t)),
                None => return Err("parse_ascii: unexpected end of input before 'endsolid'".to_owned()),
            }
        }

        Ok(())
    }

    /// Parse a binary STL byte slice into this reader, appending to any
    /// existing geometry.
    ///
    /// Returns `Err(String)` on any parse failure.
    pub fn parse_binary(&mut self, data: &[u8]) -> Result<(), String> {
        // Binary STL layout:
        //   80 bytes  — header (ignored)
        //    4 bytes  — triangle count (little-endian u32)
        //   50 bytes per triangle:
        //     12 bytes — normal  (3 × f32 LE)
        //     12 bytes — vertex0 (3 × f32 LE)
        //     12 bytes — vertex1 (3 × f32 LE)
        //     12 bytes — vertex2 (3 × f32 LE)
        //      2 bytes — attribute byte count (ignored)
        const HEADER: usize = 80;
        const COUNT_SZ: usize = 4;
        const TRI_SZ: usize = 50;

        if data.len() < HEADER + COUNT_SZ {
            return Err("parse_binary: file too short to contain a valid binary STL header".to_owned());
        }

        let count = u32::from_le_bytes([data[80], data[81], data[82], data[83]]) as usize;
        let expected_len = HEADER + COUNT_SZ + count * TRI_SZ;
        if data.len() < expected_len {
            return Err(format!(
                "parse_binary: file declares {} triangles but is only {} bytes (need {})",
                count,
                data.len(),
                expected_len
            ));
        }

        self.nodes.reserve(count * 3);
        self.triangles.reserve(count);
        self.normals.reserve(count);

        let mut offset = HEADER + COUNT_SZ;
        for _ in 0..count {
            let nx = f32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as f64;
            let ny = f32::from_le_bytes(data[offset + 4..offset + 8].try_into().unwrap()) as f64;
            let nz = f32::from_le_bytes(data[offset + 8..offset + 12].try_into().unwrap()) as f64;
            offset += 12;

            let mut verts = [[0f64; 3]; 3];
            for v in &mut verts {
                v[0] = f32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as f64;
                v[1] = f32::from_le_bytes(data[offset + 4..offset + 8].try_into().unwrap()) as f64;
                v[2] = f32::from_le_bytes(data[offset + 8..offset + 12].try_into().unwrap()) as f64;
                offset += 12;
            }
            offset += 2; // attribute byte count

            let base = self.nodes.len() as u32;
            self.nodes.push(verts[0]);
            self.nodes.push(verts[1]);
            self.nodes.push(verts[2]);
            self.triangles.push([base, base + 1, base + 2]);
            self.normals.push([nx, ny, nz]);
        }

        Ok(())
    }

    /// Return the number of triangles currently held by this reader.
    pub fn triangle_count(&self) -> usize {
        self.triangles.len()
    }
}

// ─── free function ───────────────────────────────────────────────────────────

/// Write an indexed mesh to `path` in **binary** STL format.
///
/// This is a thin convenience wrapper around [`StlWriter::new`] +
/// [`StlWriter::write`].
pub fn write_stl_binary(
    nodes: &[[f64; 3]],
    tris: &[[u32; 3]],
    path: &str,
) -> Result<(), String> {
    StlWriter::new().write(nodes, tris, None, path)
}

// ─── encoding helpers ────────────────────────────────────────────────────────

/// Encode the mesh as a binary STL byte vector.
fn encode_binary(
    nodes: &[[f64; 3]],
    tris: &[[u32; 3]],
    normals: Option<&[[f64; 3]]>,
) -> Vec<u8> {
    let mut buf: Vec<u8> = Vec::with_capacity(84 + tris.len() * 50);

    // 80-byte header — zeroed
    buf.extend_from_slice(&[0u8; 80]);

    // triangle count
    buf.extend_from_slice(&(tris.len() as u32).to_le_bytes());

    for (i, tri) in tris.iter().enumerate() {
        let [i0, i1, i2] = *tri;
        let v0 = nodes[i0 as usize];
        let v1 = nodes[i1 as usize];
        let v2 = nodes[i2 as usize];

        let n = if let Some(ns) = normals {
            ns[i]
        } else {
            compute_normal(v0, v1, v2)
        };

        // normal
        for c in n {
            buf.extend_from_slice(&(c as f32).to_le_bytes());
        }
        // vertices
        for v in [v0, v1, v2] {
            for c in v {
                buf.extend_from_slice(&(c as f32).to_le_bytes());
            }
        }
        // attribute byte count
        buf.extend_from_slice(&0u16.to_le_bytes());
    }

    buf
}

/// Encode the mesh as an ASCII STL string.
fn encode_ascii(
    nodes: &[[f64; 3]],
    tris: &[[u32; 3]],
    normals: Option<&[[f64; 3]]>,
) -> String {
    let mut out = String::new();
    out.push_str("solid mesh\n");

    for (i, tri) in tris.iter().enumerate() {
        let [i0, i1, i2] = *tri;
        let v0 = nodes[i0 as usize];
        let v1 = nodes[i1 as usize];
        let v2 = nodes[i2 as usize];

        let n = if let Some(ns) = normals {
            ns[i]
        } else {
            compute_normal(v0, v1, v2)
        };

        out.push_str(&format!(
            "  facet normal {} {} {}\n    outer loop\n",
            n[0], n[1], n[2]
        ));
        for v in [v0, v1, v2] {
            out.push_str(&format!("      vertex {} {} {}\n", v[0], v[1], v[2]));
        }
        out.push_str("    endloop\n  endfacet\n");
    }

    out.push_str("endsolid mesh\n");
    out
}

// ─── detection helper ────────────────────────────────────────────────────────

/// Heuristic: return `true` when `data` looks like an ASCII STL file.
///
/// The check is: the first non-whitespace bytes spell "solid" AND the
/// size is inconsistent with binary STL (header + 4-byte count + N×50 bytes).
fn looks_like_ascii(data: &[u8]) -> bool {
    let trimmed = data.iter().position(|&b| !b.is_ascii_whitespace());
    let starts_with_solid = trimmed
        .map(|i| {
            data[i..]
                .get(..5)
                .map(|s| s.eq_ignore_ascii_case(b"solid"))
                .unwrap_or(false)
        })
        .unwrap_or(false);

    if !starts_with_solid {
        return false;
    }

    // Verify that the binary interpretation would be size-inconsistent.
    if data.len() >= 84 {
        let count = u32::from_le_bytes([data[80], data[81], data[82], data[83]]) as usize;
        let expected = 84 + count * 50;
        if data.len() == expected {
            // Matches binary layout exactly — treat as binary.
            return false;
        }
    }

    true
}

// ─── ASCII parse helpers ─────────────────────────────────────────────────────

fn ascii_expect<'a>(
    tokens: &mut impl Iterator<Item = &'a str>,
    expected: &str,
) -> Result<(), String> {
    match tokens.next() {
        Some(t) if t.eq_ignore_ascii_case(expected) => Ok(()),
        Some(t) => Err(format!("parse_ascii: expected '{}', got '{}'", expected, t)),
        None => Err(format!("parse_ascii: expected '{}', got end of input", expected)),
    }
}

fn ascii_f64<'a>(
    tokens: &mut impl Iterator<Item = &'a str>,
    label: &str,
) -> Result<f64, String> {
    match tokens.next() {
        Some(t) => t
            .parse::<f64>()
            .map_err(|e| format!("parse_ascii: invalid f64 for {}: {}", label, e)),
        None => Err(format!("parse_ascii: expected f64 for {}, got end of input", label)),
    }
}

fn ascii_vertex<'a>(
    tokens: &mut impl Iterator<Item = &'a str>,
) -> Result<[f64; 3], String> {
    ascii_expect(tokens, "vertex")?;
    let x = ascii_f64(tokens, "vertex x")?;
    let y = ascii_f64(tokens, "vertex y")?;
    let z = ascii_f64(tokens, "vertex z")?;
    Ok([x, y, z])
}

// ─── unit tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn unit_triangle_nodes() -> Vec<[f64; 3]> {
        vec![[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]]
    }

    fn unit_triangle_tris() -> Vec<[u32; 3]> {
        vec![[0, 1, 2]]
    }

    // ── StlWriter ──

    #[test]
    fn writer_new_is_binary() {
        assert!(StlWriter::new().binary);
    }

    #[test]
    fn writer_ascii_is_ascii() {
        assert!(!StlWriter::ascii().binary);
    }

    #[test]
    fn to_bytes_produces_valid_binary_header() {
        let nodes = unit_triangle_nodes();
        let tris = unit_triangle_tris();
        let bytes = StlWriter::to_bytes(&nodes, &tris);
        // 80-byte header + 4-byte count + 1 triangle × 50 bytes = 134
        assert_eq!(bytes.len(), 134);
        // triangle count field
        let count = u32::from_le_bytes([bytes[80], bytes[81], bytes[82], bytes[83]]);
        assert_eq!(count, 1);
    }

    #[test]
    fn binary_normal_is_z_axis() {
        let nodes = unit_triangle_nodes();
        let tris = unit_triangle_tris();
        let bytes = StlWriter::to_bytes(&nodes, &tris);
        // normal is at offset 84
        let nx = f32::from_le_bytes(bytes[84..88].try_into().unwrap());
        let ny = f32::from_le_bytes(bytes[88..92].try_into().unwrap());
        let nz = f32::from_le_bytes(bytes[92..96].try_into().unwrap());
        assert!((nx).abs() < 1e-6);
        assert!((ny).abs() < 1e-6);
        assert!((nz - 1.0).abs() < 1e-6);
    }

    // ── StlReader ──

    #[test]
    fn reader_new_is_empty() {
        let r = StlReader::new();
        assert_eq!(r.triangle_count(), 0);
        assert!(r.nodes.is_empty());
        assert!(r.normals.is_empty());
    }

    #[test]
    fn parse_ascii_round_trip() {
        let nodes = unit_triangle_nodes();
        let tris = unit_triangle_tris();
        let ascii = encode_ascii(&nodes, &tris, None);

        let mut reader = StlReader::new();
        reader.parse_ascii(&ascii).expect("parse failed");

        assert_eq!(reader.triangle_count(), 1);
        assert_eq!(reader.nodes.len(), 3);
        let n = reader.normals[0];
        assert!((n[2] - 1.0).abs() < 1e-5, "expected z-normal, got {:?}", n);
    }

    #[test]
    fn parse_binary_round_trip() {
        let nodes = unit_triangle_nodes();
        let tris = unit_triangle_tris();
        let bytes = encode_binary(&nodes, &tris, None);

        let mut reader = StlReader::new();
        reader.parse_binary(&bytes).expect("binary parse failed");

        assert_eq!(reader.triangle_count(), 1);
        assert_eq!(reader.nodes.len(), 3);
    }

    #[test]
    fn parse_binary_rejects_truncated_data() {
        let bytes = vec![0u8; 10];
        let mut reader = StlReader::new();
        assert!(reader.parse_binary(&bytes).is_err());
    }

    #[test]
    fn parse_ascii_rejects_garbage() {
        let mut reader = StlReader::new();
        assert!(reader.parse_ascii("not an stl file").is_err());
    }

    #[test]
    fn write_stl_binary_free_fn() {
        let nodes = unit_triangle_nodes();
        let tris = unit_triangle_tris();
        let dir = std::env::temp_dir();
        let path = dir.join("rw_stl_writer_test.stl");
        let path_str = path.to_str().unwrap();
        write_stl_binary(&nodes, &tris, path_str).expect("write failed");
        let data = std::fs::read(path_str).unwrap();
        assert_eq!(data.len(), 134);
    }

    #[test]
    fn looks_like_ascii_detection() {
        let ascii = b"solid test\nendsolid test\n";
        assert!(looks_like_ascii(ascii));
        let binary = encode_binary(&unit_triangle_nodes(), &unit_triangle_tris(), None);
        assert!(!looks_like_ascii(&binary));
    }

    #[test]
    fn compute_normal_unit_z() {
        let n = compute_normal([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);
        assert!((n[2] - 1.0).abs() < 1e-12);
    }

    #[test]
    fn compute_normal_degenerate() {
        let n = compute_normal([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
        assert_eq!(n, [0.0, 0.0, 0.0]);
    }

    #[test]
    fn writer_write_ascii_then_read_back() {
        let nodes = unit_triangle_nodes();
        let tris = unit_triangle_tris();
        let dir = std::env::temp_dir();
        let path = dir.join("rw_stl_writer_ascii_test.stl");
        let path_str = path.to_str().unwrap();

        let w = StlWriter::ascii();
        w.write(&nodes, &tris, None, path_str).expect("ascii write failed");

        let mut reader = StlReader::new();
        reader.read_file(path_str).expect("read_file failed");
        assert_eq!(reader.triangle_count(), 1);
    }
}
