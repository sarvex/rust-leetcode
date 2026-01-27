impl Solution {
    /// Two-pointer approach on sorted array for k-sum pairs.
    ///
    /// # Intuition
    /// Sorting enables a two-pointer scan: if the sum of endpoints equals k,
    /// count and shrink both pointers; if too large, shrink right; if too
    /// small, expand left.
    ///
    /// # Approach
    /// 1. Sort the array
    /// 2. Two pointers from both ends
    /// 3. Match on the sum comparison to k
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(1) in-place sort
    pub fn max_operations(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let (mut l, mut r, mut count) = (0, nums.len() - 1, 0);

        while l < r {
            match (nums[l] + nums[r]).cmp(&k) {
                std::cmp::Ordering::Equal => {
                    count += 1;
                    l += 1;
                    r -= 1;
                }
                std::cmp::Ordering::Greater => r -= 1,
                std::cmp::Ordering::Less => l += 1,
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_pairs() {
        assert_eq!(Solution::max_operations(vec![1, 2, 3, 4], 5), 2);
    }

    #[test]
    fn one_pair() {
        assert_eq!(Solution::max_operations(vec![3, 1, 3, 4, 3], 6), 1);
    }

    #[test]
    fn no_pairs() {
        assert_eq!(Solution::max_operations(vec![1, 1, 1], 10), 0);
    }
}
