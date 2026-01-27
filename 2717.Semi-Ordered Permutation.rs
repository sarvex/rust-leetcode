impl Solution {
    /// Minimum swaps to place 1 first and n last in a semi-ordered permutation.
    ///
    /// # Intuition
    /// Moving 1 to the front costs its index, moving n to the end costs `n - 1 - j`.
    /// If 1 appears after n, their paths cross and we save one swap.
    ///
    /// # Approach
    /// 1. Find positions of 1 and n using iterator search.
    /// 2. Compute total swaps as `pos_one + (n - 1 - pos_n)`.
    /// 3. Subtract 1 if 1 is initially to the right of n (paths overlap).
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let pos_one = nums.iter().position(|&x| x == 1).unwrap();
        let pos_n = nums.iter().position(|&x| x == n as i32).unwrap();
        let overlap = match pos_one > pos_n {
            true => 1,
            false => 0,
        };
        (pos_one + n - 1 - pos_n - overlap) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_overlap_needed() {
        assert_eq!(Solution::semi_ordered_permutation(vec![2, 1, 4, 3]), 2);
    }

    #[test]
    fn already_semi_ordered() {
        assert_eq!(Solution::semi_ordered_permutation(vec![1, 3, 4, 2, 5]), 0);
    }

    #[test]
    fn overlap_saves_one_swap() {
        assert_eq!(Solution::semi_ordered_permutation(vec![2, 4, 1, 3]), 3);
    }
}
