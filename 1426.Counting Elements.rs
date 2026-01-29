use std::collections::HashSet;

impl Solution {
    /// Hash set lookup to count elements with successor present.
    ///
    /// # Intuition
    /// An element x is counted if x + 1 exists in the array. Building a set
    /// of all values allows O(1) successor checks.
    ///
    /// # Approach
    /// 1. Collect all values into a HashSet
    /// 2. Count elements whose value + 1 is in the set
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the hash set
    pub fn count_elements(arr: Vec<i32>) -> i32 {
        let set: HashSet<i32> = arr.iter().copied().collect();
        arr.iter().filter(|x| set.contains(&(**x + 1))).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_counting() {
        assert_eq!(Solution::count_elements(vec![1, 2, 3]), 2);
    }

    #[test]
    fn no_successors() {
        assert_eq!(Solution::count_elements(vec![1, 1, 3, 3, 5, 5, 7, 7]), 0);
    }

    #[test]
    fn with_duplicates() {
        assert_eq!(Solution::count_elements(vec![1, 3, 2, 3, 5, 0]), 3);
    }

    #[test]
    fn all_have_successor() {
        assert_eq!(Solution::count_elements(vec![1, 1, 2, 2]), 2);
    }
}
