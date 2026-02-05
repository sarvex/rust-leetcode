const MOD: i32 = 1_000_000_007;

impl Solution {
    /// Count valid partitions where block XORs alternate target1, target2, starting with target1.
    ///
    /// # Intuition
    /// Valid cut positions have prefix XOR in a 4-cycle: 0 → target1 → (target1^target2) →
    /// target2 → 0. So we only need four DP states instead of indexing by all possible XOR values.
    ///
    /// # Approach
    /// 1. States: cycle of prefix XORs at cuts: 0 → target1 → (target1^target2) → target2 → 0.
    /// 2. ways_for_state[k] = number of ways to partition the current prefix with that state.
    /// 3. When running_prefix_xor equals the next cycle value, we can cut and transition.
    /// 4. Single pass with rolling prefix XOR; no extra allocation.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn alternating_xor(nums: Vec<i32>, target1: i32, target2: i32) -> i32 {
        let length = nums.len();
        let cycle_prefix_xors: [i32; 4] = [0, target1, target1 ^ target2, target2];

        let mut ways_for_state = [0i32; 4];
        ways_for_state[0] = 1;
        let mut running_prefix_xor = 0i32;
        for index in 0..length - 1 {
            running_prefix_xor ^= nums[index];
            let mut transition_ways = [0i32; 4];
            if running_prefix_xor == cycle_prefix_xors[1] {
                transition_ways[1] = ways_for_state[0];
            }
            if running_prefix_xor == cycle_prefix_xors[2] {
                transition_ways[2] = ways_for_state[1];
            }
            if running_prefix_xor == cycle_prefix_xors[3] {
                transition_ways[3] = ways_for_state[2];
            }
            if running_prefix_xor == cycle_prefix_xors[0] {
                transition_ways[0] = ways_for_state[3];
            }
            ways_for_state
                .iter_mut()
                .zip(transition_ways.iter())
                .for_each(|(state, &trans)| *state = (*state + trans) % MOD);
        }
        running_prefix_xor ^= nums[length - 1];
        let mut result = 0i32;
        if running_prefix_xor == cycle_prefix_xors[1] {
            result = (result + ways_for_state[0]) % MOD;
        }
        if running_prefix_xor == cycle_prefix_xors[2] {
            result = (result + ways_for_state[1]) % MOD;
        }
        if running_prefix_xor == cycle_prefix_xors[3] {
            result = (result + ways_for_state[2]) % MOD;
        }
        if running_prefix_xor == cycle_prefix_xors[0] {
            result = (result + ways_for_state[3]) % MOD;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::alternating_xor(vec![2, 3, 1, 4], 1, 5), 1);
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
