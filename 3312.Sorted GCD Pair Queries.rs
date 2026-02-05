impl Solution {
    /// Count gcd-pair frequencies with a divisor sieve and answer by prefix search.
    ///
    /// # Intuition
    /// For any divisor `d`, all pairs whose elements are both divisible by `d` contribute
    /// to gcds that are multiples of `d`. If we can count how many numbers are divisible
    /// by each `d`, we can use inclusion-exclusion to isolate the pairs whose gcd is
    /// exactly `d`.
    ///
    /// # Approach
    /// - Build a frequency table of `nums` and compute `divisible[d]`, the count of
    ///   elements divisible by `d`, using a sieve over multiples.
    /// - The number of pairs with gcd exactly `d` is:
    ///   `pairs[d] = C(divisible[d], 2) - sum_{k=2d,3d,...} pairs[k]`.
    /// - Build a prefix sum over `pairs` in increasing `d`; this is the sorted
    ///   `gcdPairs` index space.
    /// - For each query `q`, binary search the smallest `d` with `prefix[d] > q`.
    ///
    /// # Complexity
    /// - Time: O(M log M + Q log M), where M = max(nums)
    /// - Space: O(M)
    pub fn gcd_values(nums: Vec<i32>, queries: Vec<i64>) -> Vec<i32> {
        #[inline]
        fn first_greater(prefix: &[i64], target: i64) -> i32 {
            let mut left = 1_usize;
            let mut right = prefix.len() - 1;
            while left < right {
                let mid = left + (right - left) / 2;
                if prefix[mid] > target {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            left as i32
        }

        let max_value = nums.iter().copied().max().unwrap_or(0) as usize;
        let mut freq = vec![0_i64; max_value + 1];
        nums.into_iter().for_each(|num| freq[num as usize] += 1);

        let mut divisible = vec![0_i64; max_value + 1];
        (1..=max_value).for_each(|d| {
            divisible[d] = (d..=max_value)
                .step_by(d)
                .map(|multiple| freq[multiple])
                .sum();
        });

        let mut exact_pairs = vec![0_i64; max_value + 1];
        (1..=max_value).rev().for_each(|d| {
            let count = divisible[d];
            let pairs = count * (count - 1) / 2;
            let subtract: i64 = ((d + d)..=max_value)
                .step_by(d)
                .map(|multiple| exact_pairs[multiple])
                .sum();
            exact_pairs[d] = pairs - subtract;
        });

        let mut prefix = vec![0_i64; max_value + 1];
        (1..=max_value).fold(0_i64, |running, d| {
            let new_running = running + exact_pairs[d];
            prefix[d] = new_running;
            new_running
        });

        queries
            .into_iter()
            .map(|q| first_greater(&prefix, q))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        assert_eq!(
            Solution::gcd_values(vec![2, 3, 4], vec![0, 2, 2]),
            vec![1, 2, 2]
        );
    }

    #[test]
    fn example_two() {
        assert_eq!(
            Solution::gcd_values(vec![4, 4, 2, 1], vec![5, 3, 1, 0]),
            vec![4, 2, 1, 1]
        );
    }

    #[test]
    fn example_three() {
        assert_eq!(Solution::gcd_values(vec![2, 2], vec![0, 0]), vec![2, 2]);
    }

    #[test]
    fn all_same_values() {
        assert_eq!(
            Solution::gcd_values(vec![6, 6, 6], vec![0, 1, 2]),
            vec![6, 6, 6]
        );
    }
}
