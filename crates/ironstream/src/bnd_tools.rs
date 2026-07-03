// FILE: rust/ironstream/crates/ironstream/src/bnd_tools.rs

/// What was added when updating a bounding box.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BndLibAddResult {
    Added,
    Enlarged,
    NoChange,
}

/// Computed bounding box with gap.
// occt: Bnd_Box + BndLib tools
pub struct BndLibBox {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
    pub zmin: f64,
    pub zmax: f64,
    pub gap: f64,
    pub is_void: bool,
}

impl BndLibBox {
    /// Create a void (empty) bounding box.
    pub fn new_void() -> Self {
        Self {
            xmin: f64::MAX,
            xmax: f64::MIN,
            ymin: f64::MAX,
            ymax: f64::MIN,
            zmin: f64::MAX,
            zmax: f64::MIN,
            gap: 0.0,
            is_void: true,
        }
    }

    /// Create a bounding box from a sphere centered at (cx, cy, cz) with radius r and tolerance tol.
    /// min = center - r - tol, max = center + r + tol
    pub fn new_from_sphere(cx: f64, cy: f64, cz: f64, r: f64, tol: f64) -> Self {
        Self {
            xmin: cx - r - tol,
            xmax: cx + r + tol,
            ymin: cy - r - tol,
            ymax: cy + r + tol,
            zmin: cz - r - tol,
            zmax: cz + r + tol,
            gap: tol,
            is_void: false,
        }
    }

    /// Create a bounding box from a cylinder along Z with center (cx, cy), z extent [z_min, z_max],
    /// radius r, and tolerance tol.
    pub fn new_from_cylinder(
        cx: f64,
        cy: f64,
        z_min: f64,
        z_max: f64,
        r: f64,
        tol: f64,
    ) -> Self {
        Self {
            xmin: cx - r - tol,
            xmax: cx + r + tol,
            ymin: cy - r - tol,
            ymax: cy + r + tol,
            zmin: z_min - tol,
            zmax: z_max + tol,
            gap: tol,
            is_void: false,
        }
    }

    /// Create a bounding box from explicit corner points with tolerance.
    pub fn new_from_box_points(
        xmin: f64,
        xmax: f64,
        ymin: f64,
        ymax: f64,
        zmin: f64,
        zmax: f64,
        tol: f64,
    ) -> Self {
        Self {
            xmin: xmin - tol,
            xmax: xmax + tol,
            ymin: ymin - tol,
            ymax: ymax + tol,
            zmin: zmin - tol,
            zmax: zmax + tol,
            gap: tol,
            is_void: false,
        }
    }

    /// Expand the box to include the given point. Marks box as non-void.
    pub fn add_point(&mut self, x: f64, y: f64, z: f64) {
        if self.is_void {
            self.xmin = x;
            self.xmax = x;
            self.ymin = y;
            self.ymax = y;
            self.zmin = z;
            self.zmax = z;
            self.is_void = false;
        } else {
            if x < self.xmin {
                self.xmin = x;
            }
            if x > self.xmax {
                self.xmax = x;
            }
            if y < self.ymin {
                self.ymin = y;
            }
            if y > self.ymax {
                self.ymax = y;
            }
            if z < self.zmin {
                self.zmin = z;
            }
            if z > self.zmax {
                self.zmax = z;
            }
        }
    }

    /// Expand each face of the box outward by tol.
    pub fn enlarge(&mut self, tol: f64) {
        if !self.is_void {
            self.xmin -= tol;
            self.xmax += tol;
            self.ymin -= tol;
            self.ymax += tol;
            self.zmin -= tol;
            self.zmax += tol;
            self.gap += tol;
        }
    }

