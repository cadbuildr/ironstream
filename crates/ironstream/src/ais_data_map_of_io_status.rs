// FILE: ais_data_map_of_io_status.rs
// occt: AIS_DataMapOfIOStatus

//! Deprecated NCollection alias: DataMap<int, IO_Status>
//! This is a type alias over std::collections::HashMap.

use std::collections::HashMap;

/// IO status enumeration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IOStatus {
    None = 0,
    Selected = 1,
    Highlighted = 2,
}

/// DataMap from int to IOStatus (HashMap wrapper).
pub type AisDataMapOfIOStatus = HashMap<i32, IOStatus>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_map_insert_get() {
        let mut map: AisDataMapOfIOStatus = HashMap::new();
        map.insert(1, IOStatus::Selected);
        assert_eq!(map.get(&1), Some(&IOStatus::Selected));
    }

    #[test]
    fn test_data_map_remove() {
        let mut map: AisDataMapOfIOStatus = HashMap::new();
        map.insert(1, IOStatus::Highlighted);
        let removed = map.remove(&1);
        assert_eq!(removed, Some(IOStatus::Highlighted));
    }
}
