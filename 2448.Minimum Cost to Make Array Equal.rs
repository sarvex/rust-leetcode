impl Solution {
    /// Calculates minimum cost to make all elements equal using weighted median.
    ///
    /// # Intuition
    /// The optimal target value to minimize total weighted distance is the weighted median.
    /// Moving all elements to this point minimizes the sum of `|nums[i] - target| * cost[i]`.
    ///
    /// # Approach
    /// 1. Pair each number with its cost and sort by value
    /// 2. Find the weighted median by accumulating costs until reaching half the total
    /// 3. Calculate total cost to move all elements to the weighted median
    ///
    /// # Complexity
    /// - Time: O(n log n) for sorting
    /// - Space: O(n) for the paired vector
    pub fn min_cost(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
        let target = Self::find_weighted_median(&nums, &cost);
        Self::calculate_total_cost(&nums, &cost, target)
    }

    fn find_weighted_median(nums: &[i32], cost: &[i32]) -> i64 {
        let mut pairs: Vec<(i32, i32)> = nums.iter().copied().zip(cost.iter().copied()).collect();
        pairs.sort_unstable_by_key(|&(num, _)| num);

        let total_weight: i64 = cost.iter().map(|&c| i64::from(c)).sum();
        let median_threshold = (total_weight + 1) / 2;

        let mut accumulated = 0i64;
        for &(num, weight) in &pairs {
            accumulated += i64::from(weight);
            if accumulated >= median_threshold {
                return i64::from(num);
            }
        }

        i64::from(pairs.last().map_or(0, |&(num, _)| num))
    }

    fn calculate_total_cost(nums: &[i32], cost: &[i32], target: i64) -> i64 {
        nums.iter()
            .zip(cost.iter())
            .map(|(&num, &c)| (i64::from(num) - target).abs() * i64::from(c))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::min_cost(vec![1, 3, 5, 2], vec![2, 3, 1, 14]), 8);
    }

    #[test]
    fn test_all_equal() {
        assert_eq!(
            Solution::min_cost(vec![2, 2, 2, 2, 2], vec![4, 2, 8, 1, 3]),
            0
        );
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::min_cost(vec![5], vec![10]), 0);
    }

    #[test]
    fn test_two_elements() {
        assert_eq!(Solution::min_cost(vec![1, 10], vec![5, 2]), 18);
    }

    #[test]
    fn test_equal_costs() {
        assert_eq!(Solution::min_cost(vec![1, 2, 3], vec![1, 1, 1]), 2);
    }
}
