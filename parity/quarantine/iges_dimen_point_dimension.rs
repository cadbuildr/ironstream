// FILE: iges_dimen_point_dimension.rs
// occt: IGESDimen_PointDimension

/// Defines PointDimension, Type <220> Form <0>
/// in package IGESDimen
pub struct IgesDimen_PointDimension {
    note: Option<Box<IgesDimen_GeneralNote>>,
    point: Option<Box<IgesData_IgesEntity>>,
}

impl IgesDimen_PointDimension {
    pub fn new() -> Self {
        IgesDimen_PointDimension {
            note: None,
            point: None,
        }
    }

    pub fn init(&mut self, a_note: IgesDimen_GeneralNote, a_point: IgesData_IgesEntity) {
        self.note = Some(Box::new(a_note));
        self.point = Some(Box::new(a_point));
    }

    pub fn note(&self) -> Option<&IgesDimen_GeneralNote> {
        self.note.as_ref().map(|n| n.as_ref())
    }

    pub fn point(&self) -> Option<&IgesData_IgesEntity> {
        self.point.as_ref().map(|p| p.as_ref())
    }
}

impl Default for IgesDimen_PointDimension {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct IgesDimen_GeneralNote;

#[derive(Clone)]
pub struct IgesData_IgesEntity;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_dimension_creation() {
        let dim = IgesDimen_PointDimension::new();
        assert!(dim.note().is_none());
    }
}
