// FILE: b_rep_fill_list_of_offset_wire.rs
// occt: BRepFill_ListOfOffsetWire

//! Deprecated type alias for backward compatibility.
//! A list of offset wire representations.

/// An offset wire representation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OffsetWire {
    pub id: usize,
    pub offset: i32,
}

impl OffsetWire {
    pub fn new(id: usize, offset: i32) -> Self {
        Self { id, offset }
    }
}

/// A list of offset wires.
pub type BRepFillListOfOffsetWire = Vec<OffsetWire>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offset_wire_creation() {
        let wire = OffsetWire::new(1, 5);
        assert_eq!(wire.id, 1);
        assert_eq!(wire.offset, 5);
    }

    #[test]
    fn test_list_creation() {
        let mut list: BRepFillListOfOffsetWire = Vec::new();
        list.push(OffsetWire::new(1, 10));
        list.push(OffsetWire::new(2, 20));
        list.push(OffsetWire::new(3, 30));

        assert_eq!(list.len(), 3);
        assert_eq!(list[0].offset, 10);
        assert_eq!(list[2].id, 3);
    }

    #[test]
    fn test_list_iteration() {
        let mut list: BRepFillListOfOffsetWire = Vec::new();
        for i in 0..5 {
            list.push(OffsetWire::new(i, (i * 10) as i32));
        }

        let sum: i32 = list.iter().map(|w| w.offset).sum();
        assert_eq!(sum, 0 + 10 + 20 + 30 + 40);
    }
}
