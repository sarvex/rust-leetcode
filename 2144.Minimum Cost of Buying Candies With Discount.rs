impl Solution {
    /// Greedy sort: skip every third candy (the cheapest in each group of three).
    ///
    /// # Intuition
    /// To minimize total cost, we want the free candies to be as expensive as possible.
    /// Sorting in descending order and skipping every third element (index 2, 5, 8, …)
    /// ensures the most valuable candy is always the free one in each group.
    ///
    /// # Approach
    /// 1. Sort `cost` in descending order.
    /// 2. Sum all elements except those at indices 2, 5, 8, … (i.e., `i % 3 == 2`).
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(1)
    pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
        cost.sort_unstable_by(|a, b| b.cmp(a));
        cost.iter()
            .enumerate()
            .filter(|(i, _)| i % 3 != 2)
            .map(|(_, &c)| c)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::minimum_cost(vec![1, 2, 3]), 5);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::minimum_cost(vec![6, 5, 7, 9, 2, 2]), 23);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::minimum_cost(vec![5, 5]), 10);
    }

    #[test]
    fn test_single_candy() {
        assert_eq!(Solution::minimum_cost(vec![7]), 7);
    }

    #[test]
    fn test_all_same_cost() {
        // 9 candies of cost 4: groups of 3 → pay for 6, free 3 → 6 * 4 = 24
        assert_eq!(Solution::minimum_cost(vec![4; 9]), 24);
    }

    #[test]
    fn test_exactly_three() {
        // Buy 10 and 8, get 5 free → 18
        assert_eq!(Solution::minimum_cost(vec![5, 8, 10]), 18);
    }
}
