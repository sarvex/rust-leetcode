use std::collections::HashMap;

impl Solution {
    /// Finds the minimum number of groups for a valid assignment.
    ///
    /// # Intuition
    /// Count frequencies of each number. We can partition each frequency count
    /// into groups of size `k` or `k+1`. The optimal `k` is the largest value
    /// where every frequency can be split into such groups.
    ///
    /// # Approach
    /// 1. Count element frequencies using a `HashMap`.
    /// 2. Find the minimum frequency as the upper bound for group size `k`.
    /// 3. Try each `k` from the minimum down to 1.
    /// 4. For each `k`, verify all frequencies can split into groups of size
    ///    `k` or `k+1` by checking `v / k >= v % k`.
    /// 5. Sum the group counts `(v + k) / (k + 1)` for valid assignments.
    ///
    /// # Complexity
    /// - Time: O(n + min_freq * d) where d is the number of distinct elements
    /// - Space: O(d) for the frequency map
    pub fn min_groups_for_valid_assignment(nums: Vec<i32>) -> i32 {
        let mut freq: HashMap<i32, i32> = HashMap::new();
        nums.iter().for_each(|&x| {
            *freq.entry(x).or_insert(0) += 1;
        });

        let min_freq = freq.values().copied().min().unwrap_or(0);

        for k in (1..=min_freq).rev() {
            let mut ans = 0;
            let mut valid = true;

            for &v in freq.values() {
                if v / k < v % k {
                    valid = false;
                    break;
                }
                ans += (v + k) / (k + 1);
            }

            if valid && ans > 0 {
                return ans;
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_assignment() {
        assert_eq!(
            Solution::min_groups_for_valid_assignment(vec![3, 2, 3, 2, 3]),
            2
        );
    }

    #[test]
    fn test_uniform_elements() {
        assert_eq!(
            Solution::min_groups_for_valid_assignment(vec![1, 1, 1, 1]),
            1
        );
    }
}
