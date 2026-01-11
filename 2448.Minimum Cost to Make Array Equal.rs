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
impl Solution {
    pub fn min_cost(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
        let target = Self::find_weighted_median(&nums, &cost);
        Self::calculate_total_cost(&nums, &cost, target)
    }

    /// Finds the weighted median value from nums using costs as weights.
    fn find_weighted_median(nums: &[i32], cost: &[i32]) -> i64 {
        let mut pairs: Vec<(i32, i32)> = nums.iter().copied().zip(cost.iter().copied()).collect();

        pairs.sort_unstable_by_key(|&(num, _)| num);

        let total_weight: i64 = cost.iter().map(|&c| i64::from(c)).sum();
        let median_threshold = (total_weight + 1) / 2;

        let mut accumulated_weight = 0_i64;
        for &(num, weight) in &pairs {
            accumulated_weight += i64::from(weight);
            if accumulated_weight >= median_threshold {
                return i64::from(num);
            }
        }

        i64::from(pairs.last().map_or(0, |&(num, _)| num))
    }

    /// Calculates total weighted Manhattan distance from all elements to target.
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

    struct TestCase {
        nums: Vec<i32>,
        cost: Vec<i32>,
        expected: i64,
    }

    #[test]
    fn test_basic_cases() {
        let test_cases = [
            TestCase {
                nums: vec![1, 3, 5, 2],
                cost: vec![2, 3, 1, 14],
                expected: 8,
            },
            TestCase {
                nums: vec![2, 2, 2, 2, 2],
                cost: vec![4, 2, 8, 1, 3],
                expected: 0,
            },
        ];

        for (i, tc) in test_cases.iter().enumerate() {
            let result = Solution::min_cost(tc.nums.clone(), tc.cost.clone());
            assert_eq!(result, tc.expected, "Test case {} failed", i + 1);
        }
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::min_cost(vec![5], vec![10]), 0);
    }

    #[test]
    fn test_two_elements() {
        let result = Solution::min_cost(vec![1, 10], vec![5, 2]);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_equal_costs() {
        let result = Solution::min_cost(vec![1, 2, 3], vec![1, 1, 1]);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_weighted_median_selection() {
        let nums = vec![1, 3, 5, 2];
        let cost = vec![2, 3, 1, 14];
        let target = Solution::find_weighted_median(&nums, &cost);
        assert_eq!(target, 2);
    }
}
