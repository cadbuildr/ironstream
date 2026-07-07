// FILE: bop_test_objects.rs
// occt: BOPTest_Objects

use std::sync::{Mutex, OnceLock};
use std::collections::VecDeque;

/// Enumeration for glue mode in Boolean operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GlueMode {
    Off,
    Full,
    Shift,
}

/// Session for managing Boolean operation test context.
/// This mirrors OCCT's BOPTest_Session class.
struct TestSession {
    run_parallel: bool,
    non_destructive: bool,
    fuzzy_value: f64,
    glue_mode: GlueMode,
    draw_warn_shapes: bool,
    check_inverted: bool,
    use_obb: bool,
    unify_edges: bool,
    unify_faces: bool,
    angular_tol: f64,
}

impl TestSession {
    fn new() -> Self {
        TestSession {
            run_parallel: false,
            non_destructive: false,
            fuzzy_value: 1e-7,                // Precision::Confusion equivalent
            glue_mode: GlueMode::Off,
            draw_warn_shapes: false,
            check_inverted: true,
            use_obb: false,
            unify_edges: false,
            unify_faces: false,
            angular_tol: 1e-10,               // Precision::Angular equivalent
        }
    }

    fn set_default_options(&mut self) {
        self.run_parallel = false;
        self.non_destructive = false;
        self.fuzzy_value = 1e-7;
        self.glue_mode = GlueMode::Off;
        self.draw_warn_shapes = false;
        self.check_inverted = true;
        self.use_obb = false;
        self.unify_edges = false;
        self.unify_faces = false;
        self.angular_tol = 1e-10;
    }
}

/// Global test session state (replaces static BOPTest_Session in OCCT).
#[allow(non_snake_case)]
fn SESSION() -> &'static Mutex<TestSession> {
    static CELL: OnceLock<Mutex<TestSession>> = OnceLock::new();
    CELL.get_or_init(|| Mutex::new(TestSession::new()))
}

/// Mock static storage for shape lists since we can't use lazy_static without deps.
struct ShapeStorage {
    shapes: VecDeque<usize>,       // Shape IDs instead of TopoDS_Shape
    tools: VecDeque<usize>,
}

impl ShapeStorage {
    fn new() -> Self {
        ShapeStorage {
            shapes: VecDeque::new(),
            tools: VecDeque::new(),
        }
    }

    fn clear(&mut self) {
        self.shapes.clear();
        self.tools.clear();
    }
}

/// Global storage for shapes and tools (thread-local for testing).
thread_local! {
    static SHAPES_STORAGE: Mutex<ShapeStorage> = Mutex::new(ShapeStorage::new());
}

/// BOPTest_Objects: Static utility class for managing Boolean operation testing.
pub struct BopTestObjects;

impl BopTestObjects {
    /// Initializes the test session.
    pub fn init() {
        if let Ok(mut session) = SESSION().lock() {
            session.set_default_options();
        }
    }

    /// Clears all state from the test session.
    pub fn clear() {
        SHAPES_STORAGE.with(|storage| {
            if let Ok(mut s) = storage.lock() {
                s.clear();
            }
        });
        if let Ok(mut session) = SESSION().lock() {
            session.set_default_options();
        }
    }

    /// Returns true if parallel execution is enabled.
    pub fn run_parallel() -> bool {
        SESSION().lock().ok().map(|s| s.run_parallel).unwrap_or(false)
    }

    /// Enables or disables parallel execution.
    pub fn set_run_parallel(flag: bool) {
        if let Ok(mut session) = SESSION().lock() {
            session.run_parallel = flag;
        }
    }

    /// Sets the fuzzy value for tolerance.
    pub fn set_fuzzy_value(value: f64) {
        if let Ok(mut session) = SESSION().lock() {
            session.fuzzy_value = value;
        }
    }

    /// Returns the current fuzzy value.
    pub fn fuzzy_value() -> f64 {
        SESSION().lock().ok().map(|s| s.fuzzy_value).unwrap_or(1e-7)
    }

