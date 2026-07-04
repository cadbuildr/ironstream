// FILE: iges_dimen_radius_dimension.rs
// occt: IGESDimen_RadiusDimension

/// Defines RadiusDimension, Type <222> Form <0>
/// in package IGESDimen
pub struct IgesDimen_RadiusDimension {
    note: Option<Box<IgesDimen_GeneralNote>>,
    leader: Option<Box<IgesDimen_LeaderArrow>>,
    center: (f64, f64),
}

impl IgesDimen_RadiusDimension {
    pub fn new() -> Self {
        IgesDimen_RadiusDimension {
            note: None,
            leader: None,
            center: (0.0, 0.0),
        }
    }

    pub fn init(&mut self, a_note: IgesDimen_GeneralNote, a_leader: IgesDimen_LeaderArrow, center: (f64, f64)) {
        self.note = Some(Box::new(a_note));
        self.leader = Some(Box::new(a_leader));
        self.center = center;
    }

    pub fn note(&self) -> Option<&IgesDimen_GeneralNote> {
        self.note.as_ref().map(|n| n.as_ref())
    }

    pub fn leader(&self) -> Option<&IgesDimen_LeaderArrow> {
        self.leader.as_ref().map(|l| l.as_ref())
    }

    pub fn center(&self) -> (f64, f64) {
        self.center
    }
}

impl Default for IgesDimen_RadiusDimension {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct IgesDimen_GeneralNote;

#[derive(Clone)]
pub struct IgesDimen_LeaderArrow;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radius_dimension_creation() {
        let dim = IgesDimen_RadiusDimension::new();
        assert_eq!(dim.center(), (0.0, 0.0));
    }
}
