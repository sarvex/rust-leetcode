use std::collections::BTreeMap;

struct CountIntervals {
    intervals: BTreeMap<i32, i32>,
    count: i32,
}

impl CountIntervals {
    /// Count Integers in Intervals
    ///
    /// # Intuition
    /// Maintain non-overlapping intervals in a BTreeMap, merging on insertion.
    /// Track running count to avoid O(n) recalculation.
    ///
    /// # Approach
    /// Use `BTreeMap<i32, i32>` keyed by left endpoint. When adding [left, right]:
    /// 1. Iterate backwards from intervals with l <= new_right
    /// 2. Stop early when r < new_left (no more overlaps possible)
    /// 3. Remove overlapping intervals in-place, updating bounds
    /// 4. Insert merged interval
    ///
    /// # Complexity
    /// - Time: O(k log n) per add where k = overlapping intervals, O(1) for count
    /// - Space: O(n) where n = number of non-overlapping intervals
    fn new() -> Self {
        Self {
            intervals: BTreeMap::new(),
            count: 0,
        }
    }

    fn add(&mut self, left: i32, right: i32) {
        let mut new_left = left;
        let mut new_right = right;

        // Iterate backwards from the largest l <= new_right
        // Remove overlapping intervals in-place for efficiency
        while let Some((&l, &r)) = self.intervals.range(..=new_right).next_back() {
            if r < new_left {
                // No overlap; all earlier intervals also won't overlap
                break;
            }
            // Merge this overlapping interval
            self.count -= r - l + 1;
            new_left = new_left.min(l);
            new_right = new_right.max(r);
            self.intervals.remove(&l);
        }

        // Insert the merged interval
        self.intervals.insert(new_left, new_right);
        self.count += new_right - new_left + 1;
    }

    fn count(&self) -> i32 {
        self.count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut ci = CountIntervals::new();
        ci.add(2, 3);
        ci.add(7, 10);
        assert_eq!(ci.count(), 6);
        ci.add(5, 8);
        assert_eq!(ci.count(), 8);
    }

    #[test]
    fn test_single_interval() {
        let mut ci = CountIntervals::new();
        ci.add(1, 5);
        assert_eq!(ci.count(), 5);
    }

    #[test]
    fn test_overlapping_intervals() {
        let mut ci = CountIntervals::new();
        ci.add(1, 5);
        ci.add(3, 7);
        assert_eq!(ci.count(), 7);
    }

    #[test]
    fn test_contained_interval() {
        let mut ci = CountIntervals::new();
        ci.add(1, 10);
        ci.add(3, 7);
        assert_eq!(ci.count(), 10);
    }

    #[test]
    fn test_adjacent_intervals() {
        let mut ci = CountIntervals::new();
        ci.add(1, 3);
        ci.add(4, 6);
        assert_eq!(ci.count(), 6);
    }

    #[test]
    fn test_disjoint_intervals() {
        let mut ci = CountIntervals::new();
        ci.add(1, 2);
        ci.add(5, 6);
        ci.add(10, 12);
        assert_eq!(ci.count(), 7);
    }

    #[test]
    fn test_merge_multiple_intervals() {
        let mut ci = CountIntervals::new();
        ci.add(1, 2);
        ci.add(5, 6);
        ci.add(9, 10);
        assert_eq!(ci.count(), 6);
        ci.add(2, 9);
        assert_eq!(ci.count(), 10);
    }

    #[test]
    fn test_exact_duplicate() {
        let mut ci = CountIntervals::new();
        ci.add(1, 5);
        ci.add(1, 5);
        assert_eq!(ci.count(), 5);
    }

    #[test]
    fn test_large_values() {
        let mut ci = CountIntervals::new();
        ci.add(1_000_000_000, 1_000_000_000);
        assert_eq!(ci.count(), 1);
        ci.add(1, 1);
        assert_eq!(ci.count(), 2);
    }

    #[test]
    fn test_extending_right() {
        let mut ci = CountIntervals::new();
        ci.add(1, 5);
        ci.add(10, 15);
        ci.add(4, 12);
        assert_eq!(ci.count(), 15);
    }
}
