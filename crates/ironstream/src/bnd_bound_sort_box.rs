// FILE: bnd_bound_sort_box.rs
//! A tool to compare a bounding box or a plane with a set of bounding boxes.
//! Ported from OpenCascade Bnd_BoundSortBox class.

// occt: Bnd_BoundSortBox

use std::collections::HashMap;

/// A 3D axis-aligned bounding box
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BndBox {
    xmin: f64,
    ymin: f64,
    zmin: f64,
    xmax: f64,
    ymax: f64,
    zmax: f64,
    is_void: bool,
}

impl BndBox {
    /// Creates a void (empty) bounding box
    pub fn new() -> Self {
        BndBox {
            xmin: 0.0,
            ymin: 0.0,
            zmin: 0.0,
            xmax: 0.0,
            ymax: 0.0,
            zmax: 0.0,
            is_void: true,
        }
    }

    /// Updates the bounding box with new extent values
    pub fn update(&mut self, xmin: f64, ymin: f64, zmin: f64, xmax: f64, ymax: f64, zmax: f64) {
        self.xmin = xmin;
        self.ymin = ymin;
        self.zmin = zmin;
        self.xmax = xmax;
        self.ymax = ymax;
        self.zmax = zmax;
        self.is_void = false;
    }

    /// Checks if this box is void (empty)
    pub fn is_void(&self) -> bool {
        self.is_void
    }

    /// Checks if this box and another box are disjoint
    pub fn is_out(&self, other: &BndBox) -> bool {
        if self.is_void || other.is_void {
            return true;
        }
        self.xmax < other.xmin
            || self.xmin > other.xmax
            || self.ymax < other.ymin
            || self.ymin > other.ymax
            || self.zmax < other.zmin
            || self.zmin > other.zmax
    }

    /// Checks if this box intersects another box (including touching)
    pub fn intersects(&self, other: &BndBox) -> bool {
        if self.is_void || other.is_void {
            return false;
        }
        !(self.is_out(other))
    }

    /// Gets the minimum extents
    pub fn min(&self) -> (f64, f64, f64) {
        (self.xmin, self.ymin, self.zmin)
    }

    /// Gets the maximum extents
    pub fn max(&self) -> (f64, f64, f64) {
        (self.xmax, self.ymax, self.zmax)
    }
}

impl Default for BndBox {
    fn default() -> Self {
        Self::new()
    }
}

/// A tool to sort bounding boxes and find intersections with a query box.
///
/// Uses spatial partitioning (voxel grid) to efficiently find boxes that intersect
/// a query box or plane.
pub struct BndBoundSortBox {
    /// The enclosing bounding box containing all boxes to be sorted
    enclosing_box: BndBox,
    /// Array of bounding boxes to be sorted
    boxes: Vec<BndBox>,
    /// Coefficient for X direction (boxes per unit distance)
    coeff_x: f64,
    /// Coefficient for Y direction
    coeff_y: f64,
    /// Coefficient for Z direction
    coeff_z: f64,
    /// Resolution of voxel grid (number of voxels per dimension)
    resolution: i32,
    /// Last result of Compare method
    last_result: Vec<i32>,
    /// Indices of boxes that are larger than the enclosing box
    large_boxes: Vec<i32>,
    /// Voxel grid data: maps voxel index to list of box indices
    voxel_grid: HashMap<(i32, i32, i32), Vec<i32>>,
}

impl BndBoundSortBox {
    /// Creates an empty comparison algorithm for bounding boxes
    pub fn new() -> Self {
        BndBoundSortBox {
            enclosing_box: BndBox::new(),
            boxes: Vec::new(),
            coeff_x: 0.0,
            coeff_y: 0.0,
            coeff_z: 0.0,
            resolution: 8,
            last_result: Vec::new(),
            large_boxes: Vec::new(),
            voxel_grid: HashMap::new(),
        }
    }

    /// Initializes with a set of boxes
    pub fn initialize(&mut self, boxes: &[BndBox]) {
        self.boxes = boxes.to_vec();
        self.compute_enclosing_box();
        self.calculate_coefficients();
        self.sort_boxes();
    }

