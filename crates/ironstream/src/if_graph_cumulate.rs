// FILE: if_graph_cumulate.rs
// occt: IFGraph_Cumulate

/// Evaluates effect of cumulated sub-parts: overlapping, forgotten entities.
/// Results are kept in a Graph, several questions can be set.
/// Basic iteration gives entities which are part of cumulation.
#[derive(Clone, Debug)]
pub struct IfGraphCumulate {
    the_graph: IfGraphImpl,
}

/// Internal representation of an interface graph
#[derive(Clone, Debug)]
struct IfGraphImpl {
    statuses: Vec<i32>,
    present: Vec<bool>,
    size: usize,
}

impl IfGraphCumulate {
    /// Creates empty Cumulate, ready to work
    pub fn new() -> Self {
        IfGraphCumulate {
            the_graph: IfGraphImpl {
                statuses: vec![],
                present: vec![],
                size: 0,
            },
        }
    }

    /// Set up with a specific graph size
    pub fn with_size(size: usize) -> Self {
        IfGraphCumulate {
            the_graph: IfGraphImpl {
                statuses: vec![0; size],
                present: vec![false; size],
                size,
            },
        }
    }

    /// Adds an entity (by index) and its shared ones to the list
    pub fn get_from_entity(&mut self, entity_num: usize) {
        if entity_num > 0 && entity_num <= self.the_graph.size {
            self.get_from_iter(entity_num);
        }
    }

    /// Adds a list of entities (as indices) as such, that is,
    /// without their shared entities
    pub fn get_from_iter(&mut self, entity_num: usize) {
        if entity_num > 0 && entity_num <= self.the_graph.size {
            let idx = entity_num - 1;
            // OCCT: Interface_Graph::GetFromIter(iter, 1, 1, Standard_True)
            // - not yet present: noted with newstat = 1
            // - already present: status is cumulated (incremented by overlapstat = 1)
            if !self.the_graph.present[idx] {
                self.the_graph.present[idx] = true;
                self.the_graph.statuses[idx] = 1;
            } else {
                self.the_graph.statuses[idx] += 1;
            }
            // OCCT: thegraph.ChangeStatus(1, 2) once the count is done,
            // so a first-time entity gets status 2 (status = 1 + nb of times)
            if self.the_graph.statuses[idx] == 1 {
                self.the_graph.statuses[idx] = 2;
            }
        }
    }

    /// Allows to restart on a new data set
    pub fn reset_data(&mut self) {
        self.the_graph.statuses.iter_mut().for_each(|s| *s = 0);
        self.the_graph.present.iter_mut().for_each(|p| *p = false);
    }

    /// Evaluates the result of cumulation
    pub fn evaluate(&mut self) {
        // OCCT: Reset() then GetFromGraph(thegraph) -- the evaluation is
        // already done in the graph (statuses are cumulated as entities are
        // added); this only rebuilds the iteration content, which this
        // simplified port does not keep separately.
    }

    /// Returns indices of entities which are taken several times (status > 2)
    pub fn overlapped(&self) -> Vec<usize> {
        let mut result = vec![];
        for i in 0..self.the_graph.size {
            if self.the_graph.present[i] && self.the_graph.statuses[i] > 2 {
                result.push(i + 1);
            }
        }
        result
    }

    /// Returns indices of entities which are not taken
    pub fn forgotten(&self) -> Vec<usize> {
        let mut result = vec![];
        for i in 0..self.the_graph.size {
            if !self.the_graph.present[i] {
                result.push(i + 1);
            }
        }
        result
    }

    /// Returns indices of entities taken a given count of times
    /// (0: same as Forgotten, 1: same as no Overlap, default=1)
    pub fn per_count(&self, count: i32) -> Vec<usize> {
        let mut result = vec![];
        let target_status = count + 1;
        for i in 0..self.the_graph.size {
            if self.the_graph.present[i] && self.the_graph.statuses[i] == target_status {
                result.push(i + 1);
            }
        }
        result
    }

