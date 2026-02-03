impl Solution {
    /// Count gaps by tracking contiguous value segments.
    ///
    /// # Intuition
    /// Duplicates do not affect imbalance; only distinct values matter. When those values are
    /// sorted, the imbalance equals the number of gaps between consecutive values, which is one
    /// less than the number of contiguous value segments.
    ///
    /// # Approach
    /// Fix a start index and extend the end index. Maintain a `seen` array for values `1..=n` and a
    /// `segments` count for contiguous blocks of seen values. When inserting a new value:
    /// - `segments += 1` for the new block,
    /// - `segments -= 1` if the left neighbor exists,
    /// - `segments -= 1` if the right neighbor exists.
    /// The imbalance of the current subarray is `segments - 1`. Sum it over all starts and ends.
    ///
    /// # Complexity
    /// - Time: O(n^2)
    /// - Space: O(n)
    pub fn sum_imbalance_numbers(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut total: i64 = 0;

        for start in 0..n {
            let mut seen = vec![false; n + 2];
            let mut segments: i32 = 0;

            for end in start..n {
                let value = nums[end] as usize;
                if !seen[value] {
                    segments += 1;
                    if seen[value - 1] {
                        segments -= 1;
                    }
                    if seen[value + 1] {
                        segments -= 1;
                    }
                    seen[value] = true;
                }

                total += (segments - 1) as i64;
            }
        }

        total as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let nums = vec![2, 3, 1, 4];
        assert_eq!(Solution::sum_imbalance_numbers(nums), 3);
    }

    #[test]
    fn example_two() {
        let nums = vec![1, 3, 3, 3, 5];
        assert_eq!(Solution::sum_imbalance_numbers(nums), 8);
    }

    #[test]
    fn all_equal() {
        let nums = vec![4, 4, 4, 4];
        assert_eq!(Solution::sum_imbalance_numbers(nums), 0);
    }

    #[test]
    fn spaced_values() {
        let nums = vec![1, 3, 5];
        assert_eq!(Solution::sum_imbalance_numbers(nums), 4);
    }
}
