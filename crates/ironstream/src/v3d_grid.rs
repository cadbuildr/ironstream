// FILE: rust/ironstream/crates/ironstream/src/v3d_grid.rs

// occt: Aspect_GridType
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GridType {
    Rectangular,
    Circular,
}

// occt: Aspect_GridDrawMode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GridDrawMode {
    Lines,
    Points,
}

// occt: V3d_RectangularGrid
#[derive(Clone, Debug)]
pub struct RectangularGrid {
    pub x_step: f64,
    pub y_step: f64,
    pub x_rotation: f64,
    pub x_origin: f64,
    pub y_origin: f64,
}

impl RectangularGrid {
    /// Create a new rectangular grid with the given X and Y step sizes.
    /// Rotation and origin default to zero.
    pub fn new(xs: f64, ys: f64) -> Self {
        RectangularGrid {
            x_step: xs,
            y_step: ys,
            x_rotation: 0.0,
            x_origin: 0.0,
            y_origin: 0.0,
        }
    }

    /// Set the rotation angle (in radians) of the grid around the Z axis.
    pub fn set_rotation(&mut self, r: f64) {
        self.x_rotation = r;
    }

    /// Set the 2-D origin offset of the grid in the view plane.
    pub fn set_origin(&mut self, x: f64, y: f64) {
        self.x_origin = x;
        self.y_origin = y;
    }

    /// Return the grid type (always Rectangular for this struct).
    pub fn display_mode(&self) -> GridType {
        GridType::Rectangular
    }
}

// occt: V3d_CircularGrid
#[derive(Clone, Debug)]
pub struct CircularGrid {
    pub radius_step: f64,
    pub division_count: u32,
    pub rotation: f64,
}

impl CircularGrid {
    /// Create a new circular grid with the given radius step and angular division count.
    /// Rotation defaults to zero.
    pub fn new(step: f64, divisions: u32) -> Self {
        CircularGrid {
            radius_step: step,
            division_count: divisions,
            rotation: 0.0,
        }
    }

    /// Set the rotation angle (in radians) applied to the circular grid.
    pub fn set_rotation(&mut self, r: f64) {
        self.rotation = r;
    }

    /// Return the radius of the i-th ring (zero-indexed).
    /// radius_at(0) == radius_step, radius_at(1) == 2 * radius_step, etc.
    pub fn radius_at(&self, i: u32) -> f64 {
        self.radius_step * (i + 1) as f64
    }
}

// occt: V3d_RectangularGrid // / V3d_CircularGrid display container
#[derive(Clone, Debug)]
pub struct GridDisplay {
    pub draw_mode: u8,
    pub grid_type: GridType,
}

impl GridDisplay {
    /// Create a new GridDisplay for the given grid type.
    /// draw_mode starts at 0 (inactive / hidden).
    pub fn new(gt: GridType) -> Self {
        GridDisplay {
            draw_mode: 0,
            grid_type: gt,
        }
    }

    /// Return true when the grid is set to be drawn (draw_mode != 0).
    pub fn is_active(&self) -> bool {
        self.draw_mode != 0
    }
}

// occt: V3d_RectangularGrid // / V3d_CircularGrid (unified internal representation)
pub struct V3dGrid {
    grid_type: GridType,
    draw_mode: GridDrawMode,
    x_step: f64,
    y_step: f64,
    x_origin: f64,
    y_origin: f64,
    angle: f64,
    rotation_angle: f64,
    is_active: bool,
    color: [f32; 3],
    elevation_color: [f32; 3],
}

impl V3dGrid {
    /// Create a rectangular grid with the given X and Y step sizes.
    pub fn rectangular(x_step: f64, y_step: f64) -> Self {
        V3dGrid {
            grid_type: GridType::Rectangular,
            draw_mode: GridDrawMode::Lines,
            x_step,
            y_step,
            x_origin: 0.0,
            y_origin: 0.0,
            angle: 0.0,
            rotation_angle: 0.0,
            is_active: false,
            color: [0.5, 0.5, 0.5],
            elevation_color: [0.0, 0.0, 1.0],
        }
    }

    /// Create a circular grid with the given radius step and number of angular divisions.
    /// Stored as x_step = radius_step, y_step = nb_divisions as f64.
    pub fn circular(radius_step: f64, nb_divisions: u32) -> Self {
        V3dGrid {
            grid_type: GridType::Circular,
            draw_mode: GridDrawMode::Lines,
            x_step: radius_step,
            y_step: nb_divisions as f64,
            x_origin: 0.0,
            y_origin: 0.0,
            angle: 0.0,
            rotation_angle: 0.0,
            is_active: false,
            color: [0.5, 0.5, 0.5],
            elevation_color: [0.0, 0.0, 1.0],
        }
    }

