impl Solution {
    /// Computes the sum of even numbers after each query.
    ///
    /// # Intuition
    /// Maintain a running sum of even numbers. For each query, adjust the
    /// sum based on whether the affected element was/becomes even.
    ///
    /// # Approach
    /// Precompute even sum. For each query, subtract the element if it was
    /// even, apply the update, then add it back if it is now even.
    ///
    /// # Complexity
    /// - Time: O(n + q) where q is query count
    /// - Space: O(q) for the result
    pub fn sum_even_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut even_sum: i32 = nums.iter().filter(|&&x| x % 2 == 0).sum();
        let mut result = Vec::with_capacity(queries.len());

        for query in &queries {
            let (val, idx) = (query[0], query[1] as usize);
            if nums[idx] % 2 == 0 {
                even_sum -= nums[idx];
            }
            nums[idx] += val;
            if nums[idx] % 2 == 0 {
                even_sum += nums[idx];
            }
            result.push(even_sum);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(
            Solution::sum_even_after_queries(
                vec![1, 2, 3, 4],
                vec![vec![1, 0], vec![-3, 1], vec![-4, 0], vec![2, 3]],
            ),
            vec![8, 6, 2, 4]
        );
    }

    #[test]
    fn test_single() {
        assert_eq!(
            Solution::sum_even_after_queries(vec![1], vec![vec![4, 0]]),
            vec![0]
        );
    }
}
