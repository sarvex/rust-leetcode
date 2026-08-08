struct Solution;

impl Solution {
    /// Compute sum of GCDs of pairs formed from a sorted prefixGcd array.
    ///
    /// # Intuition
    /// For each index i, `prefixGcd[i] = gcd(nums[i], max(nums[0..=i]))`. After
    /// sorting this array, pairing the smallest with the largest element and
    /// accumulating their GCDs gives the answer.
    ///
    /// # Approach
    /// 1. Single pass over `nums`: maintain `running_max`; push
    ///    `gcd(nums[i], running_max)` using an iterative GCD — O(n log V).
    /// 2. Sort `prefixGcd` — O(n log n).
    /// 3. Two-pointer scan summing `gcd(prefix[left], prefix[right])` for each of
    ///    the ⌊n/2⌋ pairs — O(n log V).
    ///
    /// # Complexity
    /// - Time: O(n log n) — sorting dominates.
    /// - Space: O(n) — for the `prefixGcd` array.
    pub fn gcd_sum(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut prefix_gcd = Vec::with_capacity(n);

        let mut running_max = 0i32;
        for &x in &nums {
            running_max = running_max.max(x);
            prefix_gcd.push(Self::gcd(x, running_max));
        }

        prefix_gcd.sort_unstable();

        let (mut left, mut right) = (0usize, n - 1);
        let mut total = 0i64;
        while left < right {
            total += Self::gcd(prefix_gcd[left], prefix_gcd[right]) as i64;
            left += 1;
            right -= 1;
        }
        total
    }

    /// Iterative Euclidean GCD — avoids recursive call overhead.
    #[inline]
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            a %= b;
            std::mem::swap(&mut a, &mut b);
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // prefixGcd = [2, 6, 2] → sorted [2, 2, 6]
        // pair: gcd(2, 6) = 2; middle 2 ignored → sum = 2
        assert_eq!(Solution::gcd_sum(vec![2, 6, 4]), 2);
    }

    #[test]
    fn test_example_2() {
        // prefixGcd = [3, 6, 2, 8] → sorted [2, 3, 6, 8]
        // pairs: gcd(2,8)=2, gcd(3,6)=3 → sum = 5
        assert_eq!(Solution::gcd_sum(vec![3, 6, 2, 8]), 5);
    }

    #[test]
    fn test_single_element() {
        // n=1, no pairs formed → sum = 0
        assert_eq!(Solution::gcd_sum(vec![7]), 0);
    }

    #[test]
    fn test_two_elements() {
        // prefixGcd = [5, 5] → sorted [5, 5]; pair: gcd(5,5) = 5
        assert_eq!(Solution::gcd_sum(vec![5, 5]), 5);
    }

    #[test]
    fn test_all_same() {
        // nums = [4,4,4,4] → prefixGcd = [4,4,4,4]; pairs: gcd(4,4)+gcd(4,4) = 8
        assert_eq!(Solution::gcd_sum(vec![4, 4, 4, 4]), 8);
    }

    #[test]
    fn test_strictly_increasing() {
        // nums = [1,2,3,4] → running_max = [1,2,3,4]
        // prefixGcd = [gcd(1,1), gcd(2,2), gcd(3,3), gcd(4,4)] = [1,2,3,4]
        // sorted [1,2,3,4]; pairs: gcd(1,4)=1, gcd(2,3)=1 → sum = 2
        assert_eq!(Solution::gcd_sum(vec![1, 2, 3, 4]), 2);
    }

    #[test]
    fn test_large_values() {
        // nums = [10^9, 10^9] → prefixGcd = [10^9, 10^9]; pair gcd = 10^9
        assert_eq!(
            Solution::gcd_sum(vec![1_000_000_000, 1_000_000_000]),
            1_000_000_000
        );
    }
}
