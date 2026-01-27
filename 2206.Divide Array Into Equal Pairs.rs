use std::collections::HashMap;

impl Solution {
    /// Check if the array can be divided into pairs of equal elements.
    ///
    /// # Intuition
    /// Every element must appear an even number of times to form valid pairs.
    ///
    /// # Approach
    /// Count frequencies with a HashMap and verify all are even.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut count: HashMap<i32, i32> = HashMap::new();
        for x in &nums {
            *count.entry(*x).or_insert(0) += 1;
        }
        count.values().all(|&v| v % 2 == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_pairs() {
        assert!(Solution::divide_array(vec![3, 2, 3, 2, 2, 2]));
    }

    #[test]
    fn invalid_pairs() {
        assert!(!Solution::divide_array(vec![1, 2, 3, 4]));
    }
}
