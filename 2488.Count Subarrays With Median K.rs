impl Solution {
    /// Counts subarrays where k is the median using balance transformation
    ///
    /// # Intuition
    /// For k to be the median, the count of elements greater than k must equal
    /// the count of elements less than k (odd length) or exceed by 1 (even length,
    /// left middle rule).
    ///
    /// # Approach
    /// Transform the problem into a balance problem:
    /// - Assign -1 for elements < k, +1 for elements > k, 0 for k
    /// - Valid subarrays containing k have total balance of 0 or 1
    /// - Use prefix sums on left side stored in hashmap, then iterate right side
    ///   finding complementary balances
    ///
    /// # Complexity
    /// - Time: O(n) - single pass left and right from k's position
    /// - Space: O(n) - hashmap storing at most n prefix sums
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k_pos = nums.iter().position(|&x| x == k).unwrap();

        let mut left_count = std::collections::HashMap::with_capacity(k_pos + 1);
        left_count.insert(0, 1);

        let mut balance = 0i32;
        for i in (0..k_pos).rev() {
            balance += if nums[i] < k { -1 } else { 1 };
            *left_count.entry(balance).or_insert(0) += 1;
        }

        let mut result = 0;
        balance = 0;

        for i in k_pos..n {
            if nums[i] < k {
                balance -= 1;
            } else if nums[i] > k {
                balance += 1;
            }
            result += left_count.get(&(-balance)).unwrap_or(&0);
            result += left_count.get(&(1 - balance)).unwrap_or(&0);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![3, 2, 1, 4, 5];
        let k = 4;
        assert_eq!(Solution::count_subarrays(nums, k), 3);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![2, 3, 1];
        let k = 3;
        assert_eq!(Solution::count_subarrays(nums, k), 1);
    }

    #[test]
    fn test_single_element() {
        let nums = vec![1];
        let k = 1;
        assert_eq!(Solution::count_subarrays(nums, k), 1);
    }

    #[test]
    fn test_k_at_start() {
        // Valid: [1], [1,2] (sorted [1,2], left middle is 1)
        let nums = vec![1, 2, 3];
        let k = 1;
        assert_eq!(Solution::count_subarrays(nums, k), 2);
    }

    #[test]
    fn test_k_at_end() {
        let nums = vec![1, 2, 3];
        let k = 3;
        assert_eq!(Solution::count_subarrays(nums, k), 1);
    }

    #[test]
    fn test_k_in_middle() {
        let nums = vec![1, 2, 3];
        let k = 2;
        assert_eq!(Solution::count_subarrays(nums, k), 3);
    }

    #[test]
    fn test_larger_array() {
        // Valid: [3], [4,3], [1,4,3], [5,2,1,4,3]
        let nums = vec![5, 2, 1, 4, 3];
        let k = 3;
        assert_eq!(Solution::count_subarrays(nums, k), 4);
    }
}
