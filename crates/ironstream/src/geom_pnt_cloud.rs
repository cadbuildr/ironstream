// FILE: geom_pnt_cloud.rs
// occt-note: Graphic3d_ArrayOfPoints, RWMesh point cloud, spatial statistics

/// A point cloud with optional normals and colors.
#[derive(Clone, Debug, Default)]
pub struct PointCloud {
    pub points: Vec<[f64; 3]>,
    pub normals: Vec<[f64; 3]>,
    pub colors: Vec<[f32; 4]>,
}

impl PointCloud {
    pub fn new() -> Self { Self::default() }

    pub fn add_point(&mut self, p: [f64; 3]) { self.points.push(p); }

    pub fn add_point_with_normal(&mut self, p: [f64; 3], n: [f64; 3]) {
        self.points.push(p);
        self.normals.push(n);
    }

    pub fn add_point_with_color(&mut self, p: [f64; 3], color: [f32; 4]) {
        self.points.push(p);
        self.colors.push(color);
    }

    pub fn nb_points(&self) -> usize { self.points.len() }
    pub fn has_normals(&self) -> bool { self.normals.len() == self.points.len() }
    pub fn has_colors(&self) -> bool { self.colors.len() == self.points.len() }

    /// Compute axis-aligned bounding box.
    pub fn bounding_box(&self) -> Option<([f64; 3], [f64; 3])> {
        if self.points.is_empty() { return None; }
        let mut min = [f64::INFINITY; 3];
        let mut max = [f64::NEG_INFINITY; 3];
        for p in &self.points {
            for i in 0..3 {
                min[i] = min[i].min(p[i]);
                max[i] = max[i].max(p[i]);
            }
        }
        Some((min, max))
    }

    /// Compute centroid.
    pub fn centroid(&self) -> Option<[f64; 3]> {
        if self.points.is_empty() { return None; }
        let n = self.points.len() as f64;
        let cx: f64 = self.points.iter().map(|p| p[0]).sum::<f64>() / n;
        let cy: f64 = self.points.iter().map(|p| p[1]).sum::<f64>() / n;
        let cz: f64 = self.points.iter().map(|p| p[2]).sum::<f64>() / n;
        Some([cx, cy, cz])
    }

    /// Find k nearest neighbors to query point (brute force).
    pub fn k_nearest(&self, query: [f64; 3], k: usize) -> Vec<(usize, f64)> {
        let mut dists: Vec<(usize, f64)> = self.points.iter().enumerate().map(|(i, &p)| {
            let dx=query[0]-p[0]; let dy=query[1]-p[1]; let dz=query[2]-p[2];
            (i, dx*dx+dy*dy+dz*dz)
        }).collect();
        dists.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        dists.truncate(k);
        dists.iter().map(|&(i, d)| (i, d.sqrt())).collect()
    }

    /// Average point-to-point distance to centroid (spread measure).
    pub fn spread(&self) -> f64 {
        if let Some(c) = self.centroid() {
            let n = self.points.len() as f64;
            self.points.iter().map(|&p| {
                let dx=p[0]-c[0]; let dy=p[1]-c[1]; let dz=p[2]-c[2];
                (dx*dx+dy*dy+dz*dz).sqrt()
            }).sum::<f64>() / n
        } else { 0.0 }
    }

    /// Downsample using voxel grid: keep one point per voxel of given size.
    pub fn voxel_downsample(&self, voxel_size: f64) -> PointCloud {
        use std::collections::HashMap;
        let mut voxels: HashMap<(i64, i64, i64), [f64; 3]> = HashMap::new();
        for &p in &self.points {
            let key = (
                (p[0] / voxel_size).floor() as i64,
                (p[1] / voxel_size).floor() as i64,
                (p[2] / voxel_size).floor() as i64,
            );
            voxels.entry(key).or_insert(p);
        }
        let mut out = PointCloud::new();
        for (_, p) in voxels { out.add_point(p); }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_cloud_basic() {
        let mut pc = PointCloud::new();
        pc.add_point([0.0, 0.0, 0.0]);
        pc.add_point([1.0, 0.0, 0.0]);
        pc.add_point([0.0, 1.0, 0.0]);
        assert_eq!(pc.nb_points(), 3);
    }

    #[test]
    fn point_cloud_bbox() {
        let mut pc = PointCloud::new();
        pc.add_point([0.0,0.0,0.0]);
        pc.add_point([3.0,4.0,5.0]);
        let (min, max) = pc.bounding_box().unwrap();
        assert!((min[0]).abs() < 1e-10);
        assert!((max[2] - 5.0).abs() < 1e-10);
    }

    #[test]
    fn point_cloud_centroid() {
        let mut pc = PointCloud::new();
        pc.add_point([0.0,0.0,0.0]);
        pc.add_point([2.0,0.0,0.0]);
        let c = pc.centroid().unwrap();
        assert!((c[0] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn point_cloud_k_nearest() {
        let mut pc = PointCloud::new();
        pc.add_point([0.0,0.0,0.0]);
        pc.add_point([1.0,0.0,0.0]);
        pc.add_point([10.0,0.0,0.0]);
        let nn = pc.k_nearest([0.1, 0.0, 0.0], 2);
        assert_eq!(nn.len(), 2);
        assert_eq!(nn[0].0, 0);
    }

    #[test]
    fn point_cloud_voxel_downsample() {
        let mut pc = PointCloud::new();
        for i in 0..10 { pc.add_point([i as f64 * 0.01, 0.0, 0.0]); }
        let down = pc.voxel_downsample(0.1);
        assert!(down.nb_points() <= pc.nb_points());
    }
}
