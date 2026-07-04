// FILE: iges_graph_uniform_rect_grid.rs
// occt: IGESGraph_UniformRectGrid

#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Point2D { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Vector2D {
    pub fn new(x: f64, y: f64) -> Self {
        Vector2D { x, y }
    }
}

pub struct IGESGraphUniformRectGrid {
    nb_property_values: i32,
    is_it_finite: i32,
    is_it_line: i32,
    is_it_weighted: i32,
    grid_point: Point2D,
    grid_spacing: Vector2D,
    nb_points_x: i32,
    nb_points_y: i32,
}

impl IGESGraphUniformRectGrid {
    pub fn new() -> Self {
        IGESGraphUniformRectGrid {
            nb_property_values: 0,
            is_it_finite: 0,
            is_it_line: 0,
            is_it_weighted: 0,
            grid_point: Point2D::new(0.0, 0.0),
            grid_spacing: Vector2D::new(0.0, 0.0),
            nb_points_x: 0,
            nb_points_y: 0,
        }
    }

    pub fn init(
        &mut self,
        nb_props: i32,
        finite: i32,
        line: i32,
        weighted: i32,
        a_grid_point: Point2D,
        a_grid_spacing: Vector2D,
        points_x: i32,
        points_y: i32,
    ) {
        self.nb_property_values = nb_props;
        self.is_it_finite = finite;
        self.is_it_line = line;
        self.is_it_weighted = weighted;
        self.grid_point = a_grid_point;
        self.grid_spacing = a_grid_spacing;
        self.nb_points_x = points_x;
        self.nb_points_y = points_y;
    }

    pub fn nb_property_values(&self) -> i32 {
        self.nb_property_values
    }

    pub fn is_finite(&self) -> bool {
        self.is_it_finite == 1
    }

    pub fn is_line(&self) -> bool {
        self.is_it_line == 1
    }

    pub fn is_weighted(&self) -> bool {
        self.is_it_weighted == 0
    }

    pub fn grid_point(&self) -> Point2D {
        self.grid_point
    }

    pub fn grid_spacing(&self) -> Vector2D {
        self.grid_spacing
    }

    pub fn nb_points_x(&self) -> i32 {
        self.nb_points_x
    }

    pub fn nb_points_y(&self) -> i32 {
        self.nb_points_y
    }
}

impl Default for IGESGraphUniformRectGrid {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let grid = IGESGraphUniformRectGrid::new();
        assert_eq!(grid.nb_property_values(), 0);
        assert!(!grid.is_finite());
        assert!(!grid.is_line());
        // Per IGESGraph_UniformRectGrid.cxx: IsWeighted() == (isItWeighted == 0),
        // so a zero-initialized grid IS weighted (IGES weight flag 0 = weighted).
        assert!(grid.is_weighted());
    }

    #[test]
    fn test_init() {
        let mut grid = IGESGraphUniformRectGrid::new();
        let point = Point2D::new(10.0, 20.0);
        let spacing = Vector2D::new(5.0, 5.0);

        grid.init(9, 1, 1, 0, point, spacing, 10, 20);

        assert_eq!(grid.nb_property_values(), 9);
        assert!(grid.is_finite());
        assert!(grid.is_line());
        assert!(grid.is_weighted());
        assert_eq!(grid.nb_points_x(), 10);
        assert_eq!(grid.nb_points_y(), 20);
    }

    #[test]
    fn test_grid_point() {
        let mut grid = IGESGraphUniformRectGrid::new();
        let point = Point2D::new(15.5, 25.5);
        let spacing = Vector2D::new(1.0, 1.0);

        grid.init(9, 0, 0, 1, point, spacing, 0, 0);

        let result = grid.grid_point();
        assert_eq!(result.x, 15.5);
        assert_eq!(result.y, 25.5);
    }

    #[test]
    fn test_grid_spacing() {
        let mut grid = IGESGraphUniformRectGrid::new();
        let point = Point2D::new(0.0, 0.0);
        let spacing = Vector2D::new(2.5, 3.5);

        grid.init(9, 1, 1, 0, point, spacing, 5, 5);

        let result = grid.grid_spacing();
        assert_eq!(result.x, 2.5);
        assert_eq!(result.y, 3.5);
    }

    #[test]
    fn test_weighted_flag() {
        let mut grid = IGESGraphUniformRectGrid::new();
        let point = Point2D::new(0.0, 0.0);
        let spacing = Vector2D::new(1.0, 1.0);

        grid.init(9, 1, 1, 0, point, spacing, 1, 1);
        assert!(grid.is_weighted());

        grid.init(9, 1, 1, 1, point, spacing, 1, 1);
        assert!(!grid.is_weighted());
    }
}
