use std::collections::HashMap;

const MOD: i32 = 1_000_000_007;

impl Solution {
    /// Count valid partitions where block XORs alternate target1, target2, starting with target1.
    ///
    /// # Intuition
    /// A partition is valid iff block XORs are target1, target2, target1, ... Prefix XOR gives
    /// xor(nums[j..i]) = prefix_xor[i] ^ prefix_xor[j]. So we can aggregate by prefix XOR and
    /// use DP with two states (last block target1 vs target2) and hash maps for O(1) transition.
    ///
    /// # Approach
    /// 1. `dp[i][0]` = ways to partition nums[0..i] ending with XOR target1; `dp[i][1]` = ending
    ///    with target2. To extend with a block [j..i] with XOR target1 we need previous state
    ///    target2, so dp[i][0] = sum of dp[j][1] over j with prefix_xor[j] = prefix_xor[i]^target1.
    /// 2. Maintain `cum0`, `cum1`: for each prefix XOR value v, cumulative sums of dp[j][0] and
    ///    dp[j][1]. Then dp[i][0] = cum1[prefix_xor[i]^target1], dp[i][1] = cum0[prefix_xor[i]^target2].
    /// 3. Base: empty prefix counts as one way to be "ready for target1", so cum1[0]=1 initially.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for prefix XOR and maps
    pub fn alternating_xor(nums: Vec<i32>, target1: i32, target2: i32) -> i32 {
        let n = nums.len();
        let mut prefix_xor = vec![0i32; n + 1];
        for (i, &x) in nums.iter().enumerate() {
            prefix_xor[i + 1] = prefix_xor[i] ^ x;
        }

        let mut cum0: HashMap<i32, i32> = HashMap::new();
        let mut cum1: HashMap<i32, i32> = HashMap::new();
        *cum1.entry(0).or_insert(0) = 1;

        let mut full_array_ways = 0i32;
        for i in 1..=n {
            let px = prefix_xor[i];
            let ways_end_target1 = *cum1.get(&(px ^ target1)).unwrap_or(&0) % MOD;
            let ways_end_target2 = *cum0.get(&(px ^ target2)).unwrap_or(&0) % MOD;

            *cum0.entry(px).or_insert(0) =
                (*cum0.get(&px).unwrap_or(&0) + ways_end_target1) % MOD;
            *cum1.entry(px).or_insert(0) =
                (*cum1.get(&px).unwrap_or(&0) + ways_end_target2) % MOD;
            if i == n {
                full_array_ways = (ways_end_target1 + ways_end_target2) % MOD;
            }
        }
        full_array_ways
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::alternating_xor(vec![2, 3, 1, 4], 1, 5),
            1
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::alternating_xor(vec![1, 0, 0], 1, 0), 3);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::alternating_xor(vec![7], 1, 7), 0);
    }

    #[test]
    fn test_single_match_target1() {
        assert_eq!(Solution::alternating_xor(vec![1], 1, 0), 1);
    }

    #[test]
    fn test_single_no_match() {
        assert_eq!(Solution::alternating_xor(vec![0], 1, 2), 0);
    }
}