    /// Sets the non-destructive mode flag.
    pub fn set_non_destructive(flag: bool) {
        if let Ok(mut session) = SESSION().lock() {
            session.non_destructive = flag;
        }
    }

    /// Returns true if non-destructive mode is enabled.
    pub fn non_destructive() -> bool {
        SESSION().lock().ok().map(|s| s.non_destructive).unwrap_or(false)
    }

    /// Sets the glue mode.
    pub fn set_glue(mode: GlueMode) {
        if let Ok(mut session) = SESSION().lock() {
            session.glue_mode = mode;
        }
    }

    /// Returns the current glue mode.
    pub fn glue() -> GlueMode {
        SESSION().lock().ok().map(|s| s.glue_mode).unwrap_or(GlueMode::Off)
    }

    /// Enables or disables drawing of warning shapes.
    pub fn set_draw_warn_shapes(flag: bool) {
        if let Ok(mut session) = SESSION().lock() {
            session.draw_warn_shapes = flag;
        }
    }

    /// Returns true if warning shapes are drawn.
    pub fn draw_warn_shapes() -> bool {
        SESSION().lock().ok().map(|s| s.draw_warn_shapes).unwrap_or(false)
    }

    /// Enables or disables inversion check.
    pub fn set_check_inverted(flag: bool) {
        if let Ok(mut session) = SESSION().lock() {
            session.check_inverted = flag;
        }
    }

    /// Returns true if inversion check is enabled.
    pub fn check_inverted() -> bool {
        SESSION().lock().ok().map(|s| s.check_inverted).unwrap_or(true)
    }

    /// Enables or disables oriented bounding box usage.
    pub fn set_use_obb(flag: bool) {
        if let Ok(mut session) = SESSION().lock() {
            session.use_obb = flag;
        }
    }

    /// Returns true if OBB is used.
    pub fn use_obb() -> bool {
        SESSION().lock().ok().map(|s| s.use_obb).unwrap_or(false)
    }

    /// Enables or disables edge unification.
    pub fn set_unify_edges(flag: bool) {
        if let Ok(mut session) = SESSION().lock() {
            session.unify_edges = flag;
        }
    }

    /// Returns true if edge unification is enabled.
    pub fn unify_edges() -> bool {
        SESSION().lock().ok().map(|s| s.unify_edges).unwrap_or(false)
    }

    /// Enables or disables face unification.
    pub fn set_unify_faces(flag: bool) {
        if let Ok(mut session) = SESSION().lock() {
            session.unify_faces = flag;
        }
    }

    /// Returns true if face unification is enabled.
    pub fn unify_faces() -> bool {
        SESSION().lock().ok().map(|s| s.unify_faces).unwrap_or(false)
    }

    /// Sets the angular tolerance.
    pub fn set_angular(tol: f64) {
        if let Ok(mut session) = SESSION().lock() {
            session.angular_tol = tol;
        }
    }

    /// Returns the current angular tolerance.
    pub fn angular() -> f64 {
        SESSION().lock().ok().map(|s| s.angular_tol).unwrap_or(1e-10)
    }

    /// Sets default options for all flags.
    pub fn set_default_options() {
        if let Ok(mut session) = SESSION().lock() {
            session.set_default_options();
        }
    }

    /// Returns a mutable reference to the shapes list.
    pub fn shapes() -> Vec<usize> {
        SHAPES_STORAGE.with(|storage| {
            storage.lock().ok().map(|s| s.shapes.iter().copied().collect()).unwrap_or_default()
        })
    }

    /// Returns a mutable reference to the tools list.
    pub fn tools() -> Vec<usize> {
        SHAPES_STORAGE.with(|storage| {
            storage.lock().ok().map(|s| s.tools.iter().copied().collect()).unwrap_or_default()
        })
    }

    /// Adds a shape to the shapes list.
    pub fn add_shape(shape_id: usize) {
        SHAPES_STORAGE.with(|storage| {
            if let Ok(mut s) = storage.lock() {
                s.shapes.push_back(shape_id);
            }
        });
    }

