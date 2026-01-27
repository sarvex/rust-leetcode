use std::collections::HashSet;

impl Solution {
    /// Finds the longest consecutive element sequence using a HashSet for O(1) lookups.
    ///
    /// # Intuition
    /// Only start counting from sequence beginnings — elements with no predecessor
    /// in the set. This ensures each element is visited at most twice.
    ///
    /// # Approach
    /// 1. Insert all numbers into a HashSet.
    /// 2. For each number, if `num - 1` is not in the set, it starts a sequence.
    /// 3. Count consecutive elements from that start.
    /// 4. Track the maximum length.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the HashSet
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.iter().copied().collect();
        set.iter()
            .filter(|&&num| !set.contains(&(num - 1)))
            .map(|&start| {
                (0..)
                    .take_while(|&len| set.contains(&(start + len)))
                    .count() as i32
            })
            .max()
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_case() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }

    #[test]
    fn with_duplicates() {
        assert_eq!(
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );
    }

    #[test]
    fn empty_input() {
        assert_eq!(Solution::longest_consecutive(vec![]), 0);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::longest_consecutive(vec![1]), 1);
    }
}
