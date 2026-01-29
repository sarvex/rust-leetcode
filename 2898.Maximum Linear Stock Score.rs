use std::collections::HashMap;

impl Solution {
    /// Finds the maximum sum of elements forming a linear stock score.
    ///
    /// # Intuition
    /// Elements at indices i and j lie on the same line with slope 1 when
    /// `prices[i] - i == prices[j] - j`. Grouping by this key and summing
    /// each group yields the answer.
    ///
    /// # Approach
    /// 1. Iterate with `enumerate`, computing the key `price - index` for each element.
    /// 2. Accumulate sums in a `HashMap` keyed by the difference.
    /// 3. Return the maximum accumulated sum.
    ///
    /// # Complexity
    /// - Time: O(n) where n is the length of prices
    /// - Space: O(n) for the hash map
    pub fn max_score(prices: Vec<i32>) -> i64 {
        let mut groups: HashMap<i32, i64> = HashMap::new();
        prices.iter().enumerate().for_each(|(i, x)| {
            *groups.entry(x - i as i32).or_insert(0) += x as i64;
        });
        groups.values().copied().max().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_sequence() {
        assert_eq!(Solution::max_score(vec![1, 5, 3, 7, 8]), 20);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::max_score(vec![5]), 5);
    }
}
