// FILE: gce_make_parab2d.rs
// occt: gce_MakeParab2d

use crate::gce_root::{GceRoot, GceErrorType};
use crate::gp_parab2d::Parab2d;
use crate::gp2d::{Pnt2d, Ax2d};
use crate::geom2d_circle::Ax22d2;

const RESOLUTION: f64 = 1e-10;

/// Construction algorithm for gp_Parab2d (2D parabola).
/// Implements construction algorithms for 2D parabolas:
/// - from symmetry axis and focal length;
/// - from full axis system and focal length;
/// - from directrix and focus;
/// - from focus and vertex.
pub struct GceMakeParab2d {
    root: GceRoot,
    parab: Option<Parab2d>,
}

impl GceMakeParab2d {
    /// Creates a parabola from symmetry axis and focal length.
    /// The symmetry axis defines the origin (vertex) and X direction.
    /// `sense` indicates orientation of parametrization (true = direct).
    pub fn new_from_mirror_axis(mirror_axis: Ax2d, focal: f64, sense: bool) -> Self {
        if focal < 0.0 {
            GceMakeParab2d {
                root: GceRoot::with_error(GceErrorType::NullFocusLength),
                parab: None,
            }
        } else {
            let parab = Parab2d::new_with_sense(mirror_axis, focal, sense);
            GceMakeParab2d {
                root: GceRoot::new(),
                parab: Some(parab),
            }
        }
    }

    /// Creates a parabola from full 2D local coordinate system and focal length.
    pub fn new_from_ax22d(ax: Ax22d2, focal: f64) -> Self {
        if focal < 0.0 {
            GceMakeParab2d {
                root: GceRoot::with_error(GceErrorType::NullFocusLength),
                parab: None,
            }
        } else {
            let parab = Parab2d::from_axis(ax, focal);
            GceMakeParab2d {
                root: GceRoot::new(),
                parab: Some(parab),
            }
        }
    }

    /// Creates a parabola from directrix and focus.
    /// The directrix is given as an axis (a line in 2D).
    /// The focus is a point in 2D space.
    pub fn new_from_directrix_focus(directrix: Ax2d, focus: Pnt2d, sense: bool) -> Self {
        // Construct parab from directrix and focus.
        // The parabola is defined by: distance(P, focus) = distance(P, directrix)
        // for any point P on the parabola.

        // Project focus onto the directrix line
        let d_origin = directrix.location;
        let d_dir = directrix.direction;

        let v = focus - d_origin;
        let t = v.dot(d_dir);
        let proj = d_origin + d_dir * t;

        // Vertex is the midpoint between focus and its projection on directrix
        let vertex = (focus + proj) * 0.5;

        // Focal length is half the distance from vertex to focus
        let focal = focus.distance(vertex);

        // X-axis direction: from vertex toward focus
        let x_dir = if focal > RESOLUTION {
            ((focus - vertex).normalized())
        } else {
            d_dir
        };

        // Y-axis: parallel to directrix
        let y_dir = d_dir;

        let ax2d = Ax2d::new(vertex, x_dir);
        let ax22d = Ax22d2::from_x_axis(vertex, x_dir, sense);

        let parab = Parab2d::from_axis(ax22d, focal);
        GceMakeParab2d {
            root: GceRoot::new(),
            parab: Some(parab),
        }
    }

    /// Creates a parabola from focus and vertex.
    /// S1 is the focus point, center is the vertex point.
    pub fn new_from_focus_vertex(focus: Pnt2d, center: Pnt2d, sense: bool) -> Self {
        let focal_dist = focus.distance(center);
        if focal_dist < RESOLUTION {
            GceMakeParab2d {
                root: GceRoot::with_error(GceErrorType::NullAxis),
                parab: None,
            }
        } else {
            // X-axis: from center (vertex) toward focus
            let x_dir = (focus - center).normalized();
            let mirror_axis = Ax2d::new(center, x_dir);
            let parab = Parab2d::new_with_sense(mirror_axis, focal_dist, sense);
            GceMakeParab2d {
                root: GceRoot::new(),
                parab: Some(parab),
            }
        }
    }

    /// Returns the constructed parabola.
    /// Panics if construction failed (status != Done).
    pub fn value(&self) -> Parab2d {
        if !self.root.is_done() {
            panic!(
                "gce_MakeParab2d::Value() - construction failed with status: {:?}",
                self.root.status()
            );
        }
        self.parab.expect("parab must be Some if construction succeeded")
    }

    /// Alias for value() returning a copy.
    pub fn operator(&self) -> Parab2d {
        self.value()
    }

    /// Returns true if the construction is successful.
    pub fn is_done(&self) -> bool {
        self.root.is_done()
    }

    /// Returns the construction status.
    pub fn status(&self) -> GceErrorType {
        self.root.status()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_parab2d_from_mirror_axis() {
        let axis = Ax2d::new(Pnt2d::new(0.0, 0.0), Pnt2d::new(1.0, 0.0));
        let focal = 1.0;
        let maker = GceMakeParab2d::new_from_mirror_axis(axis, focal, true);
        assert!(maker.is_done());
        let _parab = maker.value();
    }

    #[test]
    fn test_make_parab2d_from_mirror_axis_negative_focal() {
        let axis = Ax2d::new(Pnt2d::new(0.0, 0.0), Pnt2d::new(1.0, 0.0));
        let maker = GceMakeParab2d::new_from_mirror_axis(axis, -1.0, true);
        assert!(!maker.is_done());
        assert_eq!(maker.status(), GceErrorType::NullFocusLength);
    }

    #[test]
    fn test_make_parab2d_from_ax22d() {
        let ax22d = Ax22d2::from_x_axis(
            Pnt2d::new(0.0, 0.0),
            Pnt2d::new(1.0, 0.0),
            true,
        );
        let maker = GceMakeParab2d::new_from_ax22d(ax22d, 2.0);
        assert!(maker.is_done());
        let _parab = maker.value();
    }

    #[test]
    fn test_make_parab2d_from_directrix_focus() {
        let directrix = Ax2d::new(Pnt2d::new(0.0, 0.0), Pnt2d::new(0.0, 1.0));
        let focus = Pnt2d::new(2.0, 0.0);
        let maker = GceMakeParab2d::new_from_directrix_focus(directrix, focus, true);
        assert!(maker.is_done());
        let _parab = maker.value();
    }

    #[test]
    fn test_make_parab2d_from_focus_vertex() {
        let focus = Pnt2d::new(2.0, 0.0);
        let vertex = Pnt2d::new(0.0, 0.0);
        let maker = GceMakeParab2d::new_from_focus_vertex(focus, vertex, true);
        assert!(maker.is_done());
        let _parab = maker.value();
    }

    #[test]
    fn test_make_parab2d_from_focus_vertex_coincident() {
        let focus = Pnt2d::new(0.0, 0.0);
        let vertex = Pnt2d::new(0.0, 0.0);
        let maker = GceMakeParab2d::new_from_focus_vertex(focus, vertex, true);
        assert!(!maker.is_done());
        assert_eq!(maker.status(), GceErrorType::NullAxis);
    }

    #[test]
    #[should_panic(expected = "construction failed")]
    fn test_make_parab2d_value_on_failed_construction() {
        let axis = Ax2d::new(Pnt2d::new(0.0, 0.0), Pnt2d::new(1.0, 0.0));
        let maker = GceMakeParab2d::new_from_mirror_axis(axis, -1.0, true);
        let _ = maker.value(); // Should panic
    }
}
