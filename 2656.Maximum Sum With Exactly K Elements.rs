impl Solution {
    /// Maximize sum by picking the max element k times with increments.
    ///
    /// # Intuition
    /// Always pick the current maximum and increment it. The sum forms an
    /// arithmetic series starting from the array maximum.
    ///
    /// # Approach
    /// Sum = k * max + k*(k-1)/2 using the arithmetic series formula.
    ///
    /// # Complexity
    /// - Time: O(n) to find max
    /// - Space: O(1)
    pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mx = *nums.iter().max().unwrap_or(&0);
        k * mx + k * (k - 1) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::maximize_sum(vec![1, 2, 3, 4, 5], 3), 18);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::maximize_sum(vec![5], 2), 11);
    }

    #[test]
    fn test_k_one() {
        assert_eq!(Solution::maximize_sum(vec![3, 7, 1], 1), 7);
    }
}
