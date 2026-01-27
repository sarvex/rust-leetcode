impl Solution {
    /// Sums the minimum of every subarray using monotonic stack contribution.
    ///
    /// # Intuition
    /// Each element `arr[i]` is the minimum for a range of subarrays determined
    /// by its nearest smaller elements on both sides. Use monotonic stacks
    /// to find these boundaries.
    ///
    /// # Approach
    /// Compute `left[i]` (previous less element) and `right[i]` (next less-or-equal
    /// element) via monotonic stacks. The contribution of `arr[i]` is
    /// `arr[i] * (i - left[i]) * (right[i] - i)`.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n) for the stacks and boundary arrays
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = arr.len();
        let mut left = vec![-1i32; n];
        let mut right = vec![n as i32; n];
        let mut stack: Vec<usize> = Vec::new();

        for i in 0..n {
            while stack.last().is_some_and(|&top| arr[top] >= arr[i]) {
                stack.pop();
            }
            if let Some(&top) = stack.last() {
                left[i] = top as i32;
            }
            stack.push(i);
        }

        stack.clear();
        for i in (0..n).rev() {
            while stack.last().is_some_and(|&top| arr[top] > arr[i]) {
                stack.pop();
            }
            if let Some(&top) = stack.last() {
                right[i] = top as i32;
            }
            stack.push(i);
        }

        (0..n)
            .map(|i| {
                let l = (i as i32 - left[i]) as i64;
                let r = (right[i] - i as i32) as i64;
                arr[i] as i64 * l % MOD * r % MOD
            })
            .sum::<i64>() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::sum_subarray_mins(vec![3, 1, 2, 4]), 17);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::sum_subarray_mins(vec![11, 81, 94, 43, 3]), 444);
    }

    #[test]
    fn test_single() {
        assert_eq!(Solution::sum_subarray_mins(vec![5]), 5);
    }
}
