use std::collections::HashMap;

/// Tracks element frequencies and allows querying whether any element has a given frequency.
///
/// # Approach
/// Maintain two maps: `cnt` maps numbers to their count, `freq` maps frequency values
/// to how many numbers have that frequency. Both are updated in O(1) per operation.
struct FrequencyTracker {
    cnt: HashMap<i32, i32>,
    freq: HashMap<i32, i32>,
}

impl FrequencyTracker {
    fn new() -> Self {
        Self {
            cnt: HashMap::new(),
            freq: HashMap::new(),
        }
    }

    fn add(&mut self, number: i32) {
        let count = self.cnt.entry(number).or_insert(0);
        *self.freq.entry(*count).or_insert(0) -= 1;
        *count += 1;
        *self.freq.entry(*count).or_insert(0) += 1;
    }

    fn delete_one(&mut self, number: i32) {
        if let Some(count) = self.cnt.get_mut(&number) {
            if *count > 0 {
                *self.freq.entry(*count).or_insert(0) -= 1;
                *count -= 1;
                *self.freq.entry(*count).or_insert(0) += 1;
            }
        }
    }

    fn has_frequency(&self, frequency: i32) -> bool {
        self.freq.get(&frequency).is_some_and(|&f| f > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_query() {
        let mut tracker = FrequencyTracker::new();
        tracker.add(3);
        tracker.add(3);
        assert!(tracker.has_frequency(2));
        assert!(!tracker.has_frequency(3));
    }

    #[test]
    fn test_delete_and_query() {
        let mut tracker = FrequencyTracker::new();
        tracker.add(1);
        tracker.delete_one(1);
        assert!(!tracker.has_frequency(1));
    }

    #[test]
    fn test_delete_nonexistent() {
        let mut tracker = FrequencyTracker::new();
        tracker.delete_one(5);
        assert!(!tracker.has_frequency(0));
    }
}
