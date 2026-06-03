impl Solution {
    /// Sort-by-height DP with early-exit neighbour scans.
    ///
    /// # Intuition
    /// You can only jump from a taller bar to a shorter one. Processing indices
    /// in ascending order of `arr[i]` guarantees every reachable destination
    /// `j` (`arr[j] < arr[i]`) is already resolved when we reach `i`, turning
    /// the problem into a simple bottom-up DP with no cycles.
    ///
    /// # Approach
    /// 1. Sort indices by `arr[i]` ascending.
    /// 2. For each index `i` in that order scan left (`i-d..i` reversed) and
    ///    right (`i+1..=i+d`), stopping as soon as a bar ≥ `arr[i]` is hit —
    ///    it blocks all further jumps in that direction.
    /// 3. `dp[i] = 1 + max(dp[j])` over all reachable `j`.
    /// 4. Return `dp.iter().max()`.
    ///
    /// # Complexity
    /// - Time: O(n log n) — sort; scans are O(n) amortised (each blocker
    ///   terminates the scan early; in the worst case O(n × d) but d ≤ n)
    /// - Space: O(n)
    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let n = arr.len();
        let d = d as usize;
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_unstable_by_key(|&i| arr[i]);
        let mut dp = vec![1i32; n];

        for i in order {
            // Left: scan from i-1 down to i-d; stop at first bar >= arr[i].
            for j in (i.saturating_sub(d)..i).rev() {
                if arr[j] >= arr[i] {
                    break;
                }
                dp[i] = dp[i].max(dp[j] + 1);
            }
            // Right: scan from i+1 up to i+d; stop at first bar >= arr[i].
            for j in i + 1..=(i + d).min(n - 1) {
                if arr[j] >= arr[i] {
                    break;
                }
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }

        *dp.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        // 10 -> 8 -> 6 -> 7 visits 4 indices
        assert_eq!(
            Solution::max_jumps(vec![6, 4, 14, 6, 8, 13, 9, 7, 10, 6, 12], 2),
            4
        );
    }

    #[test]
    fn all_equal_no_jumps() {
        // No jump possible when all values are equal
        assert_eq!(Solution::max_jumps(vec![3, 3, 3, 3, 3], 3), 1);
    }

    #[test]
    fn strictly_decreasing_full_chain() {
        // Can chain all 7 indices with d = 1
        assert_eq!(Solution::max_jumps(vec![7, 6, 5, 4, 3, 2, 1], 1), 7);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::max_jumps(vec![42], 1), 1);
    }

    #[test]
    fn d_larger_than_array() {
        // d covers the whole array; strictly decreasing so full chain reachable
        assert_eq!(Solution::max_jumps(vec![5, 4, 3, 2, 1], 10), 5);
    }

    #[test]
    fn peak_can_reach_all() {
        // Peak at index 2 (val 10) reaches 0-1 left and 3-4 right.
        // Best chain: 2 -> 1 -> 0 or 2 -> 3 -> 4, length 3.
        assert_eq!(Solution::max_jumps(vec![1, 2, 10, 2, 1], 2), 3);
    }

    #[test]
    fn two_elements_reachable() {
        assert_eq!(Solution::max_jumps(vec![1, 2], 1), 2);
    }

    #[test]
    fn chain_through_intermediate() {
        // arr = [3, 1, 5, 1, 3], d = 2
        // dp[1]=dp[3]=1. dp[0]=2 (reaches index 1). dp[4]=2 (reaches index 3).
        // dp[2]=3 (reaches index 0 with dp=2).
        assert_eq!(Solution::max_jumps(vec![3, 1, 5, 1, 3], 2), 3);
    }

    #[test]
    fn multiple_hops() {
        // arr = [1, 3, 1, 3, 1], d = 2
        // dp[0]=dp[2]=dp[4]=1. dp[1]=2. dp[3]=2. Answer = 2.
        assert_eq!(Solution::max_jumps(vec![1, 3, 1, 3, 1], 2), 2);
    }
}
