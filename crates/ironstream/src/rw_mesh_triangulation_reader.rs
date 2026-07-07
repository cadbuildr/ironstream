// FILE: rw_mesh_triangulation_reader.rs
// occt: RWMesh_TriangulationReader

//! Interface for reading primitive array from the buffer.
//! Provides functionality for loading triangulation data with configurable
//! options for precision, degenerate triangle handling, and statistics.

use std::sync::Mutex;

/// Statistics about the loaded triangulation
#[derive(Clone, Debug)]
pub struct LoadingStatistic {
    /// Expected number of nodes
    pub expected_nodes_nb: i32,
    /// Actually loaded nodes
    pub loaded_nodes_nb: i32,
    /// Expected number of triangles
    pub expected_triangles_nb: i32,
    /// Degenerated triangles encountered
    pub degenerated_triangles_nb: i32,
    /// Actually loaded triangles
    pub loaded_triangles_nb: i32,
}

impl LoadingStatistic {
    /// Create new statistics structure
    pub fn new() -> Self {
        LoadingStatistic {
            expected_nodes_nb: 0,
            loaded_nodes_nb: 0,
            expected_triangles_nb: 0,
            degenerated_triangles_nb: 0,
            loaded_triangles_nb: 0,
        }
    }

    /// Reset all statistics to zero
    pub fn reset(&mut self) {
        self.expected_nodes_nb = 0;
        self.loaded_nodes_nb = 0;
        self.expected_triangles_nb = 0;
        self.degenerated_triangles_nb = 0;
        self.loaded_triangles_nb = 0;
    }

    /// Print statistics with optional prefix message
    pub fn print_statistic(&self, prefix: &str) {
        println!(
            "{}Expected nodes: {}, Loaded: {}",
            prefix, self.expected_nodes_nb, self.loaded_nodes_nb
        );
        println!(
            "{}Expected triangles: {}, Loaded: {}, Degenerated: {}",
            prefix, self.expected_triangles_nb, self.loaded_triangles_nb, self.degenerated_triangles_nb
        );
    }
}

impl Default for LoadingStatistic {
    fn default() -> Self {
        Self::new()
    }
}

/// Coordinate system converter for mesh data
#[derive(Clone, Debug, Default)]
pub struct CoordinateSystemConverter;

/// Reader for triangulation data
pub struct TriangulationReader {
    /// File name for reporting
    file_name: String,
    /// Coordinate system converter
    coord_sys_converter: CoordinateSystemConverter,
    /// Double precision flag (false = single precision)
    is_double_precision: bool,
    /// Skip degenerate triangles during loading
    to_skip_degenerate_tris: bool,
    /// Print debug messages flag
    to_print_debug_messages: bool,
    /// Loading statistics (optional)
    loading_statistic: Mutex<Option<LoadingStatistic>>,
}

impl TriangulationReader {
    /// Create a new triangulation reader
    pub fn new() -> Self {
        TriangulationReader {
            file_name: String::new(),
            coord_sys_converter: CoordinateSystemConverter::default(),
            is_double_precision: false,
            to_skip_degenerate_tris: false,
            to_print_debug_messages: false,
            loading_statistic: Mutex::new(None),
        }
    }

    /// Get the file name for reporting
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    /// Set the file name for reporting
    pub fn set_file_name(&mut self, file_name: impl Into<String>) {
        self.file_name = file_name.into();
    }

    /// Get the coordinate system converter
    pub fn coordinate_system_converter(&self) -> &CoordinateSystemConverter {
        &self.coord_sys_converter
    }

    /// Set the coordinate system converter
    pub fn set_coordinate_system_converter(&mut self, converter: CoordinateSystemConverter) {
        self.coord_sys_converter = converter;
    }

    /// Check if double precision is enabled
    pub fn is_double_precision(&self) -> bool {
        self.is_double_precision
    }

    /// Set double precision flag
    pub fn set_double_precision(&mut self, is_double: bool) {
        self.is_double_precision = is_double;
    }