    /// Adds a shape to the tools list.
    pub fn add_tool(shape_id: usize) {
        SHAPES_STORAGE.with(|storage| {
            if let Ok(mut s) = storage.lock() {
                s.tools.push_back(shape_id);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_and_clear() {
        BopTestObjects::init();
        BopTestObjects::add_shape(1);
        BopTestObjects::add_tool(2);
        assert_eq!(BopTestObjects::shapes().len(), 1);
        assert_eq!(BopTestObjects::tools().len(), 1);

        BopTestObjects::clear();
        assert_eq!(BopTestObjects::shapes().len(), 0);
        assert_eq!(BopTestObjects::tools().len(), 0);
    }

    #[test]
    fn test_default_options() {
        BopTestObjects::init();
        assert!(!BopTestObjects::run_parallel());
        assert!(!BopTestObjects::non_destructive());
        assert_eq!(BopTestObjects::glue(), GlueMode::Off);
        assert!(!BopTestObjects::draw_warn_shapes());
        assert!(BopTestObjects::check_inverted());
        assert!(!BopTestObjects::use_obb());
        assert!(!BopTestObjects::unify_edges());
        assert!(!BopTestObjects::unify_faces());
    }

    #[test]
    fn test_run_parallel() {
        BopTestObjects::init();
        assert!(!BopTestObjects::run_parallel());
        BopTestObjects::set_run_parallel(true);
        assert!(BopTestObjects::run_parallel());
        BopTestObjects::set_run_parallel(false);
        assert!(!BopTestObjects::run_parallel());
    }

    #[test]
    fn test_fuzzy_value() {
        BopTestObjects::init();
        let default_fuzzy = BopTestObjects::fuzzy_value();
        assert!(default_fuzzy > 0.0);
        BopTestObjects::set_fuzzy_value(0.001);
        assert_eq!(BopTestObjects::fuzzy_value(), 0.001);
    }

    #[test]
    fn test_non_destructive() {
        BopTestObjects::init();
        assert!(!BopTestObjects::non_destructive());
        BopTestObjects::set_non_destructive(true);
        assert!(BopTestObjects::non_destructive());
    }

    #[test]
    fn test_glue_mode() {
        BopTestObjects::init();
        assert_eq!(BopTestObjects::glue(), GlueMode::Off);
        BopTestObjects::set_glue(GlueMode::Full);
        assert_eq!(BopTestObjects::glue(), GlueMode::Full);
    }

    #[test]
    fn test_check_inverted() {
        BopTestObjects::init();
        assert!(BopTestObjects::check_inverted());
        BopTestObjects::set_check_inverted(false);
        assert!(!BopTestObjects::check_inverted());
    }

    #[test]
    fn test_use_obb() {
        BopTestObjects::init();
        assert!(!BopTestObjects::use_obb());
        BopTestObjects::set_use_obb(true);
        assert!(BopTestObjects::use_obb());
    }

    #[test]
    fn test_unify_edges() {
        BopTestObjects::init();
        assert!(!BopTestObjects::unify_edges());
        BopTestObjects::set_unify_edges(true);
        assert!(BopTestObjects::unify_edges());
    }

    #[test]
    fn test_unify_faces() {
        BopTestObjects::init();
        assert!(!BopTestObjects::unify_faces());
        BopTestObjects::set_unify_faces(true);
        assert!(BopTestObjects::unify_faces());
    }

    #[test]
    fn test_angular() {
        BopTestObjects::init();
        let default_ang = BopTestObjects::angular();
        assert!(default_ang > 0.0);
        BopTestObjects::set_angular(0.001);
        assert_eq!(BopTestObjects::angular(), 0.001);
    }

    #[test]
    fn test_shapes_and_tools() {
        BopTestObjects::init();
        BopTestObjects::clear();
        assert_eq!(BopTestObjects::shapes().len(), 0);
        assert_eq!(BopTestObjects::tools().len(), 0);

        BopTestObjects::add_shape(1);
        BopTestObjects::add_shape(2);
        BopTestObjects::add_tool(3);
        assert_eq!(BopTestObjects::shapes().len(), 2);
        assert_eq!(BopTestObjects::tools().len(), 1);
    }
}
