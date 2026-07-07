// FILE: iges_dimen_flag_note.rs
// occt: IGESDimen_FlagNote

/// Defines FlagNote, Type <208> Form <0>
/// in package IGESDimen
pub struct IgesDimen_FlagNote {
    note: Option<Box<IgesDimen_GeneralNote>>,
    x_dimension: Option<Box<IgesData_IgesEntity>>,
    y_dimension: Option<Box<IgesData_IgesEntity>>,
}

impl IgesDimen_FlagNote {
    pub fn new() -> Self {
        IgesDimen_FlagNote {
            note: None,
            x_dimension: None,
            y_dimension: None,
        }
    }

    pub fn init(
        &mut self,
        a_note: IgesDimen_GeneralNote,
        x_dim: IgesData_IgesEntity,
        y_dim: IgesData_IgesEntity,
    ) {
        self.note = Some(Box::new(a_note));
        self.x_dimension = Some(Box::new(x_dim));
        self.y_dimension = Some(Box::new(y_dim));
    }

    pub fn note(&self) -> Option<&IgesDimen_GeneralNote> {
        self.note.as_ref().map(|n| n.as_ref())
    }

    pub fn x_dimension(&self) -> Option<&IgesData_IgesEntity> {
        self.x_dimension.as_ref().map(|x| x.as_ref())
    }

    pub fn y_dimension(&self) -> Option<&IgesData_IgesEntity> {
        self.y_dimension.as_ref().map(|y| y.as_ref())
    }
}

impl Default for IgesDimen_FlagNote {
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
    fn test_flag_note_creation() {
        let note = IgesDimen_FlagNote::new();
        assert!(note.note().is_none());
    }
}
