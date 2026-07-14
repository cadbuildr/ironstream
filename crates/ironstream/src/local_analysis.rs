// FILE: local_analysis.rs
// occt-ref: LocalAnalysis_Curvature, LocalAnalysis_Tangent, LocalAnalysis_SurfaceCurvature

/// Curvature analysis for a curve point.
// occt-ref: LocalAnalysis_Curvature
#[derive(Clone, Debug)]
pub struct LocalAnalysisCurvature {
    pub curvature: f64,
    pub radius: f64,
    pub torsion: f64,
    pub is_inflection: bool,
}

impl LocalAnalysisCurvature {
    pub fn new(curv: f64, torsion: f64) -> Self {
        let radius = if curv.abs() > 1e-10 { 1.0 / curv.abs() } else { f64::INFINITY };
        Self {
            curvature: curv,
            radius,
            torsion,
            is_inflection: curv.abs() < 1e-10,
        }
    }

    pub fn curvature(&self) -> f64 { self.curvature }
    pub fn radius_of_curvature(&self) -> f64 { self.radius }
    pub fn torsion(&self) -> f64 { self.torsion }
    pub fn is_inflection_point(&self) -> bool { self.is_inflection }
}

impl Default for LocalAnalysisCurvature {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

/// Tangent analysis: direction and continuity.
// occt-ref: LocalAnalysis_Tangent
#[derive(Clone, Debug)]
pub struct LocalAnalysisTangent {
    pub direction: [f64; 3],
    pub continuity_order: usize,  // 0=C0, 1=C1, 2=C2, etc
}

impl LocalAnalysisTangent {
    pub fn new(dir: [f64; 3], cont: usize) -> Self {
        Self {
            direction: dir,
            continuity_order: cont,
        }
    }

    pub fn tangent_direction(&self) -> [f64; 3] { self.direction }
    pub fn continuity(&self) -> usize { self.continuity_order }
    pub fn is_continuous(&self, order: usize) -> bool { self.continuity_order >= order }
}

impl Default for LocalAnalysisTangent {
    fn default() -> Self {
        Self::new([0.0, 0.0, 1.0], 0)
    }
}

/// Principal curvatures of a surface.
// occt-ref: LocalAnalysis_SurfaceCurvature
#[derive(Clone, Debug)]
pub struct LocalAnalysisSurfaceCurvature {
    pub principal_curvature_1: f64,
    pub principal_curvature_2: f64,
    pub mean_curvature: f64,
    pub gaussian_curvature: f64,
    pub principal_direction_1: [f64; 3],
    pub principal_direction_2: [f64; 3],
}

impl LocalAnalysisSurfaceCurvature {
    pub fn new(k1: f64, k2: f64) -> Self {
        let mean = (k1 + k2) / 2.0;
        let gaussian = k1 * k2;
        Self {
            principal_curvature_1: k1,
            principal_curvature_2: k2,
            mean_curvature: mean,
            gaussian_curvature: gaussian,
            principal_direction_1: [1.0, 0.0, 0.0],
            principal_direction_2: [0.0, 1.0, 0.0],
        }
    }

    pub fn is_umbilic(&self) -> bool {
        (self.principal_curvature_1 - self.principal_curvature_2).abs() < 1e-6
    }

    pub fn surface_type(&self) -> &'static str {
        let k1 = self.principal_curvature_1;
        let k2 = self.principal_curvature_2;
        if k1.abs() < 1e-10 && k2.abs() < 1e-10 {
            "Plane"
        } else if (k1 * k2).abs() > 1e-10 && k1 * k2 > 0.0 {
            "Sphere-like"
        } else if (k1 * k2).abs() > 1e-10 && k1 * k2 < 0.0 {
            "Saddle"
        } else {
            "Cylinder-like"
        }
    }
}

impl Default for LocalAnalysisSurfaceCurvature {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_analysis_curvature() {
        let c = LocalAnalysisCurvature::new(0.5, 0.1);
        assert!((c.curvature() - 0.5).abs() < 0.01);
        assert!((c.radius_of_curvature() - 2.0).abs() < 0.01);
        assert!(!c.is_inflection_point());
    }

    #[test]
    fn local_analysis_inflection() {
        let c = LocalAnalysisCurvature::new(0.0, 0.0);
        assert!(c.is_inflection_point());
    }

    #[test]
    fn local_analysis_tangent() {
        let t = LocalAnalysisTangent::new([1.0, 0.0, 0.0], 2);
        assert_eq!(t.tangent_direction(), [1.0, 0.0, 0.0]);
        assert!(t.is_continuous(1));
        assert!(t.is_continuous(2));
        assert!(!t.is_continuous(3));
    }

    #[test]
    fn surface_curvature_umbilic() {
        let sc = LocalAnalysisSurfaceCurvature::new(0.5, 0.5);
        assert!(sc.is_umbilic());
    }

    #[test]
    fn surface_curvature_types() {
        let plane = LocalAnalysisSurfaceCurvature::new(0.0, 0.0);
        assert_eq!(plane.surface_type(), "Plane");

        let sphere = LocalAnalysisSurfaceCurvature::new(1.0, 1.0);
        assert_eq!(sphere.surface_type(), "Sphere-like");

        let saddle = LocalAnalysisSurfaceCurvature::new(1.0, -1.0);
        assert_eq!(saddle.surface_type(), "Saddle");
    }
}