    /// Returns number of times an entity has been counted
    /// (0 means forgotten, more than 1 means overlap, 1 is normal)
    pub fn nb_times(&self, entity_num: usize) -> i32 {
        if entity_num == 0 || entity_num > self.the_graph.size {
            return 0;
        }
        let idx = entity_num - 1;
        if !self.the_graph.present[idx] {
            0
        } else {
            self.the_graph.statuses[idx] - 1
        }
    }

    /// Returns the highest number of times recorded for every entity
    /// (0 means empty, 1 means no overlap)
    pub fn highest_nb_times(&self) -> i32 {
        let mut max = 0;
        for i in 0..self.the_graph.size {
            if !self.the_graph.present[i] {
                continue;
            }
            let count = self.the_graph.statuses[i] - 1;
            if count > max {
                max = count;
            }
        }
        max
    }
}

impl Default for IfGraphCumulate {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty() {
        let cum = IfGraphCumulate::new();
        assert_eq!(cum.highest_nb_times(), 0);
    }

    #[test]
    fn test_with_size() {
        let cum = IfGraphCumulate::with_size(10);
        assert_eq!(cum.the_graph.size, 10);
        assert!(cum.the_graph.present.iter().all(|p| !p));
    }

    #[test]
    fn test_get_from_entity_single() {
        let mut cum = IfGraphCumulate::with_size(5);
        cum.get_from_entity(1);
        assert_eq!(cum.nb_times(1), 1);
        assert_eq!(cum.overlapped().len(), 0);
    }

    #[test]
    fn test_get_from_entity_multiple() {
        let mut cum = IfGraphCumulate::with_size(5);
        cum.get_from_entity(1);
        cum.get_from_entity(1); // Add same entity twice
        cum.evaluate();
        // OCCT: each new Get of an already-present entity cumulates
        // (status += 1), and NbTimes = status - 1, so adding the same
        // entity twice gives NbTimes = 2 ("more than 1 means overlap")
        assert_eq!(cum.nb_times(1), 2);
    }

    #[test]
    fn test_forgotten_entities() {
        let mut cum = IfGraphCumulate::with_size(5);
        cum.get_from_entity(1);
        cum.get_from_entity(3);
        let forgotten = cum.forgotten();
        assert_eq!(forgotten.len(), 3);
        assert!(forgotten.contains(&2));
        assert!(forgotten.contains(&4));
        assert!(forgotten.contains(&5));
    }

    #[test]
    fn test_overlapped_entities() {
        let mut cum = IfGraphCumulate::with_size(5);
        cum.get_from_entity(1);
        cum.get_from_iter(1); // Add again
        cum.get_from_iter(1); // Add third time
        cum.evaluate();
        assert!(cum.overlapped().contains(&1));
    }

    #[test]
    fn test_per_count() {
        let mut cum = IfGraphCumulate::with_size(5);
        cum.get_from_entity(1);
        cum.get_from_entity(2);
        cum.get_from_entity(2);
        let count_1 = cum.per_count(1);
        assert!(count_1.contains(&1));
        let count_2 = cum.per_count(2);
        assert!(count_2.contains(&2));
    }

    #[test]
    fn test_highest_nb_times() {
        let mut cum = IfGraphCumulate::with_size(5);
        cum.get_from_entity(1);
        assert_eq!(cum.highest_nb_times(), 1);
        cum.get_from_iter(1);
        cum.get_from_iter(1);
        assert_eq!(cum.highest_nb_times(), 3);
    }

    #[test]
    fn test_reset_data() {
        let mut cum = IfGraphCumulate::with_size(5);
        cum.get_from_entity(1);
        cum.get_from_entity(2);
        cum.reset_data();
        assert_eq!(cum.nb_times(1), 0);
        assert_eq!(cum.nb_times(2), 0);
        assert_eq!(cum.forgotten().len(), 5);
    }
}
