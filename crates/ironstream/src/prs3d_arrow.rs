// FILE: rust/ironstream/crates/ironstream/src/prs3d_arrow.rs

use std::f64::consts::PI;

// occt: Prs3d_DimensionArrowOrientation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prs3dArrowType {
    External,
    Internal,
    UserDefined,
}

// occt: Prs3d_Arrow
pub struct Prs3dArrow {
    location: [f64; 3],
    direction: [f64; 3],
    length: f64,
    angle: f64,
    arrow_type: Prs3dArrowType,
}

impl Prs3dArrow {
    /// Create a new arrow at `location` pointing in `direction` with the given `length`.
    /// `direction` should be a unit vector; if it is zero the behaviour of wing methods
    /// is unspecified (same as OCCT).
    /// Default angle is PI/12 (15°); default type is External.
    pub fn new(location: [f64; 3], direction: [f64; 3], length: f64) -> Self {
        Self {
            location,
            direction,
            length,
            angle: PI / 12.0,
            arrow_type: Prs3dArrowType::External,
        }
    }

    pub fn location(&self) -> [f64; 3] {
        self.location
    }

    pub fn direction(&self) -> [f64; 3] {
        self.direction
    }

    pub fn length(&self) -> f64 {
        self.length
    }

    pub fn angle(&self) -> f64 {
        self.angle
    }

    pub fn set_angle(&mut self, angle: f64) {
        self.angle = angle;
    }

    pub fn arrow_type(&self) -> Prs3dArrowType {
        self.arrow_type
    }

    pub fn set_arrow_type(&mut self, arrow_type: Prs3dArrowType) {
        self.arrow_type = arrow_type;
    }

    /// The tip of the arrow: location + direction * length.
    pub fn tip(&self) -> [f64; 3] {
        let [lx, ly, lz] = self.location;
        let [dx, dy, dz] = self.direction;
        let l = self.length;
        [lx + dx * l, ly + dy * l, lz + dz * l]
    }

    /// Compute a vector perpendicular to `direction` in a stable way.
    /// We pick the axis (X, Y, or Z) least parallel to `direction`, cross it
    /// with `direction`, then normalise — identical to the approach used in
    /// OCCT's Prs3d_Arrow::Draw helper.
    fn perp(&self) -> [f64; 3] {
        let [dx, dy, dz] = self.direction;
        // Choose the basis axis whose component in `direction` is smallest.
        let (ax, ay, az) = if dx.abs() <= dy.abs() && dx.abs() <= dz.abs() {
            (1.0_f64, 0.0_f64, 0.0_f64)
        } else if dy.abs() <= dz.abs() {
            (0.0, 1.0, 0.0)
        } else {
            (0.0, 0.0, 1.0)
        };
        // cross(axis, direction)
        let px = ay * dz - az * dy;
        let py = az * dx - ax * dz;
        let pz = ax * dy - ay * dx;
        let mag = (px * px + py * py + pz * pz).sqrt();
        [px / mag, py / mag, pz / mag]
    }

    /// Left wing point: the tip displaced by `length * tan(angle)` along the
    /// perpendicular and `length` back along the negative direction.
    pub fn left_wing(&self) -> [f64; 3] {
        wing(self.tip(), self.direction, self.length, self.angle, self.perp(), 1.0)
    }

    /// Right wing point: mirror of the left wing.
    pub fn right_wing(&self) -> [f64; 3] {
        wing(self.tip(), self.direction, self.length, self.angle, self.perp(), -1.0)
    }
}

/// Compute one wing of the arrowhead.
///
/// The wing lies at:
///   tip  −  direction * wing_back  ±  perp * wing_spread
///
/// where:
///   wing_back   = length
///   wing_spread = length * tan(angle)
///
/// `side` is +1.0 for left, −1.0 for right.
fn wing(
    tip: [f64; 3],
    direction: [f64; 3],
    length: f64,
    angle: f64,
    perp: [f64; 3],
    side: f64,
) -> [f64; 3] {
    let spread = length * angle.tan();
    let [tx, ty, tz] = tip;
    let [dx, dy, dz] = direction;
    let [px, py, pz] = perp;
    [
        tx - dx * length + px * spread * side,
        ty - dy * length + py * spread * side,
        tz - dz * length + pz * spread * side,
    ]
}