    /// Initializes with an explicit enclosing box and set of boxes
    pub fn initialize_with_enclosing(&mut self, enclosing_box: BndBox, boxes: &[BndBox]) {
        self.enclosing_box = enclosing_box;
        self.boxes = boxes.to_vec();
        self.calculate_coefficients();
        self.sort_boxes();
    }

    /// Initializes with enclosing box and expected count (for manual addition)
    pub fn initialize_with_count(&mut self, enclosing_box: BndBox, _count: i32) {
        self.enclosing_box = enclosing_box;
        self.boxes.clear();
        self.calculate_coefficients();
    }

    /// Adds a box at the given index (1-based to match OCCT)
    pub fn add(&mut self, bbox: BndBox, index: i32) {
        let idx = (index - 1) as usize;
        if idx >= self.boxes.len() {
            self.boxes.resize(idx + 1, BndBox::new());
        }
        self.boxes[idx] = bbox;
        self.voxel_grid.clear();
        self.sort_boxes();
    }

    /// Compares with a box and returns indices of intersecting boxes
    pub fn compare(&mut self, query_box: &BndBox) -> Vec<i32> {
        self.last_result.clear();

        if query_box.is_void() {
            return Vec::new();
        }

        // Always check large boxes
        for &box_idx in &self.large_boxes {
            if self.boxes[box_idx as usize].intersects(query_box) {
                self.last_result.push(box_idx + 1); // Convert to 1-based index
            }
        }

        // Get voxel range for query box
        let voxel_bounds = self.get_bounding_voxels(query_box);
        let (min_x, min_y, min_z, max_x, max_y, max_z) = voxel_bounds;

        // Collect candidate boxes from voxel grid
        let mut candidates = std::collections::HashSet::new();

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                for z in min_z..=max_z {
                    if let Some(boxes_in_voxel) = self.voxel_grid.get(&(x, y, z)) {
                        for &box_idx in boxes_in_voxel {
                            candidates.insert(box_idx);
                        }
                    }
                }
            }
        }

        // Check each candidate for actual intersection
        for box_idx in candidates {
            if self.boxes[box_idx as usize].intersects(query_box) {
                self.last_result.push(box_idx + 1); // Convert to 1-based index
            }
        }

        // Remove duplicates and sort
        self.last_result.sort_unstable();
        self.last_result.dedup();

        self.last_result.clone()
    }

    /// Computes the enclosing box from all input boxes
    fn compute_enclosing_box(&mut self) {
        if self.boxes.is_empty() {
            self.enclosing_box = BndBox::new();
            return;
        }

        let first_valid = self.boxes.iter().find(|b| !b.is_void());
        if let Some(first) = first_valid {
            let (xmin, ymin, zmin) = first.min();
            let (xmax, ymax, zmax) = first.max();
            let mut emin = (xmin, ymin, zmin);
            let mut emax = (xmax, ymax, zmax);

            for b in &self.boxes {
                if !b.is_void() {
                    let (bxmin, bymin, bzmin) = b.min();
                    let (bxmax, bymax, bzmax) = b.max();
                    emin.0 = emin.0.min(bxmin);
                    emin.1 = emin.1.min(bymin);
                    emin.2 = emin.2.min(bzmin);
                    emax.0 = emax.0.max(bxmax);
                    emax.1 = emax.1.max(bymax);
                    emax.2 = emax.2.max(bzmax);
                }
            }

            self.enclosing_box
                .update(emin.0, emin.1, emin.2, emax.0, emax.1, emax.2);
        }
    }

    /// Precalculates coefficients for voxel grid mapping
    fn calculate_coefficients(&mut self) {
        if self.enclosing_box.is_void() {
            self.coeff_x = 0.0;
            self.coeff_y = 0.0;
            self.coeff_z = 0.0;
            return;
        }

        // Determine resolution based on box count
        self.resolution = if self.boxes.len() > 1000 {
            32
        } else if self.boxes.len() > 100 {
            16
        } else {
            8
        };

        let (xmin, ymin, zmin) = self.enclosing_box.min();
        let (xmax, ymax, zmax) = self.enclosing_box.max();

        let dx = (xmax - xmin).abs();
        let dy = (ymax - ymin).abs();
        let dz = (zmax - zmin).abs();

        const EPS: f64 = 1e-10;
        self.coeff_x = if dx > EPS {
            self.resolution as f64 / dx
        } else {
            0.0
        };
        self.coeff_y = if dy > EPS {
            self.resolution as f64 / dy
        } else {
            0.0
        };
        self.coeff_z = if dz > EPS {
            self.resolution as f64 / dz
        } else {
            0.0
        };
    }

    /// Sorts boxes into voxel grid
    fn sort_boxes(&mut self) {
        self.voxel_grid.clear();
        self.large_boxes.clear();

        let (enc_xmin, enc_ymin, enc_zmin) = self.enclosing_box.min();
        let (enc_xmax, enc_ymax, enc_zmax) = self.enclosing_box.max();

        for (idx, bbox) in self.boxes.iter().enumerate() {
            if bbox.is_void() {
                continue;
            }

            let voxel_bounds = self.get_bounding_voxels_for_box(*bbox);
            let (min_x, min_y, min_z, max_x, max_y, max_z) = voxel_bounds;

            // Check if box is larger than enclosing box
            let (bxmin, bymin, bzmin) = bbox.min();
            let (bxmax, bymax, bzmax) = bbox.max();

            if bxmin <= enc_xmin && bymin <= enc_ymin && bzmin <= enc_zmin
                && bxmax >= enc_xmax && bymax >= enc_ymax && bzmax >= enc_zmax
            {
                self.large_boxes.push(idx as i32);
            } else {
                // Add to voxel grid
                for x in min_x..=max_x {
                    for y in min_y..=max_y {
                        for z in min_z..=max_z {
                            self.voxel_grid
                                .entry((x, y, z))
                                .or_insert_with(Vec::new)
                                .push(idx as i32);
                        }
                    }
                }
            }
        }
    }

    /// Returns voxel indices for the min and max points of a box
    fn get_bounding_voxels(&self, bbox: &BndBox) -> (i32, i32, i32, i32, i32, i32) {
        self.get_bounding_voxels_for_box(*bbox)
    }

    fn get_bounding_voxels_for_box(&self, bbox: BndBox) -> (i32, i32, i32, i32, i32, i32) {
        let (enc_xmin, enc_ymin, enc_zmin) = self.enclosing_box.min();

        let (bxmin, bymin, bzmin) = bbox.min();
        let (bxmax, bymax, bzmax) = bbox.max();

        let min_x =
            ((bxmin - enc_xmin) * self.coeff_x).floor() as i32;
        let min_y =
            ((bymin - enc_ymin) * self.coeff_y).floor() as i32;
        let min_z =
            ((bzmin - enc_zmin) * self.coeff_z).floor() as i32;
        let max_x =
            ((bxmax - enc_xmin) * self.coeff_x).floor() as i32;
        let max_y =
            ((bymax - enc_ymin) * self.coeff_y).floor() as i32;
        let max_z =
            ((bzmax - enc_zmin) * self.coeff_z).floor() as i32;

        let clamped_min_x = min_x.max(0).min(self.resolution - 1);
        let clamped_min_y = min_y.max(0).min(self.resolution - 1);
        let clamped_min_z = min_z.max(0).min(self.resolution - 1);
        let clamped_max_x = max_x.max(0).min(self.resolution - 1);
        let clamped_max_y = max_y.max(0).min(self.resolution - 1);
        let clamped_max_z = max_z.max(0).min(self.resolution - 1);

        (
            clamped_min_x,
            clamped_min_y,
            clamped_min_z,
            clamped_max_x,
            clamped_max_y,
            clamped_max_z,
        )
    }
}

