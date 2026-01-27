impl Solution {
    /// Counts maximum pairs and remaining elements using frequency counting.
    ///
    /// # Intuition
    /// Each value that appears an even number of times contributes half its count
    /// as pairs. Odd-count values contribute one leftover each.
    ///
    /// # Approach
    /// 1. Count frequency of each value using a fixed-size array
    /// 2. Sum pairs as floor(count / 2) for each value
    /// 3. Leftovers = total elements - 2 * pairs
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1) — fixed 101-entry array
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        let mut freq = [0u32; 101];
        for &v in &nums {
            freq[v as usize] += 1;
        }
        let pairs: u32 = freq.iter().map(|&f| f >> 1).sum();
        vec![pairs as i32, (nums.len() as u32 - pairs * 2) as i32]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::number_of_pairs(vec![1, 3, 2, 1, 3, 2, 2]),
            vec![3, 1]
        );
    }

    #[test]
    fn test_no_pairs() {
        assert_eq!(Solution::number_of_pairs(vec![1, 2, 3, 4, 5]), vec![0, 5]);
    }

    #[test]
    fn test_all_pairs() {
        assert_eq!(Solution::number_of_pairs(vec![0, 0]), vec![1, 0]);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::number_of_pairs(vec![1]), vec![0, 1]);
    }
}
