// FILE: convert_elementary_surface_to_b_spline_surface.rs

// occt: Convert_ElementarySurfaceToBSplineSurface

/// Represents a 3D point
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D { x, y, z }
    }

    pub fn origin() -> Self {
        Point3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

/// 2D array structure for storing surface poles (control points)
pub struct Array2D<T: Clone + Copy> {
    data: Vec<T>,
    nb_rows: usize,
    nb_cols: usize,
}

impl<T: Clone + Copy> Array2D<T> {
    /// Creates a new 2D array with given dimensions
    pub fn new(nb_rows: usize, nb_cols: usize, default: T) -> Self {
        Array2D {
            data: vec![default; nb_rows * nb_cols],
            nb_rows,
            nb_cols,
        }
    }

    /// Gets the element at (row, col) - 1-indexed
    pub fn get(&self, row: usize, col: usize) -> Option<T> {
        if row > 0 && row <= self.nb_rows && col > 0 && col <= self.nb_cols {
            let idx = (row - 1) * self.nb_cols + (col - 1);
            Some(self.data[idx])
        } else {
            None
        }
    }

    /// Sets the element at (row, col) - 1-indexed
    pub fn set(&mut self, row: usize, col: usize, value: T) {
        if row > 0 && row <= self.nb_rows && col > 0 && col <= self.nb_cols {
            let idx = (row - 1) * self.nb_cols + (col - 1);
            self.data[idx] = value;
        }
    }

    /// Returns number of rows
    pub fn nb_rows(&self) -> usize {
        self.nb_rows
    }

    /// Returns number of columns
    pub fn nb_cols(&self) -> usize {
        self.nb_cols
    }

    /// Resizes the array, trimming or padding as needed
    pub fn resize(&mut self, new_nb_rows: usize, new_nb_cols: usize, default: T) {
        if new_nb_rows == self.nb_rows && new_nb_cols == self.nb_cols {
            return;
        }

        let mut new_data = vec![default; new_nb_rows * new_nb_cols];

        let rows_to_copy = self.nb_rows.min(new_nb_rows);
        let cols_to_copy = self.nb_cols.min(new_nb_cols);

        for r in 0..rows_to_copy {
            for c in 0..cols_to_copy {
                let old_idx = r * self.nb_cols + c;
                let new_idx = r * new_nb_cols + c;
                new_data[new_idx] = self.data[old_idx];
            }
        }

        self.data = new_data;
        self.nb_rows = new_nb_rows;
        self.nb_cols = new_nb_cols;
    }

    /// Returns reference to raw data as slice
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    /// Iterates over rows
    pub fn iter_rows(&self) -> impl Iterator<Item = Vec<T>> + '_ {
        (0..self.nb_rows).map(move |r| {
            (0..self.nb_cols)
                .map(move |c| self.data[r * self.nb_cols + c])
                .collect()
        })
    }
}

/// Root class for algorithms which convert an elementary surface
/// (cylinder, cone, sphere or torus) into a BSpline surface.
pub struct ElementarySurfaceToBSplineSurface {
    poles: Array2D<Point3D>,
    weights: Array2D<f64>,
    u_knots: Vec<f64>,
    v_knots: Vec<f64>,
    u_mults: Vec<i32>,
    v_mults: Vec<i32>,
    u_degree: i32,
    v_degree: i32,
    is_u_periodic: bool,
    is_v_periodic: bool,
    nb_u_poles: usize,
    nb_v_poles: usize,
    nb_u_knots: usize,
    nb_v_knots: usize,
}

