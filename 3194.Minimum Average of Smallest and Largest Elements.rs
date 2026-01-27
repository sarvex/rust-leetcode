impl Solution {
    /// Minimum average of paired smallest and largest elements.
    ///
    /// # Intuition
    /// After sorting, pair the smallest with the largest, second smallest with
    /// second largest, etc. The minimum sum of any such pair divided by two
    /// gives the answer.
    ///
    /// # Approach
    /// 1. Sort the array.
    /// 2. Pair elements from both ends inward.
    /// 3. Find the minimum pair sum and divide by 2.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(1) auxiliary
    pub fn minimum_average(mut nums: Vec<i32>) -> f64 {
        nums.sort_unstable();
        let n = nums.len();
        (0..n / 2).map(|i| nums[i] + nums[n - 1 - i]).min().unwrap() as f64 / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn even_length_array() {
        assert!((Solution::minimum_average(vec![7, 8, 3, 4, 15, 13, 4, 1]) - 5.5).abs() < 1e-9);
    }

    #[test]
    fn two_elements() {
        assert!((Solution::minimum_average(vec![1, 9]) - 5.0).abs() < 1e-9);
    }
}
