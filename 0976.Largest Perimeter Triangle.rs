impl Solution {
    /// Finds the largest perimeter triangle from a sorted array.
    ///
    /// # Intuition
    /// Sort descending. The first triple satisfying the triangle inequality
    /// `a < b + c` yields the largest perimeter.
    ///
    /// # Approach
    /// Sort in descending order. Check consecutive triples for validity.
    ///
    /// # Complexity
    /// - Time: O(n log n)
    /// - Space: O(log n) for sorting
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable_by(|a, b| b.cmp(a));
        nums.windows(3)
            .find(|w| w[0] < w[1] + w[2])
            .map(|w| w[0] + w[1] + w[2])
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::largest_perimeter(vec![2, 1, 2]), 5);
    }

    #[test]
    fn test_no_triangle() {
        assert_eq!(Solution::largest_perimeter(vec![1, 2, 1]), 0);
    }

    #[test]
    fn test_larger() {
        assert_eq!(Solution::largest_perimeter(vec![3, 6, 2, 3]), 8);
    }
}