impl ElementarySurfaceToBSplineSurface {
    /// Creates a new elementary surface converter with allocated arrays.
    /// This is called by derived classes to initialize the base structure.
    ///
    /// # Parameters
    /// - `nb_u_poles`: Number of poles in U parametric direction
    /// - `nb_v_poles`: Number of poles in V parametric direction
    /// - `nb_u_knots`: Number of knots in U parametric direction
    /// - `nb_v_knots`: Number of knots in V parametric direction
    /// - `u_degree`: Degree in U parametric direction
    /// - `v_degree`: Degree in V parametric direction
    pub fn new(
        nb_u_poles: usize,
        nb_v_poles: usize,
        nb_u_knots: usize,
        nb_v_knots: usize,
        u_degree: i32,
        v_degree: i32,
    ) -> Self {
        ElementarySurfaceToBSplineSurface {
            poles: Array2D::new(nb_u_poles, nb_v_poles, Point3D::origin()),
            weights: Array2D::new(nb_u_poles, nb_v_poles, 1.0),
            u_knots: vec![0.0; nb_u_knots],
            v_knots: vec![0.0; nb_v_knots],
            u_mults: vec![0; nb_u_knots],
            v_mults: vec![0; nb_v_knots],
            u_degree,
            v_degree,
            is_u_periodic: false,
            is_v_periodic: false,
            nb_u_poles,
            nb_v_poles,
            nb_u_knots,
            nb_v_knots,
        }
    }

    /// Returns the degree in the U parametric direction.
    pub fn u_degree(&self) -> i32 {
        self.u_degree
    }

    /// Returns the degree in the V parametric direction.
    pub fn v_degree(&self) -> i32 {
        self.v_degree
    }

    /// Returns the number of poles in the U parametric direction.
    pub fn nb_u_poles(&self) -> usize {
        self.nb_u_poles
    }

    /// Returns the number of poles in the V parametric direction.
    pub fn nb_v_poles(&self) -> usize {
        self.nb_v_poles
    }

    /// Returns the number of knots in the U parametric direction.
    pub fn nb_u_knots(&self) -> usize {
        self.nb_u_knots
    }

    /// Returns the number of knots in the V parametric direction.
    pub fn nb_v_knots(&self) -> usize {
        self.nb_v_knots
    }

    /// Returns true if the surface is periodic in the U parametric direction.
    pub fn is_u_periodic(&self) -> bool {
        self.is_u_periodic
    }

    /// Returns true if the surface is periodic in the V parametric direction.
    pub fn is_v_periodic(&self) -> bool {
        self.is_v_periodic
    }

    /// Sets the U periodic flag
    pub fn set_u_periodic(&mut self, periodic: bool) {
        self.is_u_periodic = periodic;
    }

    /// Sets the V periodic flag
    pub fn set_v_periodic(&mut self, periodic: bool) {
        self.is_v_periodic = periodic;
    }

    /// Returns the pole at (u_index, v_index) (1-indexed)
    pub fn pole(&self, u_index: usize, v_index: usize) -> Option<Point3D> {
        if u_index < 1
            || u_index > self.nb_u_poles
            || v_index < 1
            || v_index > self.nb_v_poles
        {
            return None;
        }
        self.poles.get(u_index, v_index)
    }

    /// Sets the pole at (u_index, v_index) (1-indexed)
    pub fn set_pole(&mut self, u_index: usize, v_index: usize, pole: Point3D) {
        if u_index >= 1 && u_index <= self.nb_u_poles && v_index >= 1 && v_index <= self.nb_v_poles
        {
            self.poles.set(u_index, v_index, pole);
        }
    }

    /// Returns the weight of the pole at (u_index, v_index) (1-indexed)
    pub fn weight(&self, u_index: usize, v_index: usize) -> Option<f64> {
        if u_index < 1
            || u_index > self.nb_u_poles
            || v_index < 1
            || v_index > self.nb_v_poles
        {
            return None;
        }
        self.weights.get(u_index, v_index)
    }

    /// Sets the weight of the pole at (u_index, v_index) (1-indexed)
    pub fn set_weight(&mut self, u_index: usize, v_index: usize, weight: f64) {
        if u_index >= 1 && u_index <= self.nb_u_poles && v_index >= 1 && v_index <= self.nb_v_poles
        {
            self.weights.set(u_index, v_index, weight);
        }
    }

    /// Returns the U-knot at index (1-indexed)
    pub fn u_knot(&self, u_index: usize) -> Option<f64> {
        if u_index >= 1 && u_index <= self.nb_u_knots {
            Some(self.u_knots[u_index - 1])
        } else {
            None
        }
    }

    /// Sets the U-knot at index (1-indexed)
    pub fn set_u_knot(&mut self, u_index: usize, knot: f64) {
        if u_index >= 1 && u_index <= self.nb_u_knots {
            self.u_knots[u_index - 1] = knot;
        }
    }

