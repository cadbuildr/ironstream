// FILE: n_collection_cell_filter.rs
// occt: NCollection_CellFilter

/// Spatial cell filter for fast spatial searches.
pub struct CellFilter {
    cell_size: f64,
    cells: std::collections::HashMap<(i32, i32, i32), Vec<usize>>,
}

impl CellFilter {
    pub fn new(cell_size: f64) -> Self {
        CellFilter {
            cell_size,
            cells: std::collections::HashMap::new(),
        }
    }

    pub fn get_cell_size(&self) -> f64 {
        self.cell_size
    }

    fn get_cell_coords(&self, x: f64, y: f64, z: f64) -> (i32, i32, i32) {
        (
            (x / self.cell_size).floor() as i32,
            (y / self.cell_size).floor() as i32,
            (z / self.cell_size).floor() as i32,
        )
    }

    pub fn add_point(&mut self, index: usize, x: f64, y: f64, z: f64) {
        let coords = self.get_cell_coords(x, y, z);
        self.cells.entry(coords).or_insert_with(Vec::new).push(index);
    }

    pub fn get_points_in_sphere(
        &self,
        cx: f64,
        cy: f64,
        cz: f64,
        radius: f64,
    ) -> Vec<usize> {
        let mut result = Vec::new();
        let r_sq = radius * radius;

        let (x0, y0, z0) = self.get_cell_coords(cx - radius, cy - radius, cz - radius);
        let (x1, y1, z1) = self.get_cell_coords(cx + radius, cy + radius, cz + radius);

        for x in x0..=x1 {
            for y in y0..=y1 {
                for z in z0..=z1 {
                    if let Some(indices) = self.cells.get(&(x, y, z)) {
                        for &idx in indices {
                            result.push(idx);
                        }
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_filter() {
        let mut filter = CellFilter::new(1.0);
        assert_eq!(filter.get_cell_size(), 1.0);
        filter.add_point(0, 0.5, 0.5, 0.5);
        filter.add_point(1, 1.5, 1.5, 1.5);
        let points = filter.get_points_in_sphere(0.5, 0.5, 0.5, 1.0);
        assert!(points.contains(&0));
    }
}
