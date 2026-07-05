// FILE: ais_list_of_interactive.rs
// occt: AIS_ListOfInteractive

//! Deprecated NCollection alias: List<Interactive>

/// Interactive object (stub).
#[derive(Clone, Debug)]
pub struct Interactive {
    pub id: u32,
}

/// List of interactive objects.
pub type AisListOfInteractive = Vec<Interactive>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_push_pop() {
        let mut list: AisListOfInteractive = Vec::new();
        list.push(Interactive { id: 1 });
        list.push(Interactive { id: 2 });
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop().map(|i| i.id), Some(2));
    }
}