    /// Length of the space diagonal ||max - min||.
    pub fn diagonal(&self) -> f64 {
        if self.is_void {
            return 0.0;
        }
        let dx = self.xmax - self.xmin;
        let dy = self.ymax - self.ymin;
        let dz = self.zmax - self.zmin;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Returns true if the point lies outside the box (extended by gap).
    pub fn is_out_point(&self, x: f64, y: f64, z: f64) -> bool {
        if self.is_void {
            return true;
        }
        let g = self.gap;
        x < self.xmin - g
            || x > self.xmax + g
            || y < self.ymin - g
            || y > self.ymax + g
            || z < self.zmin - g
            || z > self.zmax + g
    }

    /// Returns true if the box is void (no points added).
    pub fn is_void(&self) -> bool {
        self.is_void
    }

    /// Returns the smallest box that contains both self and other.
    pub fn merged(&self, other: &Self) -> Self {
        if self.is_void {
            return Self {
                xmin: other.xmin,
                xmax: other.xmax,
                ymin: other.ymin,
                ymax: other.ymax,
                zmin: other.zmin,
                zmax: other.zmax,
                gap: other.gap,
                is_void: other.is_void,
            };
        }
        if other.is_void {
            return Self {
                xmin: self.xmin,
                xmax: self.xmax,
                ymin: self.ymin,
                ymax: self.ymax,
                zmin: self.zmin,
                zmax: self.zmax,
                gap: self.gap,
                is_void: self.is_void,
            };
        }
        Self {
            xmin: self.xmin.min(other.xmin),
            xmax: self.xmax.max(other.xmax),
            ymin: self.ymin.min(other.ymin),
            ymax: self.ymax.max(other.ymax),
            zmin: self.zmin.min(other.zmin),
            zmax: self.zmax.max(other.zmax),
            gap: self.gap.max(other.gap),
            is_void: false,
        }
    }
}

/// Computes the bounding sphere of a set of points.
pub struct BndLibSphereComputer {
    points: Vec<[f64; 3]>,
}

impl BndLibSphereComputer {
    /// Create a new, empty sphere computer.
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    /// Add a point to the set.
    pub fn add(&mut self, x: f64, y: f64, z: f64) {
        self.points.push([x, y, z]);
    }

    /// Compute the bounding sphere.
    /// Center is the centroid of all points; radius is the max distance from center to any point.
    /// Returns (cx, cy, cz, radius). If no points, returns (0, 0, 0, 0).
    pub fn compute(&self) -> (f64, f64, f64, f64) {
        let n = self.points.len();
        if n == 0 {
            return (0.0, 0.0, 0.0, 0.0);
        }

        let inv_n = 1.0 / n as f64;
        let cx = self.points.iter().map(|p| p[0]).sum::<f64>() * inv_n;
        let cy = self.points.iter().map(|p| p[1]).sum::<f64>() * inv_n;
        let cz = self.points.iter().map(|p| p[2]).sum::<f64>() * inv_n;

        let radius = self
            .points
            .iter()
            .map(|p| {
                let dx = p[0] - cx;
                let dy = p[1] - cy;
                let dz = p[2] - cz;
                (dx * dx + dy * dy + dz * dz).sqrt()
            })
            .fold(0.0_f64, f64::max);

        (cx, cy, cz, radius)
    }
}

impl Default for BndLibSphereComputer {
    fn default() -> Self {
        Self::new()
    }
}

/// Axis-aligned bounding box in 3D space storing min/max as `[f64; 3]` arrays.
// occt: BndLib_Box3d
#[derive(Clone, Copy, Debug)]
pub struct BndToolsBox3d {
    pub min: [f64; 3],
    pub max: [f64; 3],
    pub is_void: bool,
}

impl BndToolsBox3d {
    /// Create a void (empty) bounding box.
    pub fn new() -> Self {
        Self {
            min: [f64::MAX, f64::MAX, f64::MAX],
            max: [f64::MIN, f64::MIN, f64::MIN],
            is_void: true,
        }
    }

    /// Extend the box to include point `p`. Clears the void flag.
    pub fn add_point(&mut self, p: [f64; 3]) {
        if self.is_void {
            self.min = p;
            self.max = p;
            self.is_void = false;
        } else {
            for i in 0..3 {
                if p[i] < self.min[i] {
                    self.min[i] = p[i];
                }
                if p[i] > self.max[i] {
                    self.max[i] = p[i];
                }
            }
        }
    }

    /// Union another box into this one. If `other` is void this is a no-op.
    pub fn add_box(&mut self, other: &BndToolsBox3d) {
        if other.is_void {
            return;
        }
        if self.is_void {
            *self = *other;
            return;
        }
        for i in 0..3 {
            if other.min[i] < self.min[i] {
                self.min[i] = other.min[i];
            }
            if other.max[i] > self.max[i] {
                self.max[i] = other.max[i];
            }
        }
    }

