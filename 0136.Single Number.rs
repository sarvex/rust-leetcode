impl Solution {
    /// Finds the element appearing once using XOR reduction.
    ///
    /// # Intuition
    /// XOR of a number with itself is zero, and XOR with zero is the number itself.
    /// XORing all elements cancels out duplicates, leaving the single number.
    ///
    /// # Approach
    /// Reduce the entire array with XOR.
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(1)
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.into_iter().fold(0, |acc, x| acc ^ x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_among_pairs() {
        assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
    }

    #[test]
    fn larger_input() {
        assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::single_number(vec![1]), 1);
    }
}