impl Default for BndBoundSortBox {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_box(xmin: f64, ymin: f64, zmin: f64, xmax: f64, ymax: f64, zmax: f64) -> BndBox {
        let mut b = BndBox::new();
        b.update(xmin, ymin, zmin, xmax, ymax, zmax);
        b
    }

    #[test]
    fn test_initialize_with_boxes() {
        let small_box = create_box(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        let large_box = create_box(-10.0, -10.0, -10.0, 10.0, 10.0, 10.0);
        let offset_box = create_box(5.0, 5.0, 5.0, 7.0, 7.0, 7.0);
        let non_intersecting = create_box(20.0, 20.0, 20.0, 30.0, 30.0, 30.0);

        let boxes = vec![small_box, large_box, offset_box, non_intersecting];
        let mut sorter = BndBoundSortBox::new();
        sorter.initialize(&boxes);

        let test_box = create_box(0.5, 0.5, 0.5, 1.5, 1.5, 1.5);
        let result = sorter.compare(&test_box);

        // Should find small_box (index 1) and large_box (index 2)
        assert_eq!(result.len(), 2);
        assert!(result.contains(&1));
        assert!(result.contains(&2));
    }

    #[test]
    fn test_initialize_with_enclosing_box() {
        let small_box = create_box(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        let large_box = create_box(-10.0, -10.0, -10.0, 10.0, 10.0, 10.0);
        let offset_box = create_box(5.0, 5.0, 5.0, 7.0, 7.0, 7.0);
        let non_intersecting = create_box(20.0, 20.0, 20.0, 30.0, 30.0, 30.0);

        let boxes = vec![small_box, large_box, offset_box, non_intersecting];
        let global_box = create_box(-20.0, -20.0, -20.0, 40.0, 40.0, 40.0);

        let mut sorter = BndBoundSortBox::new();
        sorter.initialize_with_enclosing(global_box, &boxes);

        let test_box = offset_box;
        let result = sorter.compare(&test_box);

        // Should find large_box (index 2) and offset_box (index 3)
        assert_eq!(result.len(), 2);
        assert!(result.contains(&2));
        assert!(result.contains(&3));
    }

    #[test]
    fn test_initialize_with_count() {
        let global_box = create_box(-20.0, -20.0, -20.0, 40.0, 40.0, 40.0);
        let mut sorter = BndBoundSortBox::new();
        sorter.initialize_with_count(global_box, 3);

        let small_box = create_box(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        let large_box = create_box(-10.0, -10.0, -10.0, 10.0, 10.0, 10.0);
        let non_intersecting = create_box(20.0, 20.0, 20.0, 30.0, 30.0, 30.0);

        sorter.add(small_box, 1);
        sorter.add(large_box, 2);
        sorter.add(non_intersecting, 3);

        let test_box = create_box(-5.0, -5.0, -5.0, -2.0, -2.0, -2.0);
        let result = sorter.compare(&test_box);

        // Should find only large_box (index 2)
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], 2);
    }

    #[test]
    fn test_void_boxes() {
        let void_box = BndBox::new();
        let small_box = create_box(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);

        let boxes = vec![void_box, small_box];
        let mut sorter = BndBoundSortBox::new();
        sorter.initialize(&boxes);

        let test_box = small_box;
        let result = sorter.compare(&test_box);

        // Should find only small_box (index 2)
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], 2);
    }

    #[test]
    fn test_touching_boxes() {
        let box1 = create_box(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        let box2 = create_box(1.0, 1.0, 1.0, 2.0, 2.0, 2.0); // Touches at (1,1,1)

        let boxes = vec![box1, box2];
        let mut sorter = BndBoundSortBox::new();
        sorter.initialize(&boxes);

        let result1 = sorter.compare(&box1);
        // Should find both boxes (touching counts as intersection)
        assert_eq!(result1.len(), 2);
    }

    #[test]
    fn test_disjoint_boxes() {
        let small_box = create_box(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        let far_box = create_box(100.0, 100.0, 100.0, 110.0, 110.0, 110.0);

        let boxes = vec![small_box, far_box];
        let enclosing = create_box(-10.0, -10.0, -10.0, 120.0, 120.0, 120.0);

        let mut sorter = BndBoundSortBox::new();
        sorter.initialize_with_enclosing(enclosing, &boxes);

        let test_box = create_box(0.5, 0.5, 0.5, 1.5, 1.5, 1.5);
        let result = sorter.compare(&test_box);

        // Should find only small_box (index 1)
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], 1);
    }

    #[test]
    fn test_void_query() {
        let small_box = create_box(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        let large_box = create_box(-10.0, -10.0, -10.0, 10.0, 10.0, 10.0);

        let boxes = vec![small_box, large_box];
        let mut sorter = BndBoundSortBox::new();
        sorter.initialize(&boxes);

        let void_box = BndBox::new();
        let result = sorter.compare(&void_box);

        // Should find nothing when querying with void box
        assert_eq!(result.len(), 0);
    }
}
