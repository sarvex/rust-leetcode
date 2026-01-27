impl Solution {
    /// Finds the missing number in [0, n] using XOR.
    ///
    /// # Intuition
    /// XOR of all indices [0..n] with all array values cancels every pair,
    /// leaving only the missing number.
    ///
    /// # Approach
    /// Initialize result to n, then XOR each index and value pair.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        nums.iter()
            .enumerate()
            .fold(nums.len() as i32, |acc, (i, &v)| acc ^ (i as i32) ^ v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_two() {
        assert_eq!(Solution::missing_number(vec![3, 0, 1]), 2);
    }

    #[test]
    fn missing_eight() {
        assert_eq!(Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }

    #[test]
    fn missing_zero() {
        assert_eq!(Solution::missing_number(vec![1]), 0);
    }
}
