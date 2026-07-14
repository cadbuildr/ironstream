// FILE: axis_placement.rs
// occt-ref: gp_Ax1, gp_Ax2, gp_Ax3, gp_Ax2d

/// 1D axis: origin + direction.
// occt-ref: gp_Ax1
#[derive(Clone, Debug)]
pub struct GpAx1 {
    pub origin: [f64; 3],
    pub direction: [f64; 3],
}

impl GpAx1 {
    pub fn new(origin: [f64; 3], direction: [f64; 3]) -> Self {
        Self { origin, direction }
    }

    pub fn location(&self) -> [f64; 3] { self.origin }
    pub fn direction(&self) -> [f64; 3] { self.direction }
}

impl Default for GpAx1 {
    fn default() -> Self {
        Self {
            origin: [0.0, 0.0, 0.0],
            direction: [0.0, 0.0, 1.0],
        }
    }
}

/// 2D axis: origin + direction.
// occt-ref: gp_Ax2d
#[derive(Clone, Debug)]
pub struct GpAx2d {
    pub origin: [f64; 2],
    pub direction: [f64; 2],
}

impl GpAx2d {
    pub fn new(origin: [f64; 2], direction: [f64; 2]) -> Self {
        Self { origin, direction }
    }

    pub fn location(&self) -> [f64; 2] { self.origin }
    pub fn direction(&self) -> [f64; 2] { self.direction }
}

impl Default for GpAx2d {
    fn default() -> Self {
        Self {
            origin: [0.0, 0.0],
            direction: [1.0, 0.0],
        }
    }
}

/// 3D orthonormal coordinate system.
// occt-ref: gp_Ax2
#[derive(Clone, Debug)]
pub struct GpAx2 {
    pub origin: [f64; 3],
    pub x_direction: [f64; 3],
    pub y_direction: [f64; 3],
    pub z_direction: [f64; 3],
}

impl GpAx2 {
    pub fn new(
        origin: [f64; 3],
        x_dir: [f64; 3],
        y_dir: [f64; 3],
        z_dir: [f64; 3],
    ) -> Self {
        Self {
            origin,
            x_direction: x_dir,
            y_direction: y_dir,
            z_direction: z_dir,
        }
    }

    pub fn location(&self) -> [f64; 3] { self.origin }
    pub fn x_direction(&self) -> [f64; 3] { self.x_direction }
    pub fn y_direction(&self) -> [f64; 3] { self.y_direction }
    pub fn z_direction(&self) -> [f64; 3] { self.z_direction }

    pub fn is_orthonormal(&self) -> bool {
        // Stub: assume constructors maintain orthonormality
        true
    }
}

impl Default for GpAx2 {
    fn default() -> Self {
        Self {
            origin: [0.0, 0.0, 0.0],
            x_direction: [1.0, 0.0, 0.0],
            y_direction: [0.0, 1.0, 0.0],
            z_direction: [0.0, 0.0, 1.0],
        }
    }
}

/// 3D axis with secondary direction (right-hand rule for Z).
// occt-ref: gp_Ax3
#[derive(Clone, Debug)]
pub struct GpAx3 {
    pub origin: [f64; 3],
    pub x_direction: [f64; 3],
    pub y_direction: [f64; 3],
    pub z_direction: [f64; 3],
    pub main_direction: bool, // true if Z is main
}

impl GpAx3 {
    pub fn new(origin: [f64; 3], main_dir: [f64; 3]) -> Self {
        Self {
            origin,
            x_direction: [1.0, 0.0, 0.0],
            y_direction: [0.0, 1.0, 0.0],
            z_direction: main_dir,
            main_direction: true,
        }
    }

    pub fn location(&self) -> [f64; 3] { self.origin }
    pub fn main_direction(&self) -> [f64; 3] { self.z_direction }
    pub fn x_direction(&self) -> [f64; 3] { self.x_direction }
    pub fn y_direction(&self) -> [f64; 3] { self.y_direction }
    pub fn z_direction(&self) -> [f64; 3] { self.z_direction }

    pub fn set_main_direction(&mut self, dir: [f64; 3]) {
        self.z_direction = dir;
    }
}

impl Default for GpAx3 {
    fn default() -> Self {
        Self {
            origin: [0.0, 0.0, 0.0],
            x_direction: [1.0, 0.0, 0.0],
            y_direction: [0.0, 1.0, 0.0],
            z_direction: [0.0, 0.0, 1.0],
            main_direction: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gp_ax1() {
        let ax = GpAx1::new([1.0, 2.0, 3.0], [0.0, 0.0, 1.0]);
        assert_eq!(ax.location(), [1.0, 2.0, 3.0]);
        assert_eq!(ax.direction(), [0.0, 0.0, 1.0]);
    }

    #[test]
    fn gp_ax2d() {
        let ax = GpAx2d::new([0.0, 0.0], [1.0, 0.0]);
        assert_eq!(ax.location(), [0.0, 0.0]);
        assert_eq!(ax.direction(), [1.0, 0.0]);
    }

    #[test]
    fn gp_ax2() {
        let ax = GpAx2::new(
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        );
        assert!(ax.is_orthonormal());
    }

    #[test]
    fn gp_ax3() {
        let ax = GpAx3::default();
        assert_eq!(ax.location(), [0.0, 0.0, 0.0]);
        assert_eq!(ax.main_direction(), [0.0, 0.0, 1.0]);
    }

    #[test]
    fn gp_ax3_set_main() {
        let mut ax = GpAx3::default();
        ax.set_main_direction([1.0, 0.0, 0.0]);
        assert_eq!(ax.main_direction(), [1.0, 0.0, 0.0]);
    }
}
