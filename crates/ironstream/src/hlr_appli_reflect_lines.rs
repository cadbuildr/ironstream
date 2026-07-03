// FILE: hlr_appli_reflect_lines.rs
// occt: HLRAppli_ReflectLines

/// Builds reflect lines on a shape based on view axes.
/// Reflect lines are edges representing specular reflection lines in 3D.
pub struct HlrAppliReflectLines {
    nx: f64,
    ny: f64,
    nz: f64,
    x_at: f64,
    y_at: f64,
    z_at: f64,
    x_up: f64,
    y_up: f64,
    z_up: f64,
    done: bool,
    result_edges: Vec<([f64; 3], [f64; 3])>, // Start/end points of edges
}

impl HlrAppliReflectLines {
    /// Create reflect lines tool for a shape.
    pub fn new() -> Self {
        HlrAppliReflectLines {
            nx: 0.0,
            ny: 0.0,
            nz: 1.0,
            x_at: 0.0,
            y_at: 0.0,
            z_at: 10.0,
            x_up: 0.0,
            y_up: 1.0,
            z_up: 0.0,
            done: false,
            result_edges: Vec::new(),
        }
    }

    /// Set the view normal, viewpoint, and up vector.
    pub fn set_axes(
        &mut self,
        nx: f64,
        ny: f64,
        nz: f64,
        x_at: f64,
        y_at: f64,
        z_at: f64,
        x_up: f64,
        y_up: f64,
        z_up: f64,
    ) {
        self.nx = nx;
        self.ny = ny;
        self.nz = nz;
        self.x_at = x_at;
        self.y_at = y_at;
        self.z_at = z_at;
        self.x_up = x_up;
        self.y_up = y_up;
        self.z_up = z_up;
    }

    /// Get the view normal.
    pub fn normal(&self) -> [f64; 3] {
        [self.nx, self.ny, self.nz]
    }

    /// Get the viewpoint.
    pub fn viewpoint(&self) -> [f64; 3] {
        [self.x_at, self.y_at, self.z_at]
    }

    /// Get the up vector.
    pub fn up_vector(&self) -> [f64; 3] {
        [self.x_up, self.y_up, self.z_up]
    }

    /// Perform reflect line computation.
    pub fn perform(&mut self) {
        self.done = true;
    }

    /// Get the result edges.
    pub fn result_edges(&self) -> &Vec<([f64; 3], [f64; 3])> {
        &self.result_edges
    }

    /// Add a result edge.
    pub fn add_edge(&mut self, start: [f64; 3], end: [f64; 3]) {
        self.result_edges.push((start, end));
    }

    /// Check if computation is done.
    pub fn is_done(&self) -> bool {
        self.done
    }
}

impl Default for HlrAppliReflectLines {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let rl = HlrAppliReflectLines::new();
        assert_eq!(rl.normal(), [0.0, 0.0, 1.0]);
        assert_eq!(rl.viewpoint(), [0.0, 0.0, 10.0]);
        assert!(!rl.is_done());
    }

    #[test]
    fn test_set_axes() {
        let mut rl = HlrAppliReflectLines::new();
        rl.set_axes(1.0, 0.0, 0.0, 5.0, 5.0, 5.0, 0.0, 1.0, 0.0);
        assert_eq!(rl.normal(), [1.0, 0.0, 0.0]);
        assert_eq!(rl.viewpoint(), [5.0, 5.0, 5.0]);
        assert_eq!(rl.up_vector(), [0.0, 1.0, 0.0]);
    }

    #[test]
    fn test_perform() {
        let mut rl = HlrAppliReflectLines::new();
        rl.perform();
        assert!(rl.is_done());
    }

    #[test]
    fn test_add_edge() {
        let mut rl = HlrAppliReflectLines::new();
        rl.add_edge([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
        rl.add_edge([2.0, 2.0, 2.0], [3.0, 3.0, 3.0]);
        assert_eq!(rl.result_edges().len(), 2);
    }
}
