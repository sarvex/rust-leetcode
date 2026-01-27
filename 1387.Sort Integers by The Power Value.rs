impl Solution {
    /// Collatz sequence power value sorting with selection.
    ///
    /// # Intuition
    /// The power value is the number of Collatz steps to reach 1. Computing
    /// this for each number in the range and sorting by power (with value as
    /// tiebreaker) identifies the k-th element.
    ///
    /// # Approach
    /// 1. Define a closure to compute Collatz step count
    /// 2. Collect all integers in `[lo, hi]`
    /// 3. Sort by `(power(x), x)` and return the k-th element
    ///
    /// # Complexity
    /// - Time: O(n · s · log n) where s is average step count, n = hi - lo + 1
    /// - Space: O(n) for the sorted array
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        let power = |mut x: i32| -> i32 {
            let mut steps = 0;
            while x != 1 {
                x = if x % 2 == 0 { x / 2 } else { 3 * x + 1 };
                steps += 1;
            }
            steps
        };

        let mut nums: Vec<i32> = (lo..=hi).collect();
        nums.sort_unstable_by_key(|&x| (power(x), x));
        nums[(k - 1) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_12_15_k2() {
        assert_eq!(Solution::get_kth(12, 15, 2), 13);
    }

    #[test]
    fn range_1_1_k1() {
        assert_eq!(Solution::get_kth(1, 1, 1), 1);
    }

    #[test]
    fn range_7_11_k4() {
        assert_eq!(Solution::get_kth(7, 11, 4), 7);
    }
}