    /// Check if degenerate triangles should be skipped
    pub fn to_skip_degenerates(&self) -> bool {
        self.to_skip_degenerate_tris
    }

    /// Set flag to skip degenerate triangles
    pub fn set_to_skip_degenerates(&mut self, to_skip: bool) {
        self.to_skip_degenerate_tris = to_skip;
    }

    /// Check if debug messages should be printed
    pub fn to_print_debug_messages(&self) -> bool {
        self.to_print_debug_messages
    }

    /// Set flag to print debug messages
    pub fn set_to_print_debug_messages(&mut self, to_print: bool) {
        self.to_print_debug_messages = to_print;
    }

    /// Start and reset internal statistics
    pub fn start_statistic(&self) {
        let mut stat = self.loading_statistic.lock().unwrap();
        if let Some(ref mut s) = *stat {
            s.reset();
        } else {
            *stat = Some(LoadingStatistic::new());
        }
    }

    /// Stop and clear internal statistics
    pub fn stop_statistic(&self) {
        let mut stat = self.loading_statistic.lock().unwrap();
        *stat = None;
    }

    /// Print loading statistics
    pub fn print_statistic(&self) {
        let stat = self.loading_statistic.lock().unwrap();
        if let Some(s) = &*stat {
            let prefix = format!("[Mesh reader. File '{}']. ", self.file_name);
            s.print_statistic(&prefix);
        }
    }

    /// Load triangulation (virtual method to be overridden)
    /// Returns true if loading was successful
    pub fn load(&self) -> bool {
        // This is the main public interface
        // Subclasses would override the protected load() method
        true
    }

    /// Finalize loading (can be overridden for additional actions)
    pub fn finalize_loading(&self) -> bool {
        true
    }
}

impl Default for TriangulationReader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loading_statistic_creation() {
        let stat = LoadingStatistic::new();
        assert_eq!(stat.expected_nodes_nb, 0);
        assert_eq!(stat.loaded_nodes_nb, 0);
        assert_eq!(stat.expected_triangles_nb, 0);
        assert_eq!(stat.degenerated_triangles_nb, 0);
        assert_eq!(stat.loaded_triangles_nb, 0);
    }

    #[test]
    fn test_loading_statistic_reset() {
        let mut stat = LoadingStatistic::new();
        stat.expected_nodes_nb = 100;
        stat.loaded_nodes_nb = 50;

        stat.reset();

        assert_eq!(stat.expected_nodes_nb, 0);
        assert_eq!(stat.loaded_nodes_nb, 0);
    }

    #[test]
    fn test_reader_creation() {
        let reader = TriangulationReader::new();

        assert_eq!(reader.file_name(), "");
        assert!(!reader.is_double_precision());
        assert!(!reader.to_skip_degenerates());
        assert!(!reader.to_print_debug_messages());
    }

    #[test]
    fn test_reader_precision_flag() {
        let mut reader = TriangulationReader::new();

        assert!(!reader.is_double_precision());
        reader.set_double_precision(true);
        assert!(reader.is_double_precision());
    }

    #[test]
    fn test_reader_skip_degenerates_flag() {
        let mut reader = TriangulationReader::new();

        assert!(!reader.to_skip_degenerates());
        reader.set_to_skip_degenerates(true);
        assert!(reader.to_skip_degenerates());
    }

    #[test]
    fn test_reader_debug_messages_flag() {
        let mut reader = TriangulationReader::new();

        assert!(!reader.to_print_debug_messages());
        reader.set_to_print_debug_messages(true);
        assert!(reader.to_print_debug_messages());
    }

    #[test]
    fn test_reader_file_name() {
        let mut reader = TriangulationReader::new();

        reader.set_file_name("test.mesh");
        assert_eq!(reader.file_name(), "test.mesh");
    }

    #[test]
    fn test_reader_statistics_lifecycle() {
        let reader = TriangulationReader::new();

        reader.start_statistic();
        reader.print_statistic();
        reader.stop_statistic();
    }
}
