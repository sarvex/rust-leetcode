impl Solution {
    /// Find the kth smallest palindrome of a given length for each query.
    ///
    /// # Intuition
    /// Palindromes are determined by their first half. The kth palindrome has
    /// its first half equal to 10^(half-1) + k - 1, mirrored to form the full number.
    ///
    /// # Approach
    /// 1. Compute the base value for the first half.
    /// 2. For each query, form the first half and mirror it (skipping the middle
    ///    digit for odd lengths).
    /// 3. Return -1 if the query exceeds available palindromes.
    ///
    /// # Complexity
    /// - Time: O(q * L) where q is query count and L is int_length
    /// - Space: O(q)
    pub fn kth_palindrome(queries: Vec<i32>, int_length: i32) -> Vec<i64> {
        let is_odd = int_length % 2 == 1;
        let half = int_length / 2 + if is_odd { 0 } else { -1 };
        let base = 10i32.pow(half as u32);
        let max_count = base * 9;

        queries
            .iter()
            .map(|&q| {
                if q > max_count {
                    return -1;
                }
                let first_half = base + q - 1;
                let half_str = first_half.to_string();
                let mirror: String = half_str
                    .chars()
                    .rev()
                    .skip(if is_odd { 1 } else { 0 })
                    .collect();
                format!("{half_str}{mirror}").parse::<i64>().unwrap()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odd_length() {
        assert_eq!(
            Solution::kth_palindrome(vec![1, 2, 3, 4, 5, 90], 3),
            vec![101, 111, 121, 131, 141, 999]
        );
    }

    #[test]
    fn even_length() {
        assert_eq!(Solution::kth_palindrome(vec![1, 2], 2), vec![11, 22]);
    }

    #[test]
    fn out_of_range() {
        assert_eq!(Solution::kth_palindrome(vec![100], 2), vec![-1]);
    }
}
