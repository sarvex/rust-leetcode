use std::collections::HashMap;

impl Solution {
    /// Counts tuples (a,b,c,d) where a*b == c*d using product frequency map.
    ///
    /// # Intuition
    /// Each pair of indices produces a product. If k pairs share the same product,
    /// there are C(k,2) ways to pick two pairs, and each pair of pairs yields 8
    /// valid tuples (from reordering within and across pairs).
    ///
    /// # Approach
    /// 1. Enumerate all pairs and count product frequencies in a hash map.
    /// 2. For each product with count v, add v*(v-1)/2 combinations.
    /// 3. Multiply the total by 8 (each combination yields 8 tuples).
    ///
    /// # Complexity
    /// - Time: O(n²)
    /// - Space: O(n²)
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut cnt: HashMap<i32, i32> = HashMap::new();
        for i in 1..nums.len() {
            for j in 0..i {
                *cnt.entry(nums[i] * nums[j]).or_insert(0) += 1;
            }
        }
        cnt.values().map(|v| v * (v - 1) / 2).sum::<i32>() << 3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_one() {
        assert_eq!(Solution::tuple_same_product(vec![2, 3, 4, 6]), 8);
    }

    #[test]
    fn test_example_two() {
        assert_eq!(Solution::tuple_same_product(vec![1, 2, 4, 5, 10]), 16);
    }

    #[test]
    fn test_no_tuples() {
        assert_eq!(Solution::tuple_same_product(vec![1, 2, 3]), 0);
    }
}
