impl Solution {
    /// Trimmed mean after removing top and bottom 5%.
    ///
    /// # Intuition
    /// Sort the array, discard the smallest and largest 5% of elements,
    /// then compute the mean of the remaining 90%.
    ///
    /// # Approach
    /// 1. Sort the array
    /// 2. Skip the first and last `n * 0.05` elements
    /// 3. Average the middle 90%
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(1) in-place sort
    pub fn trim_mean(mut arr: Vec<i32>) -> f64 {
        arr.sort_unstable();
        let n = arr.len();
        let trim = n / 20;
        let sum: i32 = arr[trim..n - trim].iter().sum();
        sum as f64 / (n - 2 * trim) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_trim() {
        let arr = vec![1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3];
        let result = Solution::trim_mean(arr);
        assert!((result - 2.0).abs() < 1e-5);
    }

    #[test]
    fn uniform_array() {
        let arr = vec![6; 20];
        assert!((Solution::trim_mean(arr) - 6.0).abs() < 1e-5);
    }
}
