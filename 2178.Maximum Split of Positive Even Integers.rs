impl Solution {
    /// Split an even number into the maximum count of unique positive even integers.
    ///
    /// # Intuition
    /// Greedily pick the smallest even numbers (2, 4, 6, ...) and add any
    /// remainder to the last picked number to maintain uniqueness.
    ///
    /// # Approach
    /// 1. Return empty if the number is odd.
    /// 2. Greedily subtract 2, 4, 6, ... from the remaining sum.
    /// 3. Add leftover remainder to the last element.
    ///
    /// # Complexity
    /// - Time: O(√n)
    /// - Space: O(√n) for the result
    pub fn maximum_even_split(mut final_sum: i64) -> Vec<i64> {
        if final_sum % 2 != 0 {
            return Vec::new();
        }

        let mut result = Vec::new();
        let mut even = 2i64;
        while even <= final_sum {
            result.push(even);
            final_sum -= even;
            even += 2;
        }

        if let Some(last) = result.last_mut() {
            *last += final_sum;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn even_input() {
        let result = Solution::maximum_even_split(12);
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    fn odd_input() {
        assert!(Solution::maximum_even_split(7).is_empty());
    }

    #[test]
    fn small_even() {
        assert_eq!(Solution::maximum_even_split(2), vec![2]);
    }
}
