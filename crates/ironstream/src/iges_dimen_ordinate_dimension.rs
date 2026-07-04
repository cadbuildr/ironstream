// FILE: iges_dimen_ordinate_dimension.rs
// occt: IGESDimen_OrdinateDimension

/// Defines OrdinateDimension, Type <218> Form <0>
/// in package IGESDimen
pub struct IgesDimen_OrdinateDimension {
    note: Option<Box<IgesDimen_GeneralNote>>,
    curve: Option<Box<IgesData_IgesEntity>>,
}

impl IgesDimen_OrdinateDimension {
    pub fn new() -> Self {
        IgesDimen_OrdinateDimension {
            note: None,
            curve: None,
        }
    }

    pub fn init(&mut self, a_note: IgesDimen_GeneralNote, a_curve: IgesData_IgesEntity) {
        self.note = Some(Box::new(a_note));
        self.curve = Some(Box::new(a_curve));
    }

    pub fn note(&self) -> Option<&IgesDimen_GeneralNote> {
        self.note.as_ref().map(|n| n.as_ref())
    }

    pub fn curve(&self) -> Option<&IgesData_IgesEntity> {
        self.curve.as_ref().map(|c| c.as_ref())
    }
}

impl Default for IgesDimen_OrdinateDimension {
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
    fn test_ordinate_dimension_creation() {
        let dim = IgesDimen_OrdinateDimension::new();
        assert!(dim.note().is_none());
    }
}
