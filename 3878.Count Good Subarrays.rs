impl Solution {
    /// Counts subarrays whose bitwise OR equals some element within them using
    /// monotone stacks to find each element's domination range.
    ///
    /// # Intuition
    /// A subarray is "good" when its OR matches an element inside it. Since OR ≥
    /// every element, the matching element must be a bitwise superset of all others.
    /// For each index `i`, count subarrays where `nums[i]` is that dominating element:
    /// every other element in the subarray must be a bitwise subset of `nums[i]`.
    ///
    /// # Approach
    /// 1. **Left boundary**: Scan left-to-right with a monotone stack. Pop indices
    ///    whose values are bitwise subsets of `nums[i]` (`val | x == x`), including
    ///    equal values. `left[i]` = top of remaining stack sentinel, or −1.
    /// 2. **Right boundary**: Scan right-to-left with a monotone stack. Pop indices
    ///    whose values are strict subsets of `nums[i]` (`val != x && val | x == x`),
    ///    excluding equals. Accumulate contributions inline.
    ///
    /// The non-strict left / strict right asymmetry prevents double-counting when
    /// adjacent elements are equal.
    ///
    /// # Complexity
    /// - Time: O(n) — each index is pushed and popped at most once per stack.
    /// - Space: O(n)
    pub fn count_good_subarrays(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let (mut left, mut stk) = (vec![0_i32; n], vec![-1_i32]);

        for (i, &x) in nums.iter().enumerate() {
            while stk.len() > 1 && nums[*stk.last().unwrap() as usize] | x == x {
                stk.pop();
            }
            left[i] = *stk.last().unwrap();
            stk.push(i as i32);
        }

        let (mut stk, mut ans) = (vec![n], 0_i64);

        for (i, &x) in nums.iter().enumerate().rev() {
            while stk.len() > 1
                && nums[stk[stk.len() - 1]] != x
                && nums[stk[stk.len() - 1]] | x == x
            {
                stk.pop();
            }
            let right = *stk.last().unwrap() as i64;
            stk.push(i);
            ans += (i as i64 - left[i] as i64) * (right - i as i64);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        // [4],[2],[3],[2,3] are good → 4
        assert_eq!(Solution::count_good_subarrays(vec![4, 2, 3]), 4);
    }

    #[test]
    fn test_example_two() {
        // All 6 subarrays are good.
        assert_eq!(Solution::count_good_subarrays(vec![1, 3, 1]), 6);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::count_good_subarrays(vec![5]), 1);
    }

    #[test]
    fn test_all_same() {
        // All subarrays of [2,2,2] have OR = 2 which is present → n*(n+1)/2 = 6.
        assert_eq!(Solution::count_good_subarrays(vec![2, 2, 2]), 6);
    }

    #[test]
    fn test_powers_of_two() {
        // [1,2,4]: OR of any multi-element subarray introduces new bits not matching
        // any single element. Only singletons are good → 3.
        assert_eq!(Solution::count_good_subarrays(vec![1, 2, 4]), 3);
    }

    #[test]
    fn test_zero_elements() {
        // 0 | anything = anything. [0,0] → OR=0, present. [0,3] → OR=3, present.
        // [0,3,0] → OR=3, present. All 6 subarrays are good.
        assert_eq!(Solution::count_good_subarrays(vec![0, 3, 0]), 6);
    }

    #[test]
    fn test_or_matches_later() {
        // [1,3]: OR=3, 3 is present → good.
        // [3,1]: OR=3, 3 is present → good.
        // Singletons: [1],[3],[1] → 3.
        // [1,3] → good, [3,1] → good, [1,3,1] → OR=3, present → good.
        // Total = 3 + 3 = 6.
        assert_eq!(Solution::count_good_subarrays(vec![1, 3, 1]), 6);
    }

    #[test]
    fn test_two_elements_no_match() {
        // [1,2]: OR=3, neither 1 nor 2 is 3 → not good. Only singletons → 2.
        assert_eq!(Solution::count_good_subarrays(vec![1, 2]), 2);
    }

    #[test]
    fn test_large_values() {
        let v = vec![1_000_000_000, 1_000_000_000];
        // Both singletons good, pair OR = same value, present → 3.
        assert_eq!(Solution::count_good_subarrays(v), 3);
    }
}
