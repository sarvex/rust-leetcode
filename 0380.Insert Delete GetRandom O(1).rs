use std::collections::HashMap;

struct RandomizedSet {
    vals: Vec<i32>,
    indices: HashMap<i32, usize>,
}

impl RandomizedSet {
    /// Randomized set supporting O(1) insert, remove, and getRandom.
    ///
    /// # Intuition
    /// A vector provides O(1) random access. A HashMap maps values to indices
    /// for O(1) lookup. On removal, swap the target with the last element.
    ///
    /// # Approach
    /// 1. Insert: append to vector, store index in map.
    /// 2. Remove: swap with last element, update map, pop last.
    /// 3. GetRandom: pick a random index from the vector.
    ///
    /// # Complexity
    /// - Time: O(1) per operation
    /// - Space: O(n)
    fn new() -> Self {
        Self {
            vals: Vec::with_capacity(16),
            indices: HashMap::with_capacity(16),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.indices.contains_key(&val) {
            return false;
        }
        self.indices.insert(val, self.vals.len());
        self.vals.push(val);
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(&idx) = self.indices.get(&val) {
            let last = *self.vals.last().unwrap();
            self.vals[idx] = last;
            self.indices.insert(last, idx);
            self.vals.pop();
            self.indices.remove(&val);
            true
        } else {
            false
        }
    }

    fn get_random(&self) -> i32 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        use std::time::SystemTime;
        let mut hasher = DefaultHasher::new();
        SystemTime::now().hash(&mut hasher);
        let idx = hasher.finish() as usize % self.vals.len();
        self.vals[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_operations() {
        let mut set = RandomizedSet::new();
        assert!(set.insert(1));
        assert!(!set.remove(2));
        assert!(set.insert(2));
        let val = set.get_random();
        assert!(val == 1 || val == 2);
        assert!(set.remove(1));
        assert!(!set.insert(2));
        assert_eq!(set.get_random(), 2);
    }

    #[test]
    fn insert_duplicate() {
        let mut set = RandomizedSet::new();
        assert!(set.insert(1));
        assert!(!set.insert(1));
    }

    #[test]
    fn remove_nonexistent() {
        let mut set = RandomizedSet::new();
        assert!(!set.remove(1));
    }

    #[test]
    fn insert_after_remove() {
        let mut set = RandomizedSet::new();
        assert!(set.insert(1));
        assert!(set.remove(1));
        assert!(set.insert(1));
    }

    #[test]
    fn multiple_elements() {
        let mut set = RandomizedSet::new();
        assert!(set.insert(1));
        assert!(set.insert(2));
        assert!(set.insert(3));
        assert!(set.remove(2));
        let val = set.get_random();
        assert!(val == 1 || val == 3);
    }

    #[test]
    fn insert_remove_sequence() {
        let mut set = RandomizedSet::new();
        for i in 1..=10 {
            assert!(set.insert(i));
        }
        for i in 1..=5 {
            assert!(set.remove(i));
        }
        // Should only contain 6-10
        let val = set.get_random();
        assert!((6..=10).contains(&val));
    }

    #[test]
    fn negative_values() {
        let mut set = RandomizedSet::new();
        assert!(set.insert(-1));
        assert!(set.insert(-2));
        let val = set.get_random();
        assert!(val == -1 || val == -2);
    }
}
