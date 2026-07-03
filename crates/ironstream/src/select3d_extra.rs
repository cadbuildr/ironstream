// FILE: crates/ironstream/src/select3d_extra.rs

// occt: Select3D primitive type
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Select3dSensitiveType {
    Point,
    Segment,
    Triangle,
    Box,
    Circle,
    Cylinder,
    Sphere,
    Polyhedron,
}

// occt: Select3D_SensitivePoint — selectable 3D point
pub struct Select3dSensitivePoint {
    pub point: [f64; 3],
    pub sensitivity: f64,
}

impl Select3dSensitivePoint {
    pub fn new(point: [f64; 3], sensitivity: f64) -> Self {
        Self { point, sensitivity }
    }

    pub fn point(&self) -> [f64; 3] {
        self.point
    }

    pub fn sensitivity(&self) -> f64 {
        self.sensitivity
    }

    pub fn sensitive_type(&self) -> Select3dSensitiveType {
        Select3dSensitiveType::Point
    }
}

// occt: Select3D_SensitiveSegment — selectable line segment
pub struct Select3dSensitiveSegment {
    pub start: [f64; 3],
    pub end: [f64; 3],
    pub sensitivity: f64,
}

impl Select3dSensitiveSegment {
    pub fn new(start: [f64; 3], end: [f64; 3], sensitivity: f64) -> Self {
        Self { start, end, sensitivity }
    }

    pub fn start(&self) -> [f64; 3] {
        self.start
    }

    pub fn end(&self) -> [f64; 3] {
        self.end
    }

    pub fn length(&self) -> f64 {
        let dx = self.end[0] - self.start[0];
        let dy = self.end[1] - self.start[1];
        let dz = self.end[2] - self.start[2];
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    pub fn sensitive_type(&self) -> Select3dSensitiveType {
        Select3dSensitiveType::Segment
    }
}

// occt: Select3D_SensitiveSphere — selectable sphere
pub struct Select3dSensitiveSphere {
    pub center: [f64; 3],
    pub radius: f64,
}

impl Select3dSensitiveSphere {
    pub fn new(center: [f64; 3], radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn center(&self) -> [f64; 3] {
        self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn sensitive_type(&self) -> Select3dSensitiveType {
        Select3dSensitiveType::Sphere
    }
}

// occt: SelectMgr_EntityOwner — associates a selection owner with a selectable entity
pub struct Select3dOwner {
    pub label: String,
    pub priority: i32,
}

impl Select3dOwner {
    pub fn new(label: &str, priority: i32) -> Self {
        Self {
            label: label.to_string(),
            priority,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn priority(&self) -> i32 {
        self.priority
    }

    pub fn set_priority(&mut self, priority: i32) {
        self.priority = priority;
    }
}