    /// Expand each face of the box outward by `gap`. No-op when void.
    pub fn enlarge(&mut self, gap: f64) {
        if !self.is_void {
            for i in 0..3 {
                self.min[i] -= gap;
                self.max[i] += gap;
            }
        }
    }

    /// Returns `true` if no points or boxes have been added.
    pub fn is_void(&self) -> bool {
        self.is_void
    }

    /// Returns the minimum corner. Values are unspecified when void.
    pub fn min(&self) -> [f64; 3] {
        self.min
    }

    /// Returns the maximum corner. Values are unspecified when void.
    pub fn max(&self) -> [f64; 3] {
        self.max
    }

    /// Returns `[dx, dy, dz]` = `max - min`. All zeros when void.
    pub fn size(&self) -> [f64; 3] {
        if self.is_void {
            return [0.0, 0.0, 0.0];
        }
        [
            self.max[0] - self.min[0],
            self.max[1] - self.min[1],
            self.max[2] - self.min[2],
        ]
    }

    /// Returns the midpoint of min and max. All zeros when void.
    pub fn center(&self) -> [f64; 3] {
        if self.is_void {
            return [0.0, 0.0, 0.0];
        }
        [
            0.5 * (self.min[0] + self.max[0]),
            0.5 * (self.min[1] + self.max[1]),
            0.5 * (self.min[2] + self.max[2]),
        ]
    }
}

impl Default for BndToolsBox3d {
    fn default() -> Self {
        Self::new()
    }
}

/// Axis-aligned bounding box in 2D space storing min/max as `[f64; 2]` arrays.
// occt: BndLib_Box2d
#[derive(Clone, Copy, Debug)]
pub struct BndToolsBox2d {
    pub min: [f64; 2],
    pub max: [f64; 2],
    pub is_void: bool,
}

impl BndToolsBox2d {
    /// Create a void (empty) bounding box.
    pub fn new() -> Self {
        Self {
            min: [f64::MAX, f64::MAX],
            max: [f64::MIN, f64::MIN],
            is_void: true,
        }
    }

    /// Extend the box to include point `p`. Clears the void flag.
    pub fn add_point(&mut self, p: [f64; 2]) {
        if self.is_void {
            self.min = p;
            self.max = p;
            self.is_void = false;
        } else {
            for i in 0..2 {
                if p[i] < self.min[i] {
                    self.min[i] = p[i];
                }
                if p[i] > self.max[i] {
                    self.max[i] = p[i];
                }
            }
        }
    }

    /// Union another box into this one. If `other` is void this is a no-op.
    pub fn add_box(&mut self, other: &BndToolsBox2d) {
        if other.is_void {
            return;
        }
        if self.is_void {
            *self = *other;
            return;
        }
        for i in 0..2 {
            if other.min[i] < self.min[i] {
                self.min[i] = other.min[i];
            }
            if other.max[i] > self.max[i] {
                self.max[i] = other.max[i];
            }
        }
    }

    /// Expand each side of the box outward by `gap`. No-op when void.
    pub fn enlarge(&mut self, gap: f64) {
        if !self.is_void {
            for i in 0..2 {
                self.min[i] -= gap;
                self.max[i] += gap;
            }
        }
    }

    /// Returns `true` if no points or boxes have been added.
    pub fn is_void(&self) -> bool {
        self.is_void
    }

    /// Returns the minimum corner. Values are unspecified when void.
    pub fn min(&self) -> [f64; 2] {
        self.min
    }

    /// Returns the maximum corner. Values are unspecified when void.
    pub fn max(&self) -> [f64; 2] {
        self.max
    }

    /// Returns the midpoint of min and max. All zeros when void.
    pub fn center(&self) -> [f64; 2] {
        if self.is_void {
            return [0.0, 0.0];
        }
        [
            0.5 * (self.min[0] + self.max[0]),
            0.5 * (self.min[1] + self.max[1]),
        ]
    }
}

impl Default for BndToolsBox2d {
    fn default() -> Self {
        Self::new()
    }
}
