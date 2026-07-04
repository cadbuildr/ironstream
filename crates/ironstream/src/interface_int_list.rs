// FILE: interface_int_list.rs
// occt: Interface_IntList

/// This class detains the data which describe a Graph.
/// Efficient implementation using two arrays of integers
pub struct InterfaceIntList {
    thenbe: usize,        // number of entities
    thenbr: usize,        // number of references
    thenum: usize,        // current entity number
    thecount: usize,      // count for current entity
    therank: usize,       // rank for current entity
    theents: Vec<i32>,    // entity headers array
    therefs: Vec<i32>,    // references array
}

impl InterfaceIntList {
    /// Creates empty IntList
    pub fn new() -> Self {
        InterfaceIntList {
            thenbe: 0,
            thenbr: 0,
            thenum: 0,
            thecount: 0,
            therank: 0,
            theents: Vec::new(),
            therefs: Vec::new(),
        }
    }

    /// Creates an IntList for <nbe> entities
    pub fn with_capacity(nbe: usize) -> Self {
        InterfaceIntList {
            thenbe: nbe,
            thenbr: 0,
            thenum: 0,
            thecount: 0,
            therank: 0,
            theents: vec![0; nbe],
            therefs: Vec::new(),
        }
    }

    /// Creates an IntList from another one
    pub fn from_other(other: &InterfaceIntList, copied: bool) -> Self {
        if copied {
            InterfaceIntList {
                thenbe: other.thenbe,
                thenbr: other.thenbr,
                thenum: other.thenum,
                thecount: other.thecount,
                therank: other.therank,
                theents: other.theents.clone(),
                therefs: other.therefs.clone(),
            }
        } else {
            InterfaceIntList {
                thenbe: other.thenbe,
                thenbr: other.thenbr,
                thenum: other.thenum,
                thecount: other.thecount,
                therank: other.therank,
                theents: Vec::new(),
                therefs: Vec::new(),
            }
        }
    }

    /// Initialize IntList by number of entities
    pub fn initialize(&mut self, nbe: usize) {
        self.thenbe = nbe;
        self.theents = vec![0; nbe];
        self.therefs.clear();
        self.thenbr = 0;
    }

    /// Returns count of stored references
    pub fn nb_references(&self) -> usize {
        self.thenbr
    }

    /// Returns count of entities
    pub fn nb_entities(&self) -> usize {
        self.thenbe
    }

    /// Changes the count of entities
    pub fn set_nb_entities(&mut self, nbe: usize) {
        if nbe > self.thenbe {
            self.theents.resize(nbe, 0);
            self.thenbe = nbe;
        }
    }

    /// Sets an entity number as current
    pub fn set_number(&mut self, number: usize) {
        if number < self.thenbe {
            self.thenum = number;
            self.thecount = 0;
            self.therank = 0;
        }
    }

    /// Returns the current entity number
    pub fn number(&self) -> usize {
        self.thenum
    }

    /// Returns an IntList set to a specified entity Number
    pub fn list(&self, number: usize, copied: bool) -> InterfaceIntList {
        let mut result = if copied {
            InterfaceIntList::from_other(self, true)
        } else {
            InterfaceIntList::from_other(self, false)
        };
        result.set_number(number);
        result
    }

    /// Sets current entity list to be redefined or not
    pub fn set_redefined(&mut self, _mode: bool) {
        // TODO: Implement redefinition flag
    }

    /// Makes a reservation for count references
    pub fn reservate(&mut self, count: usize) {
        if count > 1 && count > self.therefs.len() - self.thenbr {
            self.therefs.reserve(count);
        }
    }

    /// Adds a reference to the current entity number
    pub fn add(&mut self, ref_val: i32) {
        if ref_val != 0 {
            self.therefs.push(ref_val);
            self.thenbr += 1;
            self.thecount += 1;
        }
    }

    /// Returns the count of refs for current entity number
    pub fn length(&self) -> usize {
        self.thecount
    }

    /// Returns True if the list is "redefined"
    pub fn is_redefined(&self, _num: Option<usize>) -> bool {
        false
    }

    /// Returns a reference number in the list, according to rank
    pub fn value(&self, rank: usize) -> Option<i32> {
        if rank < self.therefs.len() {
            Some(self.therefs[rank])
        } else {
            None
        }
    }

    /// Removes an item in the list, given its rank
    pub fn remove(&mut self, _rank: usize) -> bool {
        false
    }

    /// Clears all data
    pub fn clear(&mut self) {
        self.theents.clear();
        self.therefs.clear();
        self.thenbe = 0;
        self.thenbr = 0;
        self.thecount = 0;
    }

    /// Resizes lists to exact sizes
    pub fn adjust_size(&mut self, _margin: usize) {
        self.theents.shrink_to_fit();
        self.therefs.shrink_to_fit();
    }
}

impl Default for InterfaceIntList {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty() {
        let list = InterfaceIntList::new();
        assert_eq!(list.nb_entities(), 0);
        assert_eq!(list.nb_references(), 0);
    }

    #[test]
    fn test_with_capacity() {
        let list = InterfaceIntList::with_capacity(10);
        assert_eq!(list.nb_entities(), 10);
        assert_eq!(list.nb_references(), 0);
    }

    #[test]
    fn test_set_number() {
        let mut list = InterfaceIntList::with_capacity(5);
        list.set_number(2);
        assert_eq!(list.number(), 2);
    }

    #[test]
    fn test_add_reference() {
        let mut list = InterfaceIntList::with_capacity(5);
        list.set_number(0);
        list.add(42);
        assert_eq!(list.nb_references(), 1);
        assert_eq!(list.length(), 1);
    }

    #[test]
    fn test_clear() {
        let mut list = InterfaceIntList::with_capacity(5);
        list.add(1);
        list.clear();
        assert_eq!(list.nb_entities(), 0);
        assert_eq!(list.nb_references(), 0);
    }

    #[test]
    fn test_from_other() {
        let list1 = InterfaceIntList::with_capacity(3);
        let list2 = InterfaceIntList::from_other(&list1, true);
        assert_eq!(list2.nb_entities(), 3);
    }
}
