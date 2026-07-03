// FILE: osd_from_where.rs
// occt: OSD_FromWhere

/// Specifies file seek reference point.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FromWhere {
    /// From beginning of file
    FromBeginning,
    /// From current position
    FromHere,
    /// From end of file
    FromEnd,
}

impl FromWhere {
    /// Convert to std::io::SeekFrom.
    pub fn to_seek_from(&self, offset: i64) -> std::io::SeekFrom {
        match self {
            FromWhere::FromBeginning => std::io::SeekFrom::Start(offset as u64),
            FromWhere::FromHere => std::io::SeekFrom::Current(offset),
            FromWhere::FromEnd => std::io::SeekFrom::End(offset),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_where_variants() {
        assert_eq!(FromWhere::FromBeginning, FromWhere::FromBeginning);
        assert_ne!(FromWhere::FromBeginning, FromWhere::FromEnd);
    }

    #[test]
    fn test_to_seek_from() {
        let seek = FromWhere::FromBeginning.to_seek_from(10);
        match seek {
            std::io::SeekFrom::Start(pos) => assert_eq!(pos, 10),
            _ => panic!("Wrong seek type"),
        }
    }
}
