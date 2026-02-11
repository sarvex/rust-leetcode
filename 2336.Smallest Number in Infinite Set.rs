use std::collections::BTreeSet;

struct SmallestInfiniteSet {
    present: BTreeSet<i32>,
}

impl SmallestInfiniteSet {
    /// Maintains an infinite set of positive integers supporting pop-smallest and add-back.
    ///
    /// # Intuition
    /// A BTreeSet efficiently tracks which numbers are present, providing O(log n)
    /// insertion, removal, and minimum queries.
    ///
    /// # Approach
    /// Initialize with all integers 1..=1000. Pop returns and removes the smallest.
    /// Add-back re-inserts a removed number.
    ///
    /// # Complexity
    /// - pop_smallest: O(log n)
    /// - add_back: O(log n)
    /// - Space: O(n)
    fn new() -> Self {
        Self {
            present: (1..=1000).collect(),
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        let &smallest = self.present.iter().next().unwrap();
        self.present.remove(&smallest);
        smallest
    }

    fn add_back(&mut self, num: i32) {
        self.present.insert(num);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pop_and_add_back() {
        let mut set = SmallestInfiniteSet::new();
        assert_eq!(set.pop_smallest(), 1);
        assert_eq!(set.pop_smallest(), 2);
        set.add_back(1);
        assert_eq!(set.pop_smallest(), 1);
        assert_eq!(set.pop_smallest(), 3);
    }

    #[test]
    fn test_add_back_already_present() {
        let mut set = SmallestInfiniteSet::new();
        set.add_back(5);
        assert_eq!(set.pop_smallest(), 1);
    }

    #[test]
    fn test_sequential_pops() {
        let mut set = SmallestInfiniteSet::new();
        for expected in 1..=10 {
            assert_eq!(set.pop_smallest(), expected);
        }
    }
}