    pub fn grid_type(&self) -> GridType {
        self.grid_type
    }

    pub fn draw_mode(&self) -> GridDrawMode {
        self.draw_mode
    }

    pub fn set_draw_mode(&mut self, mode: GridDrawMode) {
        self.draw_mode = mode;
    }

    pub fn x_step(&self) -> f64 {
        self.x_step
    }

    pub fn y_step(&self) -> f64 {
        self.y_step
    }

    pub fn set_origin(&mut self, x: f64, y: f64) {
        self.x_origin = x;
        self.y_origin = y;
    }

    pub fn origin(&self) -> (f64, f64) {
        (self.x_origin, self.y_origin)
    }

    pub fn angle(&self) -> f64 {
        self.angle
    }

    pub fn set_rotation_angle(&mut self, angle: f64) {
        self.rotation_angle = angle;
    }

    pub fn rotation_angle(&self) -> f64 {
        self.rotation_angle
    }

    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn set_color(&mut self, color: [f32; 3]) {
        self.color = color;
    }

    pub fn color(&self) -> [f32; 3] {
        self.color
    }

    pub fn set_elevation_color(&mut self, color: [f32; 3]) {
        self.elevation_color = color;
    }

    pub fn elevation_color(&self) -> [f32; 3] {
        self.elevation_color
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangular_grid_new_defaults() {
        let g = RectangularGrid::new(1.0, 2.0);
        assert_eq!(g.x_step, 1.0);
        assert_eq!(g.y_step, 2.0);
        assert_eq!(g.x_rotation, 0.0);
        assert_eq!(g.x_origin, 0.0);
        assert_eq!(g.y_origin, 0.0);
    }

    #[test]
    fn test_rectangular_grid_set_rotation() {
        let mut g = RectangularGrid::new(1.0, 1.0);
        g.set_rotation(0.5);
        assert_eq!(g.x_rotation, 0.5);
    }

    #[test]
    fn test_rectangular_grid_set_origin() {
        let mut g = RectangularGrid::new(1.0, 1.0);
        g.set_origin(3.0, 4.0);
        assert_eq!(g.x_origin, 3.0);
        assert_eq!(g.y_origin, 4.0);
    }

    #[test]
    fn test_rectangular_grid_display_mode() {
        let g = RectangularGrid::new(1.0, 1.0);
        assert_eq!(g.display_mode(), GridType::Rectangular);
    }

    #[test]
    fn test_circular_grid_new_defaults() {
        let g = CircularGrid::new(0.5, 8);
        assert_eq!(g.radius_step, 0.5);
        assert_eq!(g.division_count, 8);
        assert_eq!(g.rotation, 0.0);
    }

    #[test]
    fn test_circular_grid_set_rotation() {
        let mut g = CircularGrid::new(1.0, 6);
        g.set_rotation(1.2);
        assert_eq!(g.rotation, 1.2);
    }

    #[test]
    fn test_circular_grid_radius_at() {
        let g = CircularGrid::new(2.0, 4);
        assert_eq!(g.radius_at(0), 2.0);
        assert_eq!(g.radius_at(1), 4.0);
        assert_eq!(g.radius_at(2), 6.0);
    }

    #[test]
    fn test_grid_display_new_inactive() {
        let d = GridDisplay::new(GridType::Rectangular);
        assert_eq!(d.draw_mode, 0);
        assert_eq!(d.grid_type, GridType::Rectangular);
        assert!(!d.is_active());
    }

    #[test]
    fn test_grid_display_active_when_draw_mode_nonzero() {
        let mut d = GridDisplay::new(GridType::Circular);
        d.draw_mode = 1;
        assert!(d.is_active());
    }

    #[test]
    fn test_grid_type_variants_distinct() {
        assert_ne!(GridType::Rectangular, GridType::Circular);
    }

    #[test]
    fn test_v3d_grid_rectangular_defaults() {
        let g = V3dGrid::rectangular(1.0, 2.0);
        assert_eq!(g.grid_type(), GridType::Rectangular);
        assert_eq!(g.x_step(), 1.0);
        assert_eq!(g.y_step(), 2.0);
        assert!(!g.is_active());
    }

    #[test]
    fn test_v3d_grid_circular_defaults() {
        let g = V3dGrid::circular(0.5, 6);
        assert_eq!(g.grid_type(), GridType::Circular);
        assert_eq!(g.x_step(), 0.5);
    }

    #[test]
    fn test_v3d_grid_activate_deactivate() {
        let mut g = V3dGrid::rectangular(1.0, 1.0);
        g.activate();
        assert!(g.is_active());
        g.deactivate();
        assert!(!g.is_active());
    }
}
