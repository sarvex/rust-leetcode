impl Solution {
    /// First element (left-to-right) whose frequency is unique among all values.
    ///
    /// # Intuition
    /// Replace HashMap lookups with direct array indexing for O(1) access with better
    /// cache locality. Since values and frequencies are bounded small integers, flat
    /// vectors serve as perfect hash tables with zero collision overhead.
    ///
    /// # Approach
    /// 1. Build `freq[v]` counting occurrences of each value using a flat vector.
    /// 2. Build `dup[f]` counting how many distinct values share frequency `f`.
    /// 3. Scan `nums` left to right; return the first element where `dup[freq[v]] == 1`.
    ///
    /// # Complexity
    /// - Time: O(n + max_val)
    /// - Space: O(max_val + max_freq)
    pub fn first_unique_freq(nums: Vec<i32>) -> i32 {
        let max_val = nums.iter().copied().max().unwrap() as usize;
        let mut freq = vec![0usize; max_val + 1];
        for n in nums.iter().copied() {
            freq[n as usize] += 1;
        }

        let max_freq = freq.iter().copied().max().unwrap();
        let mut dup = vec![0usize; max_freq + 1];
        for f in freq.iter().copied() {
            dup[f] += 1;
        }

        nums.iter()
            .copied()
            .find(|&n| dup[freq[n as usize]] <= 1)
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::first_unique_freq(vec![20, 10, 30, 30]), 30);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::first_unique_freq(vec![20, 20, 10, 30, 30, 30]),
            20
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::first_unique_freq(vec![10, 10, 20, 20]), -1);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::first_unique_freq(vec![5]), 5);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::first_unique_freq(vec![1, 1, 1, 1]), 1);
    }
}