    /// Returns the V-knot at index (1-indexed)
    pub fn v_knot(&self, v_index: usize) -> Option<f64> {
        if v_index >= 1 && v_index <= self.nb_v_knots {
            Some(self.v_knots[v_index - 1])
        } else {
            None
        }
    }

    /// Sets the V-knot at index (1-indexed)
    pub fn set_v_knot(&mut self, v_index: usize, knot: f64) {
        if v_index >= 1 && v_index <= self.nb_v_knots {
            self.v_knots[v_index - 1] = knot;
        }
    }

    /// Returns the multiplicity of the U-knot at index (1-indexed)
    pub fn u_multiplicity(&self, u_index: usize) -> Option<i32> {
        if u_index >= 1 && u_index <= self.nb_u_knots {
            Some(self.u_mults[u_index - 1])
        } else {
            None
        }
    }

    /// Sets the multiplicity of the U-knot at index (1-indexed)
    pub fn set_u_multiplicity(&mut self, u_index: usize, mult: i32) {
        if u_index >= 1 && u_index <= self.nb_u_knots {
            self.u_mults[u_index - 1] = mult;
        }
    }

    /// Returns the multiplicity of the V-knot at index (1-indexed)
    pub fn v_multiplicity(&self, v_index: usize) -> Option<i32> {
        if v_index >= 1 && v_index <= self.nb_v_knots {
            Some(self.v_mults[v_index - 1])
        } else {
            None
        }
    }

    /// Sets the multiplicity of the V-knot at index (1-indexed)
    pub fn set_v_multiplicity(&mut self, v_index: usize, mult: i32) {
        if v_index >= 1 && v_index <= self.nb_v_knots {
            self.v_mults[v_index - 1] = mult;
        }
    }

    /// Returns a reference to the poles array
    pub fn poles(&self) -> &Array2D<Point3D> {
        &self.poles
    }

    /// Returns a mutable reference to the poles array
    pub fn poles_mut(&mut self) -> &mut Array2D<Point3D> {
        &mut self.poles
    }

    /// Returns a reference to the weights array
    pub fn weights(&self) -> &Array2D<f64> {
        &self.weights
    }

    /// Returns a mutable reference to the weights array
    pub fn weights_mut(&mut self) -> &mut Array2D<f64> {
        &mut self.weights
    }

    /// Returns a reference to the U-knots array
    pub fn u_knots(&self) -> &[f64] {
        &self.u_knots
    }

    /// Returns a mutable reference to the U-knots array
    pub fn u_knots_mut(&mut self) -> &mut [f64] {
        &mut self.u_knots
    }

    /// Returns a reference to the V-knots array
    pub fn v_knots(&self) -> &[f64] {
        &self.v_knots
    }

    /// Returns a mutable reference to the V-knots array
    pub fn v_knots_mut(&mut self) -> &mut [f64] {
        &mut self.v_knots
    }

    /// Returns a reference to the U-multiplicities array
    pub fn u_multiplicities(&self) -> &[i32] {
        &self.u_mults
    }

    /// Returns a mutable reference to the U-multiplicities array
    pub fn u_multiplicities_mut(&mut self) -> &mut [i32] {
        &mut self.u_mults
    }

    /// Returns a reference to the V-multiplicities array
    pub fn v_multiplicities(&self) -> &[i32] {
        &self.v_mults
    }

    /// Returns a mutable reference to the V-multiplicities array
    pub fn v_multiplicities_mut(&mut self) -> &mut [i32] {
        &mut self.v_mults
    }

    /// Resizes internal arrays to match the actual sizes stored in nb_u_poles,
    /// nb_v_poles, nb_u_knots, and nb_v_knots. This is intended to be called
    /// at the end of derived class constructors when the base class constructor
    /// allocates arrays with maximum possible sizes but the derived constructor
    /// uses fewer elements.
    pub fn finalize(&mut self) {
        // Trim oversized arrays down to actual sizes
        if self.poles.nb_rows() != self.nb_u_poles || self.poles.nb_cols() != self.nb_v_poles {
            self.poles
                .resize(self.nb_u_poles, self.nb_v_poles, Point3D::origin());
        }
        if self.weights.nb_rows() != self.nb_u_poles || self.weights.nb_cols() != self.nb_v_poles
        {
            self.weights
                .resize(self.nb_u_poles, self.nb_v_poles, 1.0);
        }
        if self.u_knots.len() != self.nb_u_knots {
            self.u_knots.resize(self.nb_u_knots, 0.0);
        }
        if self.u_mults.len() != self.nb_u_knots {
            self.u_mults.resize(self.nb_u_knots, 0);
        }
        if self.v_knots.len() != self.nb_v_knots {
            self.v_knots.resize(self.nb_v_knots, 0.0);
        }
        if self.v_mults.len() != self.nb_v_knots {
            self.v_mults.resize(self.nb_v_knots, 0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point3d_creation() {
        let p = Point3D::new(1.0, 2.0, 3.0);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 3.0);
    }

    #[test]
    fn test_point3d_origin() {
        let p = Point3D::origin();
        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.0);
        assert_eq!(p.z, 0.0);
    }

    #[test]
    fn test_array2d_creation() {
        let arr: Array2D<f64> = Array2D::new(3, 4, 0.0);
        assert_eq!(arr.nb_rows(), 3);
        assert_eq!(arr.nb_cols(), 4);
    }

    #[test]
    fn test_array2d_get_set() {
        let mut arr: Array2D<f64> = Array2D::new(3, 3, 0.0);
        arr.set(1, 1, 5.0);
        assert_eq!(arr.get(1, 1), Some(5.0));
        assert_eq!(arr.get(2, 2), Some(0.0));
    }

    #[test]
    fn test_array2d_out_of_bounds() {
        let arr: Array2D<f64> = Array2D::new(3, 3, 0.0);
        assert_eq!(arr.get(0, 1), None);
        assert_eq!(arr.get(4, 1), None);
        assert_eq!(arr.get(1, 0), None);
        assert_eq!(arr.get(1, 4), None);
    }

    #[test]
    fn test_array2d_resize() {
        let mut arr: Array2D<f64> = Array2D::new(3, 3, 0.0);
        arr.set(1, 1, 5.0);
        arr.set(2, 2, 10.0);

        arr.resize(2, 2, 0.0);
        assert_eq!(arr.nb_rows(), 2);
        assert_eq!(arr.nb_cols(), 2);
        assert_eq!(arr.get(1, 1), Some(5.0));
        assert_eq!(arr.get(2, 2), Some(10.0));
    }

    #[test]
    fn test_surface_creation() {
        let surf = ElementarySurfaceToBSplineSurface::new(5, 6, 3, 4, 2, 3);
        assert_eq!(surf.nb_u_poles(), 5);
        assert_eq!(surf.nb_v_poles(), 6);
        assert_eq!(surf.nb_u_knots(), 3);
        assert_eq!(surf.nb_v_knots(), 4);
        assert_eq!(surf.u_degree(), 2);
        assert_eq!(surf.v_degree(), 3);
    }

    #[test]
    fn test_surface_periodicity() {
        let mut surf = ElementarySurfaceToBSplineSurface::new(5, 5, 3, 3, 2, 2);
        assert!(!surf.is_u_periodic());
        assert!(!surf.is_v_periodic());

        surf.set_u_periodic(true);
        surf.set_v_periodic(true);
        assert!(surf.is_u_periodic());
        assert!(surf.is_v_periodic());
    }

    #[test]
    fn test_surface_poles() {
        let mut surf = ElementarySurfaceToBSplineSurface::new(3, 3, 2, 2, 1, 1);
        let p = Point3D::new(1.0, 2.0, 3.0);
        surf.set_pole(1, 1, p);
        assert_eq!(surf.pole(1, 1), Some(p));
        assert_eq!(surf.pole(0, 1), None);
        assert_eq!(surf.pole(4, 1), None);
    }

    #[test]
    fn test_surface_weights() {
        let mut surf = ElementarySurfaceToBSplineSurface::new(3, 3, 2, 2, 1, 1);
        surf.set_weight(2, 2, 0.5);
        assert_eq!(surf.weight(2, 2), Some(0.5));
        assert_eq!(surf.weight(1, 1), Some(1.0)); // default weight
    }

    #[test]
    fn test_surface_u_knots() {
        let mut surf = ElementarySurfaceToBSplineSurface::new(3, 3, 3, 2, 1, 1);
        surf.set_u_knot(1, 0.0);
        surf.set_u_knot(2, 0.5);
        surf.set_u_knot(3, 1.0);
        assert_eq!(surf.u_knot(1), Some(0.0));
        assert_eq!(surf.u_knot(2), Some(0.5));
        assert_eq!(surf.u_knot(3), Some(1.0));
        assert_eq!(surf.u_knot(4), None);
    }

    #[test]
    fn test_surface_v_knots() {
        let mut surf = ElementarySurfaceToBSplineSurface::new(3, 3, 2, 3, 1, 1);
        surf.set_v_knot(1, 0.0);
        surf.set_v_knot(2, 0.5);
        surf.set_v_knot(3, 1.0);
        assert_eq!(surf.v_knot(1), Some(0.0));
        assert_eq!(surf.v_knot(2), Some(0.5));
        assert_eq!(surf.v_knot(3), Some(1.0));
    }

    #[test]
    fn test_surface_u_multiplicities() {
        let mut surf = ElementarySurfaceToBSplineSurface::new(3, 3, 3, 2, 2, 1);
        surf.set_u_multiplicity(1, 3);
        surf.set_u_multiplicity(2, 1);
        surf.set_u_multiplicity(3, 3);
        assert_eq!(surf.u_multiplicity(1), Some(3));
        assert_eq!(surf.u_multiplicity(2), Some(1));
        assert_eq!(surf.u_multiplicity(3), Some(3));
    }

    #[test]
    fn test_surface_v_multiplicities() {
        let mut surf = ElementarySurfaceToBSplineSurface::new(3, 3, 2, 3, 1, 2);
        surf.set_v_multiplicity(1, 3);
        surf.set_v_multiplicity(2, 1);
        surf.set_v_multiplicity(3, 3);
        assert_eq!(surf.v_multiplicity(1), Some(3));
        assert_eq!(surf.v_multiplicity(2), Some(1));
        assert_eq!(surf.v_multiplicity(3), Some(3));
    }

    #[test]
    fn test_surface_finalize() {
        let mut surf = ElementarySurfaceToBSplineSurface::new(10, 10, 10, 10, 2, 2);
        surf.nb_u_poles = 5;
        surf.nb_v_poles = 6;
        surf.nb_u_knots = 3;
        surf.nb_v_knots = 4;
        surf.finalize();
        assert_eq!(surf.poles().nb_rows(), 5);
        assert_eq!(surf.poles().nb_cols(), 6);
        assert_eq!(surf.u_knots().len(), 3);
        assert_eq!(surf.v_knots().len(), 4);
    }

    #[test]
    fn test_surface_array_access() {
        let surf = ElementarySurfaceToBSplineSurface::new(3, 3, 2, 2, 1, 1);
        let poles = surf.poles();
        assert_eq!(poles.nb_rows(), 3);
        assert_eq!(poles.nb_cols(), 3);

        let weights = surf.weights();
        assert_eq!(weights.nb_rows(), 3);
        assert_eq!(weights.nb_cols(), 3);
    }

    #[test]
    fn test_surface_mutability() {
        let mut surf = ElementarySurfaceToBSplineSurface::new(3, 3, 3, 3, 1, 1);

        // Set up some poles
        surf.set_pole(1, 1, Point3D::new(0.0, 0.0, 0.0));
        surf.set_pole(1, 2, Point3D::new(1.0, 0.0, 0.0));
        surf.set_pole(2, 1, Point3D::new(0.0, 1.0, 0.0));

        // Set up knots
        surf.set_u_knot(1, 0.0);
        surf.set_u_knot(2, 0.5);
        surf.set_u_knot(3, 1.0);

        // Verify
        assert_eq!(surf.pole(1, 1), Some(Point3D::new(0.0, 0.0, 0.0)));
        assert_eq!(surf.u_knot(2), Some(0.5));
    }
}
